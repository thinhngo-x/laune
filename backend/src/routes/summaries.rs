use crate::{
    config,
    db::DbPool,
    error::AppError,
    models::{
        ArticleSummaryInfo, FeedAggregationRequest, FeedAggregationResponse, FeedSummaryInfo,
        Summary,
    },
    summarizer,
};
use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use chrono::{Duration, Utc};
use sqlx::Row;
use tracing::{error, info};
use uuid::Uuid;

pub fn router() -> Router<DbPool> {
    Router::new()
        .route(
            "/articles/:article_id/summary",
            get(get_summary).post(create_summary),
        )
        .route("/feeds/aggregate-summary", post(create_aggregated_summary))
}

async fn get_summary(
    State(pool): State<DbPool>,
    Path(article_id): Path<Uuid>,
) -> Result<Json<Option<Summary>>, AppError> {
    // Check if article exists
    let article_exists = sqlx::query("SELECT id FROM articles WHERE id = $1")
        .bind(article_id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .is_some();

    if !article_exists {
        return Err(AppError::NotFound(format!(
            "Article with ID {} not found",
            article_id
        )));
    }

    // Get summary for article
    let summary = sqlx::query(
        r#"
        SELECT * FROM summaries
        WHERE article_id = $1
        ORDER BY created_at DESC
        LIMIT 1
        "#,
    )
    .bind(article_id)
    .map(|row: sqlx::postgres::PgRow| Summary {
        id: row.get("id"),
        article_id: row.get("article_id"),
        content: row.get("content"),
        model: row.get("model"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    })
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        error!(
            "Failed to fetch summary for article {}: {:?}",
            article_id, e
        );
        AppError::DatabaseError(e.to_string())
    })?;

    Ok(Json(summary))
}

async fn create_summary(
    State(pool): State<DbPool>,
    Path(article_id): Path<Uuid>,
) -> Result<Json<Summary>, AppError> {
    // Load configuration to get the model name
    let settings = match config::Settings::new() {
        Ok(settings) => settings,
        Err(e) => {
            error!("Failed to load config: {:?}", e);
            return Err(AppError::InternalServerError(format!(
                "Configuration error: {}",
                e
            )));
        }
    };

    // Check if article exists and get its content
    let article = sqlx::query("SELECT id, title, content FROM articles WHERE id = $1")
        .bind(article_id)
        .map(|row: sqlx::postgres::PgRow| {
            (
                row.get::<Uuid, _>("id"),
                row.get::<String, _>("title"),
                row.get::<String, _>("content"),
            )
        })
        .fetch_optional(&pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .ok_or_else(|| AppError::NotFound(format!("Article with ID {} not found", article_id)))?;

    // Check if a summary already exists
    let existing_summary = sqlx::query("SELECT id FROM summaries WHERE article_id = $1")
        .bind(article_id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    if existing_summary.is_some() {
        return Err(AppError::BadRequest(format!(
            "Summary for article {} already exists",
            article_id
        )));
    }

    // Generate summary using AI
    let (_, title, content) = article;
    let summary_content = match summarizer::generate_summary(&title, &content).await {
        Ok(content) => content,
        Err(e) => {
            error!(
                "Failed to generate summary for article {}: {:?}",
                article_id, e
            );
            return Err(AppError::SummarizationError(e.to_string()));
        }
    };

    // Save summary to database
    let summary = sqlx::query(
        r#"
        INSERT INTO summaries (article_id, content, model)
        VALUES ($1, $2, $3)
        RETURNING *
        "#,
    )
    .bind(article_id)
    .bind(&summary_content)
    .bind(&settings.openai.model)
    .map(|row: sqlx::postgres::PgRow| Summary {
        id: row.get("id"),
        article_id: row.get("article_id"),
        content: row.get("content"),
        model: row.get("model"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    })
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        error!("Failed to save summary for article {}: {:?}", article_id, e);
        AppError::DatabaseError(e.to_string())
    })?;

    info!("Created summary for article: {}", article_id);
    Ok(Json(summary))
}

async fn create_aggregated_summary(
    State(pool): State<DbPool>,
    Json(request): Json<FeedAggregationRequest>,
) -> Result<Json<FeedAggregationResponse>, AppError> {
    let hours_back = request.hours_back.unwrap_or(24);

    if request.feed_ids.is_empty() {
        return Err(AppError::BadRequest(
            "At least one feed ID must be provided".to_string(),
        ));
    }

    if hours_back <= 0 || hours_back > 168 {
        // Max 1 week
        return Err(AppError::BadRequest(
            "Hours back must be between 1 and 168 (1 week)".to_string(),
        ));
    }

    info!(
        "Creating aggregated summary for {} feeds, {} hours back",
        request.feed_ids.len(),
        hours_back
    );

    // Calculate the cutoff time
    let cutoff_time = Utc::now() - Duration::hours(hours_back);

    let mut feed_summaries = Vec::new();
    let mut total_articles = 0i64;

    // Process each feed
    for feed_id in &request.feed_ids {
        // Verify feed exists, is active, and get its title
        let feed_info = sqlx::query("SELECT id, title, active FROM feeds WHERE id = $1")
            .bind(feed_id)
            .map(|row: sqlx::postgres::PgRow| {
                (
                    row.get::<Uuid, _>("id"),
                    row.get::<String, _>("title"),
                    row.get::<bool, _>("active"),
                )
            })
            .fetch_optional(&pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let (_, feed_title, is_active) = match feed_info {
            Some(info) => info,
            None => {
                error!("Feed with ID {} not found", feed_id);
                continue; // Skip non-existent feeds instead of failing
            }
        };

        // Skip inactive feeds
        if !is_active {
            info!("Skipping inactive feed: {} ({})", feed_title, feed_id);
            continue;
        }

        // Get articles from this feed within the time constraint
        let articles_data = sqlx::query(
            r#"
            SELECT
                a.id, a.title, a.url, a.published_at,
                s.content as summary_content
            FROM articles a
            LEFT JOIN summaries s ON a.id = s.article_id
            WHERE a.feed_id = $1
                AND a.published_at >= $2
            ORDER BY a.published_at DESC
            "#,
        )
        .bind(feed_id)
        .bind(cutoff_time)
        .fetch_all(&pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let mut articles = Vec::new();
        for row in articles_data {
            articles.push(ArticleSummaryInfo {
                id: row.get("id"),
                title: row.get("title"),
                url: row.get("url"),
                published_at: row.get("published_at"),
                summary: row.get("summary_content"),
            });
        }

        let article_count = articles.len() as i64;
        total_articles += article_count;

        if article_count > 0 {
            feed_summaries.push(FeedSummaryInfo {
                feed_id: *feed_id,
                feed_title,
                article_count,
                articles,
            });
        }
    }

    if feed_summaries.is_empty() {
        return Err(AppError::NotFound(
            "No articles found in the specified time range for the selected feeds".to_string(),
        ));
    }

    // Generate aggregated summary
    let aggregated_summary =
        match summarizer::generate_aggregated_summary(&feed_summaries, hours_back).await {
            Ok(summary) => summary,
            Err(e) => {
                error!("Failed to generate aggregated summary: {:?}", e);
                return Err(AppError::SummarizationError(e));
            }
        };

    let response = FeedAggregationResponse {
        summary: aggregated_summary,
        feeds: feed_summaries,
        total_articles,
        time_range_hours: hours_back,
    };

    info!(
        "Successfully created aggregated summary for {} articles from {} feeds",
        total_articles,
        response.feeds.len()
    );
    Ok(Json(response))
}

#[cfg(test)]
mod tests {
    use crate::models::{
        ArticleSummaryInfo, FeedAggregationRequest, FeedSummaryInfo, ToggleFeedStatusRequest,
        ToggleFeedStatusResponse,
    };
    use chrono::Utc;
    use uuid::Uuid;

    #[test]
    fn test_feed_aggregation_request_validation() {
        let request = FeedAggregationRequest {
            feed_ids: vec![Uuid::new_v4(), Uuid::new_v4()],
            hours_back: Some(24),
        };

        assert_eq!(request.feed_ids.len(), 2);
        assert_eq!(request.hours_back, Some(24));
    }

    #[test]
    fn test_feed_summary_info_creation() {
        let feed_id = Uuid::new_v4();
        let article_id = Uuid::new_v4();

        let article = ArticleSummaryInfo {
            id: article_id,
            title: "Test Article".to_string(),
            url: "https://example.com/test".to_string(),
            published_at: Utc::now(),
            summary: Some("Test summary".to_string()),
        };

        let feed_summary = FeedSummaryInfo {
            feed_id,
            feed_title: "Test Feed".to_string(),
            article_count: 1,
            articles: vec![article],
        };

        assert_eq!(feed_summary.feed_id, feed_id);
        assert_eq!(feed_summary.article_count, 1);
        assert_eq!(feed_summary.articles.len(), 1);
        assert_eq!(feed_summary.articles[0].id, article_id);
    }

    #[test]
    fn test_toggle_feed_status_request() {
        let request = ToggleFeedStatusRequest { active: false };

        assert!(!request.active);
    }

    #[test]
    fn test_toggle_feed_status_response() {
        let feed_id = Uuid::new_v4();
        let response = ToggleFeedStatusResponse {
            feed_id,
            active: true,
            message: "Feed successfully activated".to_string(),
        };

        assert_eq!(response.feed_id, feed_id);
        assert!(response.active);
        assert_eq!(response.message, "Feed successfully activated");
    }
}

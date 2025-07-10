use crate::{
    db::DbPool,
    error::AppError,
    models::{Article, BulkFetchRequest, BulkFetchResponse, FeedSummary},
};
use axum::{
    extract::{Path, Query, State},
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use sqlx::Row;
use tracing::{debug, error, info};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct ArticleQuery {
    feed_id: Option<Uuid>,
    limit: Option<i64>,
    offset: Option<i64>,
}

pub fn router() -> Router<DbPool> {
    Router::new()
        .route("/articles", get(list_articles))
        .route("/articles/:id", get(get_article))
        .route("/articles/bulk-fetch", post(bulk_fetch_articles))
        .route("/feeds/:feed_id/articles", get(get_feed_articles))
}

async fn list_articles(
    State(pool): State<DbPool>,
    Query(query): Query<ArticleQuery>,
) -> Result<Json<Vec<Article>>, AppError> {
    let limit = query.limit.unwrap_or(20);
    let offset = query.offset.unwrap_or(0);

    let articles = if let Some(feed_id) = query.feed_id {
        // With feed_id filter
        sqlx::query(
            r#"
            SELECT * FROM articles
            WHERE feed_id = $1
            ORDER BY published_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(feed_id)
        .bind(limit)
        .bind(offset)
        .map(|row: sqlx::postgres::PgRow| Article {
            id: row.get("id"),
            title: row.get("title"),
            url: row.get("url"),
            feed_id: row.get("feed_id"),
            content: row.get("content"),
            published_at: row.get("published_at"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
        .fetch_all(&pool)
        .await
    } else {
        // Without feed_id filter
        sqlx::query(
            r#"
            SELECT * FROM articles
            ORDER BY published_at DESC
            LIMIT $1 OFFSET $2
            "#,
        )
        .bind(limit)
        .bind(offset)
        .map(|row: sqlx::postgres::PgRow| Article {
            id: row.get("id"),
            title: row.get("title"),
            url: row.get("url"),
            feed_id: row.get("feed_id"),
            content: row.get("content"),
            published_at: row.get("published_at"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
        .fetch_all(&pool)
        .await
    }
    .map_err(|e| {
        error!("Failed to fetch articles: {:?}", e);
        AppError::DatabaseError(e.to_string())
    })?;

    debug!("Fetched {} articles", articles.len());
    Ok(Json(articles))
}

async fn get_article(
    State(pool): State<DbPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<Article>, AppError> {
    let article = sqlx::query("SELECT * FROM articles WHERE id = $1")
        .bind(id)
        .map(|row: sqlx::postgres::PgRow| Article {
            id: row.get("id"),
            title: row.get("title"),
            url: row.get("url"),
            feed_id: row.get("feed_id"),
            content: row.get("content"),
            published_at: row.get("published_at"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
        .fetch_optional(&pool)
        .await
        .map_err(|e| {
            error!("Failed to fetch article {}: {:?}", id, e);
            AppError::DatabaseError(e.to_string())
        })?
        .ok_or_else(|| AppError::NotFound(format!("Article with ID {} not found", id)))?;

    Ok(Json(article))
}

async fn get_feed_articles(
    State(pool): State<DbPool>,
    Path(feed_id): Path<Uuid>,
    Query(query): Query<ArticleQuery>,
) -> Result<Json<Vec<Article>>, AppError> {
    // Check if feed exists
    let feed_exists = sqlx::query("SELECT id FROM feeds WHERE id = $1")
        .bind(feed_id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .is_some();

    if !feed_exists {
        return Err(AppError::NotFound(format!(
            "Feed with ID {} not found",
            feed_id
        )));
    }

    let limit = query.limit.unwrap_or(20);
    let offset = query.offset.unwrap_or(0);

    let articles = sqlx::query(
        r#"
        SELECT * FROM articles
        WHERE feed_id = $1
        ORDER BY published_at DESC
        LIMIT $2 OFFSET $3
        "#,
    )
    .bind(feed_id)
    .bind(limit)
    .bind(offset)
    .map(|row: sqlx::postgres::PgRow| Article {
        id: row.get("id"),
        title: row.get("title"),
        url: row.get("url"),
        feed_id: row.get("feed_id"),
        content: row.get("content"),
        published_at: row.get("published_at"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    })
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        error!("Failed to fetch articles for feed {}: {:?}", feed_id, e);
        AppError::DatabaseError(e.to_string())
    })?;

    debug!("Fetched {} articles for feed {}", articles.len(), feed_id);
    Ok(Json(articles))
}

async fn bulk_fetch_articles(
    State(pool): State<DbPool>,
    Json(request): Json<BulkFetchRequest>,
) -> Result<Json<BulkFetchResponse>, AppError> {
    info!("Starting bulk fetch for {} feeds", request.feed_ids.len());

    // First, fetch new articles online for all selected feeds
    if !request.feed_ids.is_empty() {
        let fetcher = crate::feeds::FeedFetcher::new();
        let mut total_new_articles = 0;

        for feed_id in &request.feed_ids {
            match fetcher.refresh_feed(&pool, *feed_id).await {
                Ok(count) => {
                    total_new_articles += count;
                    info!("Fetched {} new articles from feed {}", count, feed_id);
                }
                Err(e) => {
                    error!("Failed to refresh feed {}: {}", feed_id, e);
                    // Continue with other feeds even if one fails
                }
            }
        }

        info!("Total new articles fetched: {}", total_new_articles);
    }

    let limit = request.limit.unwrap_or(100);
    let offset = request.offset.unwrap_or(0);

    // Build the base query
    let mut conditions = vec![];
    let mut bind_index = 1;

    // Add feed filter if feed_ids are provided
    if !request.feed_ids.is_empty() {
        let feed_placeholders: Vec<String> = (bind_index..bind_index + request.feed_ids.len())
            .map(|i| format!("${}", i))
            .collect();
        conditions.push(format!("feed_id IN ({})", feed_placeholders.join(", ")));
        bind_index += request.feed_ids.len();
    }

    // Add date range filters
    if request.start_date.is_some() {
        conditions.push(format!("published_at >= ${}", bind_index));
        bind_index += 1;
    }

    if request.end_date.is_some() {
        conditions.push(format!("published_at <= ${}", bind_index));
        bind_index += 1;
    }

    // Build the complete query
    let where_clause = if !conditions.is_empty() {
        format!(" WHERE {}", conditions.join(" AND "))
    } else {
        String::new()
    };

    let query_str = format!(
        "SELECT * FROM articles{} ORDER BY published_at DESC LIMIT ${} OFFSET ${}",
        where_clause,
        bind_index,
        bind_index + 1
    );

    debug!("Executing bulk fetch query: {}", query_str);

    // Build and execute the query
    let mut query = sqlx::query(&query_str);

    // Bind feed_ids
    for feed_id in &request.feed_ids {
        query = query.bind(feed_id);
    }

    // Bind date filters
    if let Some(start_date) = request.start_date {
        query = query.bind(start_date);
    }

    if let Some(end_date) = request.end_date {
        query = query.bind(end_date);
    }

    // Bind limit and offset
    query = query.bind(limit).bind(offset);

    let articles = query
        .map(|row: sqlx::postgres::PgRow| Article {
            id: row.get("id"),
            title: row.get("title"),
            url: row.get("url"),
            feed_id: row.get("feed_id"),
            content: row.get("content"),
            published_at: row.get("published_at"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
        .fetch_all(&pool)
        .await
        .map_err(|e| {
            error!("Failed to fetch articles in bulk: {:?}", e);
            AppError::DatabaseError(e.to_string())
        })?;

    // Get total count for pagination
    let count_query = format!("SELECT COUNT(*) as total FROM articles{}", where_clause);

    let mut count_sql = sqlx::query(&count_query);

    // Bind parameters for count query (same as main query except limit/offset)
    for feed_id in &request.feed_ids {
        count_sql = count_sql.bind(feed_id);
    }

    if let Some(start_date) = request.start_date {
        count_sql = count_sql.bind(start_date);
    }

    if let Some(end_date) = request.end_date {
        count_sql = count_sql.bind(end_date);
    }

    let total_count: i64 = count_sql
        .fetch_one(&pool)
        .await
        .map_err(|e| {
            error!("Failed to get total count: {:?}", e);
            AppError::DatabaseError(e.to_string())
        })?
        .get("total");

    // Get feed summaries
    let feed_summaries = if !request.feed_ids.is_empty() {
        let mut summary_conditions = vec![];
        let mut summary_bind_index = 1;

        // Add feed filter
        let feed_placeholders: Vec<String> = (summary_bind_index
            ..summary_bind_index + request.feed_ids.len())
            .map(|i| format!("${}", i))
            .collect();
        summary_conditions.push(format!("f.id IN ({})", feed_placeholders.join(", ")));
        summary_bind_index += request.feed_ids.len();

        // Add date filters for articles
        if request.start_date.is_some() {
            summary_conditions.push(format!(
                "(a.published_at >= ${} OR a.id IS NULL)",
                summary_bind_index
            ));
            summary_bind_index += 1;
        }

        if request.end_date.is_some() {
            summary_conditions.push(format!(
                "(a.published_at <= ${} OR a.id IS NULL)",
                summary_bind_index
            ));
        }

        let summary_query = format!(
            "SELECT f.id as feed_id, f.title as feed_title, COUNT(a.id) as article_count
             FROM feeds f
             LEFT JOIN articles a ON f.id = a.feed_id
             WHERE {}
             GROUP BY f.id, f.title
             ORDER BY f.title",
            summary_conditions.join(" AND ")
        );

        let mut summary_sql = sqlx::query(&summary_query);

        // Bind feed_ids
        for feed_id in &request.feed_ids {
            summary_sql = summary_sql.bind(feed_id);
        }

        // Bind date filters
        if let Some(start_date) = request.start_date {
            summary_sql = summary_sql.bind(start_date);
        }

        if let Some(end_date) = request.end_date {
            summary_sql = summary_sql.bind(end_date);
        }

        summary_sql
            .map(|row: sqlx::postgres::PgRow| FeedSummary {
                feed_id: row.get("feed_id"),
                feed_title: row.get("feed_title"),
                article_count: row.get("article_count"),
            })
            .fetch_all(&pool)
            .await
            .map_err(|e| {
                error!("Failed to fetch feed summaries: {:?}", e);
                AppError::DatabaseError(e.to_string())
            })?
    } else {
        vec![]
    };

    debug!(
        "Bulk fetch completed: {} articles, {} total",
        articles.len(),
        total_count
    );

    Ok(Json(BulkFetchResponse {
        articles,
        total_count,
        feed_summaries,
    }))
}

use crate::{config, db::DbPool, error::AppError, models::Summary, summarizer};
use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};
use sqlx::Row;
use tracing::{error, info};
use uuid::Uuid;

pub fn router() -> Router<DbPool> {
    Router::new().route(
        "/articles/:article_id/summary",
        get(get_summary).post(create_summary),
    )
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

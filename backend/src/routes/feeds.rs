use crate::{
    db::DbPool,
    error::AppError,
    feeds::FeedFetcher,
    models::{
        CreateFeedDto, Feed, ToggleFeedStatusRequest, ToggleFeedStatusResponse, UpdateFeedDto,
    },
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, patch, post},
    Json, Router,
};
use sqlx::Row;
use tracing::{error, info};
use uuid::Uuid;

pub fn router() -> Router<DbPool> {
    Router::new()
        .route("/feeds", get(list_feeds).post(create_feed))
        .route(
            "/feeds/:id",
            get(get_feed).put(update_feed).delete(delete_feed),
        )
        .route("/feeds/:id/refresh", post(refresh_feed))
        .route("/feeds/:id/toggle-status", patch(toggle_feed_status))
        .route("/feeds/refresh-all-active", post(refresh_all_active_feeds))
}

// List all feeds
async fn list_feeds(State(pool): State<DbPool>) -> Result<Json<Vec<Feed>>, AppError> {
    let feeds = sqlx::query("SELECT * FROM feeds ORDER BY title")
        .map(|row: sqlx::postgres::PgRow| Feed {
            id: row.get("id"),
            title: row.get("title"),
            url: row.get("url"),
            active: row.get("active"),
            last_fetched: row.get("last_fetched"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
        .fetch_all(&pool)
        .await
        .map_err(|e| {
            error!("Failed to fetch feeds: {:?}", e);
            AppError::DatabaseError(e.to_string())
        })?;

    Ok(Json(feeds))
}

// Create a new feed
async fn create_feed(
    State(pool): State<DbPool>,
    Json(payload): Json<CreateFeedDto>,
) -> Result<(StatusCode, Json<Feed>), AppError> {
    // Check if feed with the same URL already exists
    let existing = sqlx::query("SELECT id FROM feeds WHERE url = $1")
        .bind(&payload.url)
        .fetch_optional(&pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    if existing.is_some() {
        return Err(AppError::BadRequest(
            "Feed with this URL already exists".to_string(),
        ));
    }

    // Create the feed
    let feed = sqlx::query(
        r#"
        INSERT INTO feeds (title, url)
        VALUES ($1, $2)
        RETURNING *
        "#,
    )
    .bind(&payload.title)
    .bind(&payload.url)
    .map(|row: sqlx::postgres::PgRow| Feed {
        id: row.get("id"),
        title: row.get("title"),
        url: row.get("url"),
        active: row.get("active"),
        last_fetched: row.get("last_fetched"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    })
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        error!("Failed to create feed: {:?}", e);
        AppError::DatabaseError(e.to_string())
    })?;

    info!("Created new feed: {} ({})", feed.title, feed.id);
    Ok((StatusCode::CREATED, Json(feed)))
}

// Get a specific feed by ID
async fn get_feed(
    State(pool): State<DbPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<Feed>, AppError> {
    let feed = sqlx::query("SELECT * FROM feeds WHERE id = $1")
        .bind(id)
        .map(|row: sqlx::postgres::PgRow| Feed {
            id: row.get("id"),
            title: row.get("title"),
            url: row.get("url"),
            active: row.get("active"),
            last_fetched: row.get("last_fetched"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
        .fetch_optional(&pool)
        .await
        .map_err(|e| {
            error!("Failed to fetch feed {}: {:?}", id, e);
            AppError::DatabaseError(e.to_string())
        })?
        .ok_or_else(|| AppError::NotFound(format!("Feed with ID {} not found", id)))?;

    Ok(Json(feed))
}

// Update an existing feed
async fn update_feed(
    State(pool): State<DbPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateFeedDto>,
) -> Result<Json<Feed>, AppError> {
    // Check if feed exists
    let feed_exists = sqlx::query("SELECT id FROM feeds WHERE id = $1")
        .bind(id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .is_some();

    if !feed_exists {
        return Err(AppError::NotFound(format!("Feed with ID {} not found", id)));
    }

    // If URL is provided, check if it's already used by another feed
    if let Some(url) = &payload.url {
        let existing = sqlx::query("SELECT id FROM feeds WHERE url = $1 AND id != $2")
            .bind(url)
            .bind(id)
            .fetch_optional(&pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        if existing.is_some() {
            return Err(AppError::BadRequest(
                "Feed with this URL already exists".to_string(),
            ));
        }
    }

    // Update the feed
    let feed = sqlx::query(
        r#"
        UPDATE feeds
        SET
            title = COALESCE($1, title),
            url = COALESCE($2, url),
            updated_at = NOW()
        WHERE id = $3
        RETURNING *
        "#,
    )
    .bind(payload.title)
    .bind(payload.url)
    .bind(id)
    .map(|row: sqlx::postgres::PgRow| Feed {
        id: row.get("id"),
        title: row.get("title"),
        url: row.get("url"),
        active: row.get("active"),
        last_fetched: row.get("last_fetched"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    })
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        error!("Failed to update feed {}: {:?}", id, e);
        AppError::DatabaseError(e.to_string())
    })?;

    info!("Updated feed: {} ({})", feed.title, feed.id);
    Ok(Json(feed))
}

// Delete a feed
async fn delete_feed(
    State(pool): State<DbPool>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    // Check if feed exists
    let feed_exists = sqlx::query("SELECT id FROM feeds WHERE id = $1")
        .bind(id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .is_some();

    if !feed_exists {
        return Err(AppError::NotFound(format!("Feed with ID {} not found", id)));
    }

    // Delete the feed
    sqlx::query("DELETE FROM feeds WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|e| {
            error!("Failed to delete feed {}: {:?}", id, e);
            AppError::DatabaseError(e.to_string())
        })?;

    info!("Deleted feed: {}", id);
    Ok((StatusCode::OK, Json(serde_json::json!({ "success": true }))))
}

// Refresh a feed by fetching new articles
async fn refresh_feed(
    State(pool): State<DbPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, AppError> {
    // Check if feed exists
    let feed_exists = sqlx::query("SELECT id FROM feeds WHERE id = $1")
        .bind(id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .is_some();

    if !feed_exists {
        return Err(AppError::NotFound(format!("Feed with ID {} not found", id)));
    }

    // Create a feed fetcher
    let fetcher = FeedFetcher::new();

    // Fetch and save articles
    let count = fetcher.refresh_feed(&pool, id).await?;

    info!("Refreshed feed {} - {} new articles", id, count);
    Ok(Json(serde_json::json!({
        "success": true,
        "feed_id": id.to_string(),
        "articles_added": count
    })))
}

// Toggle feed active/inactive status
async fn toggle_feed_status(
    State(pool): State<DbPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<ToggleFeedStatusRequest>,
) -> Result<Json<ToggleFeedStatusResponse>, AppError> {
    // Check if feed exists
    let feed_exists = sqlx::query("SELECT id FROM feeds WHERE id = $1")
        .bind(id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .is_some();

    if !feed_exists {
        return Err(AppError::NotFound(format!("Feed with ID {} not found", id)));
    }

    // Update the feed's active status
    sqlx::query(
        r#"
        UPDATE feeds
        SET active = $1, updated_at = NOW()
        WHERE id = $2
        "#,
    )
    .bind(payload.active)
    .bind(id)
    .execute(&pool)
    .await
    .map_err(|e| {
        error!("Failed to toggle feed status {}: {:?}", id, e);
        AppError::DatabaseError(e.to_string())
    })?;

    let status_text = if payload.active {
        "activated"
    } else {
        "deactivated"
    };
    let message = format!("Feed successfully {}", status_text);

    info!("Feed {} {}", id, status_text);

    Ok(Json(ToggleFeedStatusResponse {
        feed_id: id,
        active: payload.active,
        message,
    }))
}

// Refresh all active feeds
async fn refresh_all_active_feeds(
    State(pool): State<DbPool>,
) -> Result<Json<serde_json::Value>, AppError> {
    // Create a feed fetcher
    let fetcher = FeedFetcher::new();

    // Refresh all active feeds
    let result = fetcher.refresh_all_active_feeds(&pool).await?;

    Ok(Json(result))
}

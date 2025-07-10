use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// Database models - these match the database schema directly
#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Feed {
    pub id: Uuid,
    pub title: String,
    pub url: String,
    pub last_fetched: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Article {
    pub id: Uuid,
    pub title: String,
    pub url: String,
    pub feed_id: Uuid,
    pub content: String,
    pub published_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Summary {
    pub id: Uuid,
    pub article_id: Uuid,
    pub content: String,
    pub model: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// DTO models - these are used for API requests
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateFeedDto {
    pub title: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateFeedDto {
    pub title: Option<String>,
    pub url: Option<String>,
}

// New DTOs for bulk article fetching
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BulkFetchRequest {
    pub feed_ids: Vec<Uuid>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BulkFetchResponse {
    pub articles: Vec<Article>,
    pub total_count: i64,
    pub feed_summaries: Vec<FeedSummary>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FeedSummary {
    pub feed_id: Uuid,
    pub feed_title: String,
    pub article_count: i64,
}

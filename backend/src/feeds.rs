use crate::{
    error::AppError,
    models::{Article, Feed},
};
use chrono::Utc;
use feed_rs::{model::Feed as RssFeed, parser};
use reqwest::Client;
use sqlx::{PgPool, Row};
use tracing::{error, info};
use uuid::Uuid;

/// A struct to fetch and process RSS/Atom feeds
pub struct FeedFetcher {
    client: Client,
}

impl FeedFetcher {
    pub fn new() -> Self {
        let client = Client::builder()
            .user_agent("Laune RSS Reader/1.0")
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .unwrap_or_default();
        Self { client }
    }

    /// Fetch a feed and return its entries as Articles
    pub async fn fetch_feed(&self, feed: &Feed) -> Result<Vec<Article>, AppError> {
        info!("Fetching feed: {} ({})", feed.title, feed.url);

        let response = self
            .client
            .get(&feed.url)
            .send()
            .await
            .map_err(|e| AppError::FeedParsingError(format!("Failed to fetch feed: {}", e)))?;

        if !response.status().is_success() {
            return Err(AppError::FeedParsingError(format!(
                "Failed to fetch feed. Status: {}",
                response.status()
            )));
        }

        let xml = response
            .bytes()
            .await
            .map_err(|e| AppError::FeedParsingError(format!("Failed to read response: {}", e)))?;

        let parsed_feed = parser::parse(xml.as_ref())
            .map_err(|e| AppError::FeedParsingError(format!("Failed to parse XML: {}", e)))?;

        let articles = self.rss_to_articles(feed, parsed_feed)?;
        info!("Fetched {} articles from {}", articles.len(), feed.title);

        Ok(articles)
    }

    /// Convert RSS feed entries to our Article model
    fn rss_to_articles(&self, feed: &Feed, rss_feed: RssFeed) -> Result<Vec<Article>, AppError> {
        let articles = rss_feed
            .entries
            .into_iter()
            .map(|entry| {
                let content = entry
                    .content
                    .and_then(|c| c.body)
                    .or_else(|| entry.summary.map(|s| s.content))
                    .unwrap_or_default();

                let published = entry.published.or(entry.updated).unwrap_or_else(Utc::now);

                let url = entry
                    .links
                    .first()
                    .map(|l| l.href.clone())
                    .unwrap_or_default();

                let title = entry
                    .title
                    .map(|t| t.content)
                    .unwrap_or_else(|| "Untitled".to_string());

                Article {
                    id: Uuid::new_v4(),
                    title,
                    url,
                    feed_id: feed.id,
                    content,
                    published_at: published,
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                }
            })
            .collect();

        Ok(articles)
    }

    /// Fetch and save all articles from a feed to the database
    pub async fn refresh_feed(&self, pool: &PgPool, feed_id: Uuid) -> Result<usize, AppError> {
        // Get the feed from the database
        let feed = sqlx::query("SELECT * FROM feeds WHERE id = $1")
            .bind(feed_id)
            .map(|row: sqlx::postgres::PgRow| Feed {
                id: row.get("id"),
                title: row.get("title"),
                url: row.get("url"),
                last_fetched: row.get("last_fetched"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })
            .fetch_optional(pool)
            .await
            .map_err(|e| {
                error!("Database error fetching feed {}: {}", feed_id, e);
                AppError::DatabaseError(e.to_string())
            })?
            .ok_or_else(|| AppError::NotFound(format!("Feed with ID {} not found", feed_id)))?;

        // Fetch articles from the feed
        let articles = self.fetch_feed(&feed).await?;

        if articles.is_empty() {
            info!("No articles found in feed: {}", feed.title);

            // Update the last_fetched timestamp
            sqlx::query("UPDATE feeds SET last_fetched = NOW() WHERE id = $1")
                .bind(feed_id)
                .execute(pool)
                .await
                .map_err(|e| AppError::DatabaseError(e.to_string()))?;

            return Ok(0);
        }

        let mut saved_count = 0;

        // Save each article to the database
        for article in articles {
            // Check if article with this URL already exists
            let exists = sqlx::query("SELECT id FROM articles WHERE url = $1")
                .bind(&article.url)
                .fetch_optional(pool)
                .await
                .map_err(|e| AppError::DatabaseError(e.to_string()))?
                .is_some();

            if exists {
                continue;
            }

            // Insert the new article
            sqlx::query(
                r#"
                INSERT INTO articles
                (id, title, url, feed_id, content, published_at)
                VALUES ($1, $2, $3, $4, $5, $6)
                "#,
            )
            .bind(article.id)
            .bind(&article.title)
            .bind(&article.url)
            .bind(article.feed_id)
            .bind(&article.content)
            .bind(article.published_at)
            .execute(pool)
            .await
            .map_err(|e| {
                error!("Failed to save article '{}': {}", article.title, e);
                AppError::DatabaseError(e.to_string())
            })?;

            saved_count += 1;
        }

        // Update the last_fetched timestamp
        sqlx::query("UPDATE feeds SET last_fetched = NOW() WHERE id = $1")
            .bind(feed_id)
            .execute(pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        info!(
            "Saved {} new articles from feed: {}",
            saved_count, feed.title
        );
        Ok(saved_count)
    }
}

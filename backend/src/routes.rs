use crate::db::DbPool;
use axum::Router;

pub mod articles;
pub mod feeds;
pub mod summaries;

pub fn api_router() -> Router<DbPool> {
    Router::new()
        .merge(feeds::router())
        .merge(articles::router())
        .merge(summaries::router())
}

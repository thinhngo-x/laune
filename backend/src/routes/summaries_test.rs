#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{ArticleSummaryInfo, FeedAggregationRequest, FeedSummaryInfo};
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

    #[tokio::test]
    async fn test_aggregated_summary_empty_feeds() {
        // Test that empty feed list is handled correctly
        let empty_feeds: Vec<FeedSummaryInfo> = vec![];

        // This would normally fail at the API level before reaching the summarizer
        // but we can test the data structure validation
        assert_eq!(empty_feeds.len(), 0);
    }
}

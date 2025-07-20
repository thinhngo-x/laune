// Example usage of the aggregated summary endpoint
// This would typically be called from a frontend application

use serde_json::json;

fn main() {
    println!("Example request for aggregated feed summary:");

    // Example request payload
    let request_payload = json!({
        "feed_ids": [
            "550e8400-e29b-41d4-a716-446655440000",
            "6ba7b810-9dad-11d1-80b4-00c04fd430c8"
        ],
        "hours_back": 24
    });

    println!("POST /api/feeds/aggregate-summary");
    println!("Content-Type: application/json");
    println!();
    println!(
        "{}",
        serde_json::to_string_pretty(&request_payload).unwrap()
    );

    println!("\nExpected response structure:");
    let example_response = json!({
        "summary": "This is an aggregated summary of all articles from the selected feeds...",
        "feeds": [
            {
                "feed_id": "550e8400-e29b-41d4-a716-446655440000",
                "feed_title": "Tech News",
                "article_count": 5,
                "articles": [
                    {
                        "id": "article-uuid-1",
                        "title": "Latest AI Developments",
                        "url": "https://example.com/ai-news",
                        "published_at": "2024-01-15T10:30:00Z",
                        "summary": "Summary of the AI article..."
                    }
                ]
            }
        ],
        "total_articles": 10,
        "time_range_hours": 24
    });

    println!(
        "{}",
        serde_json::to_string_pretty(&example_response).unwrap()
    );
}

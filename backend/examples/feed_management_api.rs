// Example usage of the feed management API endpoints
use serde_json::json;

fn main() {
    println!("=== Feed Management API Examples ===\n");

    // Toggle feed status
    println!("1. Toggle Feed Status:");
    println!("PATCH /api/feeds/550e8400-e29b-41d4-a716-446655440000/toggle-status");
    println!("Content-Type: application/json");

    let toggle_request = json!({
        "active": false
    });

    println!(
        "{}\n",
        serde_json::to_string_pretty(&toggle_request).unwrap()
    );

    println!("Expected Response:");
    let toggle_response = json!({
        "feed_id": "550e8400-e29b-41d4-a716-446655440000",
        "active": false,
        "message": "Feed successfully deactivated"
    });

    println!(
        "{}\n",
        serde_json::to_string_pretty(&toggle_response).unwrap()
    );

    // Refresh all active feeds
    println!("2. Refresh All Active Feeds:");
    println!("POST /api/feeds/refresh-all-active");
    println!("(No request body required)\n");

    println!("Expected Response:");
    let refresh_response = json!({
        "success": true,
        "message": "Processed 3 active feeds",
        "feeds_processed": 3,
        "total_articles_added": 15,
        "results": [
            {
                "feed_id": "feed-uuid-1",
                "feed_title": "Tech News",
                "articles_added": 7,
                "success": true
            },
            {
                "feed_id": "feed-uuid-2",
                "feed_title": "Science Today",
                "articles_added": 5,
                "success": true
            },
            {
                "feed_id": "feed-uuid-3",
                "feed_title": "Business Updates",
                "articles_added": 3,
                "success": true
            }
        ]
    });

    println!(
        "{}\n",
        serde_json::to_string_pretty(&refresh_response).unwrap()
    );

    // Create feed (now includes active field)
    println!("3. Feed Model (now includes 'active' field):");
    let feed_example = json!({
        "id": "550e8400-e29b-41d4-a716-446655440000",
        "title": "Example News Feed",
        "url": "https://example.com/rss",
        "active": true,
        "last_fetched": "2025-01-15T10:30:00Z",
        "created_at": "2025-01-01T00:00:00Z",
        "updated_at": "2025-01-15T10:30:00Z"
    });

    println!("{}\n", serde_json::to_string_pretty(&feed_example).unwrap());

    println!("=== Key Features ===");
    println!("✅ Feeds can be activated/deactivated");
    println!("✅ Only active feeds are processed during refresh");
    println!("✅ Bulk refresh of all active feeds");
    println!("✅ Inactive feeds are skipped in aggregated summaries");
    println!("✅ Database migration adds 'active' column (defaults to true)");
}

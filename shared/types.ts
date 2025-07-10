/**
 * Shared types between the frontend and backend
 * These types should match the structures defined in both codebases
 */

/**
 * Feed represents an RSS/Atom feed source
 */
export interface Feed {
  id: string;
  title: string;
  url: string;
  last_fetched?: string; // ISO8601 date string
}

/**
 * Article represents a single article from a feed
 */
export interface Article {
  id: string;
  title: string;
  url: string;
  feed_id: string;
  content: string;
  published_at: string; // ISO8601 date string
}

/**
 * Summary represents an AI-generated summary of an article
 */
export interface Summary {
  id: string;
  article_id: string;
  content: string;
  created_at: string; // ISO8601 date string
  model: string;
}

/**
 * Request for bulk fetching articles from selected feeds with time constraints
 */
export interface BulkFetchRequest {
  feed_ids: string[];
  start_date?: string; // ISO8601 date string
  end_date?: string; // ISO8601 date string
  limit?: number;
  offset?: number;
}

/**
 * Response for bulk article fetch including feed summaries
 */
export interface BulkFetchResponse {
  articles: Article[];
  total_count: number;
  feed_summaries: FeedSummary[];
}

/**
 * Summary of articles count per feed
 */
export interface FeedSummary {
  feed_id: string;
  feed_title: string;
  article_count: number;
}

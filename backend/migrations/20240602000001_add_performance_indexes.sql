-- Add additional indexes for better performance on bulk fetch queries
CREATE INDEX IF NOT EXISTS idx_articles_published_at_feed_id ON articles (published_at DESC, feed_id);

CREATE INDEX IF NOT EXISTS idx_articles_feed_id_published_at ON articles (feed_id, published_at DESC);
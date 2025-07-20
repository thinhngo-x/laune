-- Add active column to feeds table
ALTER TABLE feeds
ADD COLUMN active BOOLEAN NOT NULL DEFAULT true;

-- Create index on active column for better query performance
CREATE INDEX idx_feeds_active ON feeds (active);

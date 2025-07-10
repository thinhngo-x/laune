export interface Feed {
  id: string;
  title: string;
  url: string;
  lastFetched?: string;
}

export interface Article {
  id: string;
  title: string;
  url: string;
  feedId: string;
  content: string;
  publishedAt: string;
}

export interface Summary {
  id: string;
  articleId: string;
  content: string;
  createdAt: string;
  model: string;
}

export interface BulkFetchRequest {
  feedIds: string[];
  startDate?: string;
  endDate?: string;
  limit?: number;
  offset?: number;
}

export interface BulkFetchResponse {
  articles: Article[];
  totalCount: number;
  feedSummaries: FeedSummary[];
}

export interface FeedSummary {
  feedId: string;
  feedTitle: string;
  articleCount: number;
}

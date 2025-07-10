import axios from 'axios';
import {
  Feed,
  Article,
  Summary,
  BulkFetchRequest,
  BulkFetchResponse,
  FeedSummary,
} from '../types';

const apiClient = axios.create({
  baseURL: '/api',
  headers: {
    'Content-Type': 'application/json',
  },
});

// Transform snake_case to camelCase for Feed objects
const transformFeed = (feed: any): Feed => ({
  id: feed.id,
  title: feed.title,
  url: feed.url,
  lastFetched: feed.last_fetched,
});

// Transform snake_case to camelCase for Article objects
const transformArticle = (article: any): Article => ({
  id: article.id,
  title: article.title,
  url: article.url,
  feedId: article.feed_id,
  content: article.content,
  publishedAt: article.published_at,
});

// Transform snake_case to camelCase for Summary objects
const transformSummary = (summary: any): Summary => ({
  id: summary.id,
  articleId: summary.article_id,
  content: summary.content,
  createdAt: summary.created_at,
  model: summary.model,
});

// Transform snake_case to camelCase for FeedSummary objects
const transformFeedSummary = (feedSummary: any): FeedSummary => ({
  feedId: feedSummary.feed_id,
  feedTitle: feedSummary.feed_title,
  articleCount: feedSummary.article_count,
});

// Transform snake_case to camelCase for BulkFetchResponse objects
const transformBulkFetchResponse = (response: any): BulkFetchResponse => ({
  articles: response.articles.map(transformArticle),
  totalCount: response.total_count,
  feedSummaries: response.feed_summaries.map(transformFeedSummary),
});

// Feeds API
export const fetchFeeds = async (): Promise<Feed[]> => {
  const { data } = await apiClient.get<any[]>('/feeds');
  return data.map(transformFeed);
};

export const createFeed = async (feedData: Omit<Feed, 'id'>): Promise<Feed> => {
  const { data } = await apiClient.post<any>('/feeds', feedData);
  return transformFeed(data);
};

export const deleteFeed = async (feedId: string): Promise<void> => {
  await apiClient.delete(`/feeds/${feedId}`);
};

// Articles API
export const fetchArticles = async (feedId?: string): Promise<Article[]> => {
  const url = feedId ? `/feeds/${feedId}/articles` : '/articles';
  const { data } = await apiClient.get<any[]>(url);
  return data.map(transformArticle);
};

export const fetchArticle = async (articleId: string): Promise<Article> => {
  const { data } = await apiClient.get<any>(`/articles/${articleId}`);
  return transformArticle(data);
};

// Bulk fetch articles from selected feeds with time constraints
export const bulkFetchArticles = async (
  request: BulkFetchRequest
): Promise<BulkFetchResponse> => {
  // Transform camelCase to snake_case for the API request
  const apiRequest = {
    feed_ids: request.feedIds,
    start_date: request.startDate,
    end_date: request.endDate,
    limit: request.limit,
    offset: request.offset,
  };

  const { data } = await apiClient.post<any>(
    '/articles/bulk-fetch',
    apiRequest
  );
  return transformBulkFetchResponse(data);
};

// Summaries API
export const fetchSummary = async (
  articleId: string
): Promise<Summary | null> => {
  const { data } = await apiClient.get<any>(`/articles/${articleId}/summary`);
  return data ? transformSummary(data) : null;
};

export const generateSummary = async (articleId: string): Promise<Summary> => {
  const { data } = await apiClient.post<any>(`/articles/${articleId}/summary`);
  return transformSummary(data);
};

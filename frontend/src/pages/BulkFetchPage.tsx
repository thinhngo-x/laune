import React, { useState, useEffect } from 'react';
import { Feed, BulkFetchRequest, BulkFetchResponse, Article } from '../types';
import { fetchFeeds, bulkFetchArticles } from '../api/client';

interface DateRange {
  startDate: string;
  endDate: string;
}

const BulkFetchPage: React.FC = () => {
  const [feeds, setFeeds] = useState<Feed[]>([]);
  const [selectedFeedIds, setSelectedFeedIds] = useState<string[]>([]);
  const [dateRange, setDateRange] = useState<DateRange>({
    startDate: '',
    endDate: '',
  });
  const [loading, setLoading] = useState(false);
  const [results, setResults] = useState<BulkFetchResponse | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [limit, setLimit] = useState(50);
  const [offset, setOffset] = useState(0);

  useEffect(() => {
    loadFeeds();
  }, []);

  const loadFeeds = async () => {
    try {
      const feedData = await fetchFeeds();
      setFeeds(feedData);
    } catch (err) {
      setError('Failed to load feeds');
      console.error('Error loading feeds:', err);
    }
  };

  const handleFeedToggle = (feedId: string) => {
    setSelectedFeedIds(prev =>
      prev.includes(feedId)
        ? prev.filter(id => id !== feedId)
        : [...prev, feedId]
    );
  };

  const handleSelectAll = () => {
    setSelectedFeedIds(feeds.map(feed => feed.id));
  };

  const handleSelectNone = () => {
    setSelectedFeedIds([]);
  };

  const handleDateChange = (field: keyof DateRange, value: string) => {
    setDateRange(prev => ({
      ...prev,
      [field]: value,
    }));
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    if (selectedFeedIds.length === 0) {
      setError('Please select at least one feed');
      return;
    }

    setLoading(true);
    setError(null);

    try {
      const request: BulkFetchRequest = {
        feedIds: selectedFeedIds,
        startDate: dateRange.startDate || undefined,
        endDate: dateRange.endDate || undefined,
        limit,
        offset,
      };

      const response = await bulkFetchArticles(request);
      setResults(response);
    } catch (err) {
      setError('Failed to fetch articles');
      console.error('Error fetching articles:', err);
    } finally {
      setLoading(false);
    }
  };

  const handleLoadMore = async () => {
    if (!results || loading) return;

    setLoading(true);
    try {
      const request: BulkFetchRequest = {
        feedIds: selectedFeedIds,
        startDate: dateRange.startDate || undefined,
        endDate: dateRange.endDate || undefined,
        limit,
        offset: offset + limit,
      };

      const response = await bulkFetchArticles(request);
      setResults(prev =>
        prev
          ? {
              ...response,
              articles: [...prev.articles, ...response.articles],
            }
          : response
      );
      setOffset(prev => prev + limit);
    } catch (err) {
      setError('Failed to load more articles');
      console.error('Error loading more articles:', err);
    } finally {
      setLoading(false);
    }
  };

  const formatDate = (dateString: string) => {
    return new Date(dateString).toLocaleDateString('en-US', {
      year: 'numeric',
      month: 'short',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit',
    });
  };

  return (
    <div className="max-w-6xl mx-auto p-6">
      <h1 className="text-3xl font-bold text-gray-900 mb-6">
        Bulk Article Fetch
      </h1>

      <div className="bg-white shadow rounded-lg p-6 mb-6">
        <form onSubmit={handleSubmit} className="space-y-6">
          {/* Feed Selection */}
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              Select Feeds
            </label>
            <div className="mb-4">
              <button
                type="button"
                onClick={handleSelectAll}
                className="text-sm text-blue-600 hover:text-blue-800 mr-4"
              >
                Select All
              </button>
              <button
                type="button"
                onClick={handleSelectNone}
                className="text-sm text-blue-600 hover:text-blue-800"
              >
                Select None
              </button>
            </div>
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3 max-h-64 overflow-y-auto border border-gray-200 rounded-md p-3">
              {feeds.map(feed => (
                <label key={feed.id} className="flex items-center space-x-2">
                  <input
                    type="checkbox"
                    checked={selectedFeedIds.includes(feed.id)}
                    onChange={() => handleFeedToggle(feed.id)}
                    className="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                  />
                  <span
                    className="text-sm text-gray-700 truncate"
                    title={feed.title}
                  >
                    {feed.title}
                  </span>
                </label>
              ))}
            </div>
          </div>

          {/* Date Range */}
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                Start Date (Optional)
              </label>
              <input
                type="datetime-local"
                value={dateRange.startDate}
                onChange={e => handleDateChange('startDate', e.target.value)}
                className="block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500"
              />
            </div>
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                End Date (Optional)
              </label>
              <input
                type="datetime-local"
                value={dateRange.endDate}
                onChange={e => handleDateChange('endDate', e.target.value)}
                className="block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500"
              />
            </div>
          </div>

          {/* Limit */}
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">
              Articles per page
            </label>
            <select
              value={limit}
              onChange={e => setLimit(Number(e.target.value))}
              className="block w-40 rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500"
            >
              <option value={25}>25</option>
              <option value={50}>50</option>
              <option value={100}>100</option>
              <option value={200}>200</option>
            </select>
          </div>

          {/* Submit Button */}
          <div>
            <button
              type="submit"
              disabled={loading || selectedFeedIds.length === 0}
              className="bg-blue-600 text-white px-6 py-2 rounded-md hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed"
            >
              {loading ? 'Fetching...' : 'Fetch Articles'}
            </button>
          </div>
        </form>
      </div>

      {/* Error Message */}
      {error && (
        <div className="bg-red-50 border border-red-200 rounded-md p-4 mb-6">
          <p className="text-red-800">{error}</p>
        </div>
      )}

      {/* Results */}
      {results && (
        <div className="space-y-6">
          {/* Summary */}
          <div className="bg-blue-50 border border-blue-200 rounded-md p-4">
            <h2 className="text-lg font-semibold text-blue-900 mb-2">
              Fetch Results
            </h2>
            <p className="text-blue-800">
              Found {results.totalCount} total articles. Showing{' '}
              {results.articles.length} articles.
            </p>

            {results.feedSummaries.length > 0 && (
              <div className="mt-3">
                <h3 className="text-sm font-medium text-blue-900 mb-2">
                  Articles per Feed:
                </h3>
                <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-2">
                  {results.feedSummaries.map(summary => (
                    <div key={summary.feedId} className="text-sm text-blue-800">
                      {summary.feedTitle}: {summary.articleCount} articles
                    </div>
                  ))}
                </div>
              </div>
            )}
          </div>

          {/* Articles List */}
          <div className="bg-white shadow rounded-lg">
            <div className="px-6 py-4 border-b border-gray-200">
              <h2 className="text-lg font-semibold text-gray-900">
                Articles ({results.articles.length})
              </h2>
            </div>
            <div className="divide-y divide-gray-200">
              {results.articles.map(article => (
                <div key={article.id} className="p-6">
                  <div className="flex items-start justify-between">
                    <div className="flex-1">
                      <h3 className="text-lg font-medium text-gray-900 mb-2">
                        <a
                          href={article.url}
                          target="_blank"
                          rel="noopener noreferrer"
                          className="hover:text-blue-600"
                        >
                          {article.title}
                        </a>
                      </h3>
                      <p className="text-sm text-gray-600 mb-2">
                        Published: {formatDate(article.publishedAt)}
                      </p>
                      <div className="text-sm text-gray-800 line-clamp-3">
                        {article.content.substring(0, 200)}...
                      </div>
                    </div>
                  </div>
                </div>
              ))}
            </div>

            {/* Load More Button */}
            {results.articles.length < results.totalCount && (
              <div className="px-6 py-4 border-t border-gray-200">
                <button
                  onClick={handleLoadMore}
                  disabled={loading}
                  className="w-full bg-gray-100 text-gray-700 px-4 py-2 rounded-md hover:bg-gray-200 disabled:opacity-50"
                >
                  {loading
                    ? 'Loading...'
                    : `Load More (${
                        results.totalCount - results.articles.length
                      } remaining)`}
                </button>
              </div>
            )}
          </div>
        </div>
      )}
    </div>
  );
};

export default BulkFetchPage;

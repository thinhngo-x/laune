import { useParams } from "react-router-dom";
import { useQuery } from "@tanstack/react-query";
import { fetchArticles, fetchFeeds } from "../api/client";

const FeedPage = () => {
  const { feedId } = useParams<{ feedId: string }>();

  const { data: feeds = [], isLoading: feedsLoading } = useQuery({
    queryKey: ["feeds"],
    queryFn: fetchFeeds,
  });

  const {
    data: articles = [],
    isLoading: articlesLoading,
    error,
  } = useQuery({
    queryKey: ["articles", feedId],
    queryFn: () => fetchArticles(feedId),
    enabled: !!feedId,
  });

  const currentFeed = feeds.find((feed) => feed.id === feedId);

  if (feedsLoading || articlesLoading) {
    return (
      <div className="flex items-center justify-center h-64">
        <div className="animate-spin rounded-full h-12 w-12 border-t-2 border-b-2 border-primary-600"></div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="rounded-md bg-red-50 dark:bg-red-900/30 p-4">
        <div className="flex">
          <div className="text-red-700 dark:text-red-400">
            <p>Error loading articles for this feed</p>
          </div>
        </div>
      </div>
    );
  }

  if (!currentFeed) {
    return (
      <div className="rounded-md bg-yellow-50 dark:bg-yellow-900/30 p-4">
        <div className="flex">
          <div className="text-yellow-700 dark:text-yellow-400">
            <p>Feed not found</p>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div>
      <div className="mb-6">
        <h1 className="text-2xl font-bold">{currentFeed.title}</h1>
        <p className="text-gray-500 dark:text-gray-400 text-sm mt-1">
          {currentFeed.url}
        </p>
        {currentFeed.lastFetched && (
          <p className="text-gray-500 dark:text-gray-400 text-sm">
            Last updated:{" "}
            {new Date(currentFeed.lastFetched).toLocaleDateString()}
          </p>
        )}
      </div>

      {articles.length === 0 ? (
        <div className="bg-white dark:bg-gray-800 rounded-md shadow p-6 text-center">
          <h2 className="font-medium text-lg mb-2">No articles yet</h2>
          <p className="text-gray-500 dark:text-gray-400">
            This feed hasn't been refreshed yet or contains no articles.
          </p>
        </div>
      ) : (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          {articles.map((article) => (
            <div
              key={article.id}
              className="bg-white dark:bg-gray-800 rounded-md shadow overflow-hidden flex flex-col"
            >
              <div className="p-6 flex-1">
                <h2 className="font-medium text-lg mb-2 line-clamp-2">
                  {article.title}
                </h2>
                <p className="text-gray-500 dark:text-gray-400 text-sm mb-3">
                  {article.publishedAt
                    ? new Date(article.publishedAt).toLocaleDateString()
                    : "No date available"}
                </p>
                <p className="text-gray-700 dark:text-gray-300 text-sm line-clamp-3">
                  {article.content
                    ? article.content
                        .replace(/<[^>]*>/g, "")
                        .substring(0, 150) + "..."
                    : "No preview available"}
                </p>
              </div>
              <div className="p-6 pt-0">
                <a
                  href={`/articles/${article.id}`}
                  className="text-primary-600 dark:text-primary-400 text-sm font-medium hover:underline"
                >
                  Read more →
                </a>
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
};

export default FeedPage;

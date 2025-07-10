import { useQuery } from '@tanstack/react-query';
import { fetchArticles } from '../api/client';

const HomePage = () => {
  const {
    data: articles = [],
    isLoading,
    error,
  } = useQuery({
    queryKey: ['articles'],
    queryFn: () => fetchArticles(),
  });

  if (isLoading) {
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
            <p>Error loading articles</p>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div>
      <h1 className="text-2xl font-bold mb-6">Latest Articles</h1>

      {articles.length === 0 ? (
        <div className="bg-white dark:bg-gray-800 rounded-md shadow p-6 text-center">
          <h2 className="font-medium text-lg mb-2">No articles yet</h2>
          <p className="text-gray-500 dark:text-gray-400">
            Add some feeds to get started and see articles here.
          </p>
        </div>
      ) : (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          {articles.map(article => (
            <div
              key={article.id}
              className="bg-white dark:bg-gray-800 rounded-md shadow overflow-hidden flex flex-col"
            >
              <div className="p-4 flex-1">
                <h2 className="font-medium text-lg mb-2 line-clamp-2">
                  {article.title}
                </h2>
                <p className="text-gray-500 dark:text-gray-400 text-sm mb-4">
                  {new Date(article.publishedAt).toLocaleDateString()}
                </p>
                <div className="text-sm line-clamp-3 mb-4">
                  {/* Display a text-only preview of the content */}
                  {article.content.replace(/<[^>]*>/g, '').substring(0, 120)}...
                </div>
                <a
                  href={`/articles/${article.id}`}
                  className="text-primary-600 dark:text-primary-400 text-sm font-medium hover:underline"
                >
                  Read more
                </a>
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
};

export default HomePage;

import { useState } from "react";
import { useParams, Link } from "react-router-dom";
import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { fetchArticle, fetchSummary, generateSummary } from "../api/client";

const ArticlePage = () => {
  const queryClient = useQueryClient();
  const { articleId } = useParams<{ articleId: string }>();
  const [showFullArticle, setShowFullArticle] = useState(false);

  const {
    data: article,
    isLoading: articleLoading,
    error: articleError,
  } = useQuery({
    queryKey: ["article", articleId],
    queryFn: () => fetchArticle(articleId!),
    enabled: !!articleId,
  });

  const { data: summary, isLoading: summaryLoading } = useQuery({
    queryKey: ["summary", articleId],
    queryFn: () => fetchSummary(articleId!),
    enabled: !!articleId,
  });

  const generateMutation = useMutation({
    mutationFn: generateSummary,
    onSuccess: () => {
      // Refetch the summary after generating
      queryClient.invalidateQueries({ queryKey: ["summary", articleId] });
    },
  });

  if (articleLoading) {
    return <div className="animate-pulse p-4">Loading article...</div>;
  }

  if (articleError || !article) {
    return (
      <div className="rounded-md bg-red-50 dark:bg-red-900/30 p-4">
        <p className="text-red-700 dark:text-red-400">
          Error loading article. It may have been removed or is unavailable.
        </p>
        <Link
          to="/"
          className="text-primary-600 dark:text-primary-400 mt-2 inline-block"
        >
          Return to home
        </Link>
      </div>
    );
  }

  const handleGenerateSummary = () => {
    if (articleId) {
      generateMutation.mutate(articleId);
    }
  };

  return (
    <div className="max-w-4xl mx-auto">
      <div className="mb-6">
        <Link
          to="/"
          className="text-primary-600 dark:text-primary-400 hover:underline"
        >
          ← Back to articles
        </Link>
      </div>

      <article className="bg-white dark:bg-gray-800 rounded-lg shadow-md overflow-hidden">
        <div className="p-6">
          <h1 className="text-2xl font-bold mb-2">{article.title}</h1>
          <p className="text-gray-500 dark:text-gray-400 text-sm mb-4">
            Published on {new Date(article.publishedAt).toLocaleDateString()}
          </p>

          <div className="flex space-x-4 mb-8">
            <a
              href={article.url}
              target="_blank"
              rel="noopener noreferrer"
              className="text-primary-600 dark:text-primary-400 hover:underline"
            >
              Original Article
            </a>
            <button
              onClick={() => setShowFullArticle(!showFullArticle)}
              className="text-gray-700 dark:text-gray-300 hover:underline"
            >
              {showFullArticle ? "Hide full content" : "Show full content"}
            </button>
          </div>

          {/* AI Summary Section */}
          <div className="mb-8 border border-gray-200 dark:border-gray-700 rounded-md p-4 bg-gray-50 dark:bg-gray-900">
            <h2 className="text-lg font-medium mb-3">AI Summary</h2>

            {summaryLoading ? (
              <div className="animate-pulse h-24 bg-gray-200 dark:bg-gray-700 rounded"></div>
            ) : summary ? (
              <div className="prose dark:prose-invert">
                <p>{summary.content}</p>
                <div className="text-xs text-gray-500 dark:text-gray-400 mt-2">
                  Summarized by {summary.model}
                </div>
              </div>
            ) : (
              <div className="text-center py-6">
                <p className="text-gray-500 dark:text-gray-400 mb-4">
                  No summary available for this article yet.
                </p>
                <button
                  onClick={handleGenerateSummary}
                  className="btn btn-primary"
                  disabled={generateMutation.isPending}
                >
                  {generateMutation.isPending
                    ? "Generating..."
                    : "Generate Summary"}
                </button>
              </div>
            )}
          </div>

          {/* Article Content */}
          {showFullArticle && (
            <div className="border-t border-gray-200 dark:border-gray-700 pt-6 mt-6">
              <h2 className="text-lg font-medium mb-4">Full Content</h2>
              <div
                className="article-content"
                dangerouslySetInnerHTML={{ __html: article.content }}
              />
            </div>
          )}
        </div>
      </article>
    </div>
  );
};

export default ArticlePage;

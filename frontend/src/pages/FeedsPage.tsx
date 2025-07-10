import { useState } from 'react';
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import { fetchFeeds, createFeed, deleteFeed } from '../api/client';
import { Feed } from '../types';

const FeedsPage = () => {
  const queryClient = useQueryClient();
  const [newFeed, setNewFeed] = useState<Omit<Feed, 'id'>>({
    title: '',
    url: '',
  });

  const { data: feeds = [], isLoading } = useQuery({
    queryKey: ['feeds'],
    queryFn: fetchFeeds,
  });

  const createMutation = useMutation({
    mutationFn: createFeed,
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['feeds'] });
      setNewFeed({ title: '', url: '' });
    },
  });

  const deleteMutation = useMutation({
    mutationFn: deleteFeed,
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['feeds'] });
    },
  });

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    createMutation.mutate(newFeed);
  };

  return (
    <div>
      <h1 className="text-2xl font-bold mb-6">Manage Feeds</h1>

      <div className="bg-white dark:bg-gray-800 rounded-md shadow p-6 mb-8">
        <h2 className="text-lg font-medium mb-4">Add New Feed</h2>
        <form onSubmit={handleSubmit} className="space-y-4">
          <div>
            <label
              htmlFor="title"
              className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
            >
              Feed Title
            </label>
            <input
              type="text"
              id="title"
              className="w-full rounded-md border-gray-300 dark:border-gray-600 dark:bg-gray-700 shadow-sm focus:border-primary-500 focus:ring-primary-500"
              value={newFeed.title}
              onChange={e => setNewFeed({ ...newFeed, title: e.target.value })}
              required
            />
          </div>

          <div>
            <label
              htmlFor="url"
              className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
            >
              Feed URL
            </label>
            <input
              type="url"
              id="url"
              className="w-full rounded-md border-gray-300 dark:border-gray-600 dark:bg-gray-700 shadow-sm focus:border-primary-500 focus:ring-primary-500"
              value={newFeed.url}
              onChange={e => setNewFeed({ ...newFeed, url: e.target.value })}
              placeholder="https://example.com/feed.xml"
              required
            />
          </div>

          <div>
            <button
              type="submit"
              className="btn btn-primary"
              disabled={createMutation.isPending}
            >
              {createMutation.isPending ? 'Adding...' : 'Add Feed'}
            </button>
          </div>
        </form>
      </div>

      <div className="bg-white dark:bg-gray-800 rounded-md shadow overflow-hidden">
        <h2 className="text-lg font-medium p-6 border-b border-gray-200 dark:border-gray-700">
          Your Feeds
        </h2>

        {isLoading ? (
          <div className="p-6 text-center">Loading...</div>
        ) : feeds.length === 0 ? (
          <div className="p-6 text-center text-gray-500 dark:text-gray-400">
            No feeds added yet. Add your first feed above.
          </div>
        ) : (
          <ul>
            {feeds.map(feed => (
              <li
                key={feed.id}
                className="border-b border-gray-200 dark:border-gray-700 last:border-b-0"
              >
                <div className="flex items-center justify-between p-4">
                  <div>
                    <h3 className="font-medium">{feed.title}</h3>
                    <a
                      href={feed.url}
                      target="_blank"
                      rel="noopener noreferrer"
                      className="text-sm text-gray-500 dark:text-gray-400 hover:text-primary-600 dark:hover:text-primary-400"
                    >
                      {feed.url}
                    </a>
                  </div>
                  <button
                    onClick={() => deleteMutation.mutate(feed.id)}
                    className="p-2 text-red-600 hover:text-red-800 dark:text-red-400 dark:hover:text-red-300"
                    disabled={deleteMutation.isPending}
                  >
                    Delete
                  </button>
                </div>
              </li>
            ))}
          </ul>
        )}
      </div>
    </div>
  );
};

export default FeedsPage;

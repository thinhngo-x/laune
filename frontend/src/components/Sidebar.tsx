import { useState } from 'react';
import { Link, useLocation } from 'react-router-dom';
import { useQuery } from '@tanstack/react-query';
import { fetchFeeds } from '../api/client';

const Sidebar = () => {
  const [isOpen, setIsOpen] = useState(true);
  const location = useLocation();

  const { data: feeds = [] } = useQuery({
    queryKey: ['feeds'],
    queryFn: fetchFeeds,
  });

  return (
    <div
      className={`border-r border-gray-200 dark:border-gray-700 ${
        isOpen ? 'w-64' : 'w-16'
      } transition-width duration-300 ease-in-out`}
    >
      <div className="p-4">
        <button
          onClick={() => setIsOpen(!isOpen)}
          className="p-2 rounded-md hover:bg-gray-100 dark:hover:bg-gray-700"
        >
          {isOpen ? '◀' : '▶'}
        </button>

        {isOpen && (
          <div className="mt-6 space-y-6">
            <div>
              <h3 className="text-xs uppercase font-semibold text-gray-500 dark:text-gray-400 mb-2">
                Navigation
              </h3>
              <nav className="space-y-1">
                <Link
                  to="/"
                  className={`block px-3 py-2 rounded-md text-sm ${
                    location.pathname === '/'
                      ? 'bg-primary-50 text-primary-600 dark:bg-primary-900 dark:text-primary-400'
                      : 'text-gray-700 hover:bg-gray-100 dark:text-gray-300 dark:hover:bg-gray-700'
                  }`}
                >
                  Home
                </Link>
                <Link
                  to="/feeds"
                  className={`block px-3 py-2 rounded-md text-sm ${
                    location.pathname === '/feeds'
                      ? 'bg-primary-50 text-primary-600 dark:bg-primary-900 dark:text-primary-400'
                      : 'text-gray-700 hover:bg-gray-100 dark:text-gray-300 dark:hover:bg-gray-700'
                  }`}
                >
                  Manage Feeds
                </Link>
                <Link
                  to="/bulk-fetch"
                  className={`block px-3 py-2 rounded-md text-sm ${
                    location.pathname === '/bulk-fetch'
                      ? 'bg-primary-50 text-primary-600 dark:bg-primary-900 dark:text-primary-400'
                      : 'text-gray-700 hover:bg-gray-100 dark:text-gray-300 dark:hover:bg-gray-700'
                  }`}
                >
                  Bulk Fetch Articles
                </Link>
              </nav>
            </div>

            <div>
              <div className="flex items-center justify-between mb-2">
                <h3 className="text-xs uppercase font-semibold text-gray-500 dark:text-gray-400">
                  My Feeds
                </h3>
                <Link
                  to="/feeds"
                  className="text-xs text-primary-600 hover:text-primary-700 dark:text-primary-400 dark:hover:text-primary-300"
                >
                  Add
                </Link>
              </div>

              <nav className="space-y-1">
                {feeds.map(feed => (
                  <Link
                    key={feed.id}
                    to={`/feeds/${feed.id}`}
                    className="block px-3 py-2 rounded-md text-sm truncate text-gray-700 hover:bg-gray-100 dark:text-gray-300 dark:hover:bg-gray-700"
                  >
                    {feed.title}
                  </Link>
                ))}
                {feeds.length === 0 && (
                  <div className="px-3 py-2 text-sm text-gray-500 dark:text-gray-400 italic">
                    No feeds yet
                  </div>
                )}
              </nav>
            </div>
          </div>
        )}
      </div>
    </div>
  );
};

export default Sidebar;

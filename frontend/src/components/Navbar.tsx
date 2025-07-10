import { Link } from 'react-router-dom';

const Navbar = () => {
  return (
    <header className="bg-white border-b border-gray-200 dark:bg-gray-800 dark:border-gray-700">
      <div className="container mx-auto px-4 py-3 flex items-center justify-between">
        <div className="flex items-center space-x-2">
          <Link
            to="/"
            className="text-xl font-bold text-primary-600 dark:text-primary-400"
          >
            Laune
          </Link>
          <span className="text-xs bg-gray-100 text-gray-600 dark:bg-gray-700 dark:text-gray-300 px-2 py-1 rounded">
            Beta
          </span>
        </div>

        <nav className="flex items-center space-x-4">
          <Link
            to="/feeds"
            className="text-sm font-medium text-gray-600 hover:text-primary-600 dark:text-gray-300 dark:hover:text-primary-400"
          >
            Feeds
          </Link>
        </nav>
      </div>
    </header>
  );
};

export default Navbar;

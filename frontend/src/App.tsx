import { Routes, Route } from "react-router-dom";
import Layout from "./components/Layout";
import HomePage from "./pages/HomePage";
import FeedsPage from "./pages/FeedsPage";
import FeedPage from "./pages/FeedPage";
import ArticlePage from "./pages/ArticlePage";
import BulkFetchPage from "./pages/BulkFetchPage";
import NotFoundPage from "./pages/NotFoundPage";

function App() {
  return (
    <Routes>
      <Route path="/" element={<Layout />}>
        <Route index element={<HomePage />} />
        <Route path="feeds" element={<FeedsPage />} />
        <Route path="feeds/:feedId" element={<FeedPage />} />
        <Route path="articles/:articleId" element={<ArticlePage />} />
        <Route path="bulk-fetch" element={<BulkFetchPage />} />
        <Route path="*" element={<NotFoundPage />} />
      </Route>
    </Routes>
  );
}

export default App;

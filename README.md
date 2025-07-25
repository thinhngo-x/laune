# Laune 📰

A modern RSS reader and article summarization platform built with Rust and TypeScript. Laune fetches
articles from RSS feeds and provides AI-powered summaries for quick content consumption.

## ✨ Features

- 🌐 **RSS Feed Management**: Add, manage, and organize RSS/Atom feeds
- � **Feed Status Control**: Enable/disable feeds and refresh only active ones
- �📄 **Bulk Article Fetching**: Retrieve articles from multiple feeds with date filtering
- 🤖 **AI Summarization**: Generate intelligent summaries using OpenAI GPT models
- 📊 **Aggregated Summaries**: Create comprehensive summaries from multiple feeds
- 📱 **Responsive UI**: Modern React interface with Tailwind CSS
- ⚡ **Real-time Updates**: Always fetch latest articles from online sources
- 🔍 **Advanced Filtering**: Filter articles by feed, date range, and pagination
- � **Feed Analytics**: View article counts and feed statistics

## 🏗️ Project Structure

```
laune/
├── backend/           # Rust server with Axum framework
│   ├── src/
│   │   ├── routes/    # API route handlers
│   │   ├── models.rs  # Data models and DTOs
│   │   ├── feeds.rs   # RSS feed fetching logic
│   │   └── ...
│   └── migrations/    # Database schema migrations
├── frontend/          # React TypeScript application
│   ├── src/
│   │   ├── components/ # Reusable UI components
│   │   ├── pages/     # Application pages
│   │   ├── api/       # API client
│   │   └── ...
├── shared/            # Shared TypeScript types
└── docs/              # Documentation
```

## 🐳 Docker Deployment

Laune can be deployed using Docker with the provided `docker-compose.yml` file.

### Prerequisites

- **Docker** - [Install Docker](https://docs.docker.com/get-docker/)
- **Docker Compose** - [Install Docker Compose](https://docs.docker.com/compose/install/)
- **OpenAI API Key** - [Get API Key](https://openai.com/api/)

### Quick Start with Docker

1. **Create environment file**:

   ```bash
   cp .env.example .env
   # Edit .env and add your OpenAI API key
   nano .env
   ```

2. **Start the services**:

   ```bash
   docker-compose up -d
   # This will start PostgreSQL, Redis, backend, and frontend services
   ```

3. **Access the application**:
   - Frontend: http://localhost
   - Backend API: http://localhost:8080
   - Health check: http://localhost:8080/health

4. **View logs**:

   ```bash
   # View all logs
   docker-compose logs

   # View logs for a specific service
   docker-compose logs backend
   docker-compose logs frontend
   ```

5. **Stop the services**:

   ```bash
   docker-compose down
   ```

### Development with Docker

For development with hot reload, use the `docker-compose.dev.yml` file:

```bash
cp .env.example .env
docker-compose -f docker-compose.dev.yml up
```

This will start the services with hot reload enabled for both backend and frontend.

### Customizing Docker Configuration

You can override any of the default values by creating a `.env` file in the project root. The
following variables can be customized:

- `POSTGRES_DB`: PostgreSQL database name
- `POSTGRES_USER`: PostgreSQL username
- `POSTGRES_PASSWORD`: PostgreSQL password
- `POSTGRES_PORT`: PostgreSQL port (default: 5432)
- `REDIS_PORT`: Redis port (default: 6379)
- `BACKEND_PORT`: Backend port (default: 8080)
- `FRONTEND_PORT`: Frontend port (default: 80)
- `RUST_LOG`: Rust logging level (default: info,laune_backend=debug)
- `OPENAI_API_KEY`: OpenAI API key (required)

Example `.env` file:

```env
POSTGRES_DB=laune_db
POSTGRES_USER=laune_user
POSTGRES_PASSWORD=secure_password
POSTGRES_PORT=5432
REDIS_PORT=6379
BACKEND_PORT=8080
FRONTEND_PORT=80
RUST_LOG=debug
OPENAI_API_KEY=your_actual_api_key_here
```

## 🚀 Getting Started

### Prerequisites

- **Rust** (1.70+) - [Install Rust](https://rustup.rs/)
- **Node.js** (18+) - [Install Node.js](https://nodejs.org/)
- **pnpm** - [Install pnpm](https://pnpm.io/installation)
- **PostgreSQL** (13+) - [Install PostgreSQL](https://postgresql.org/)
- **OpenAI API Key** - [Get API Key](https://openai.com/api/)

### Database Setup

1. **Create Database**:

   ```bash
   createdb laune_db
   ```

2. **Environment Configuration**: Create `backend/.env` (copy from `backend/.env.example`):

   ```env
   DATABASE_URL=postgres://username:password@localhost:5432/laune_db
   OPENAI_API_KEY=your_openai_api_key_here
   ```

3. **Database Migrations**: Automatically run when starting the backend.

### 🛠️ Development

#### Option 1: Quick Start Script

```bash
chmod +x start.sh
./start.sh
```

#### Option 2: Manual Setup

**Backend**:

```bash
cd backend
cargo run
# Server runs on http://localhost:8080
```

**Frontend**:

```bash
cd frontend
pnpm install
pnpm dev
# UI available at http://localhost:5173
```

### 🧪 Testing

**Test Bulk Fetch Feature**:

```bash
chmod +x test_bulk_fetch.sh
./test_bulk_fetch.sh
```

**Run Backend Tests**:

```bash
cd backend
cargo test
```

**Run Frontend Tests**:

```bash
cd frontend
pnpm test
```

## 📋 API Documentation

### Core Endpoints

- `GET /api/feeds` - List all feeds
- `POST /api/feeds` - Create new feed
- `GET /api/feeds/:id` - Get specific feed
- `PUT /api/feeds/:id` - Update feed
- `DELETE /api/feeds/:id` - Delete feed
- `POST /api/feeds/:id/refresh` - Refresh specific feed
- `PATCH /api/feeds/:id/toggle-status` - Toggle feed active/inactive status
- `POST /api/feeds/refresh-all-active` - Refresh all active feeds
- `GET /api/articles` - List articles with filtering
- `POST /api/articles/bulk-fetch` - Bulk fetch articles from selected feeds
- `POST /api/articles/:id/summary` - Generate article summary
- `POST /api/feeds/aggregate-summary` - Generate aggregated summary from multiple feeds

### Feed Management API

**Toggle Feed Status**:

```json
PATCH /api/feeds/:id/toggle-status
{
  "active": false
}
```

**Response**:

```json
{
  "feed_id": "uuid",
  "active": false,
  "message": "Feed successfully deactivated"
}
```

**Refresh All Active Feeds**:

```json
POST /api/feeds/refresh-all-active
```

**Response**:

```json
{
  "success": true,
  "message": "Processed 5 active feeds",
  "feeds_processed": 5,
  "total_articles_added": 23,
  "results": [
    {
      "feed_id": "uuid",
      "feed_title": "Tech News",
      "articles_added": 5,
      "success": true
    }
  ]
}
```

### Aggregated Summary API

The aggregated summary feature creates comprehensive summaries from multiple feeds within a time
constraint:

```json
POST /api/feeds/aggregate-summary
{
  "feed_ids": ["uuid1", "uuid2", "uuid3"],
  "hours_back": 24
}
```

**Response**:

```json
{
  "summary": "Comprehensive aggregated summary of all articles...",
  "feeds": [
    {
      "feed_id": "uuid1",
      "feed_title": "Tech News",
      "article_count": 5,
      "articles": [
        {
          "id": "article-uuid",
          "title": "Article Title",
          "url": "https://example.com/article",
          "published_at": "2025-01-15T10:30:00Z",
          "summary": "Individual article summary..."
        }
      ]
    }
  ],
  "total_articles": 15,
  "time_range_hours": 24
}
```

**Key Features**:

- Leverages existing individual article summaries for efficiency
- Default time constraint of 24 hours (configurable up to 1 week)
- AI-powered aggregation that identifies themes and trends
- Includes detailed breakdown by feed
- Optimal for daily news digest or feed overviews

### Bulk Fetch API

The bulk fetch feature allows retrieving articles from multiple feeds with advanced filtering:

```json
POST /api/articles/bulk-fetch
{
  "feed_ids": ["uuid1", "uuid2"],
  "start_date": "2025-01-01T00:00:00Z",
  "end_date": "2025-12-31T23:59:59Z",
  "limit": 50,
  "offset": 0
}
```

**Key Features**:

- Always fetches latest articles online first
- Supports date range filtering
- Provides pagination
- Returns feed summaries with article counts
- Handles multiple feeds efficiently

## 🏛️ Architecture

### Backend (Rust + Axum)

- **Fast & Safe**: Built with Rust for memory safety and performance
- **Async**: Tokio-based async runtime for high concurrency
- **Database**: PostgreSQL with SQLx for type-safe queries
- **Feed Parsing**: `feed-rs` for robust RSS/Atom parsing
- **HTTP Client**: `reqwest` for reliable feed fetching

### Frontend (React + TypeScript)

- **Modern Stack**: React 18 with TypeScript
- **Styling**: Tailwind CSS for responsive design
- **State Management**: React hooks and context
- **Build Tool**: Vite for fast development and builds
- **API Client**: Custom fetch-based client with proper error handling

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Commit changes: `git commit -m 'Add amazing feature'`
4. Push to branch: `git push origin feature/amazing-feature`
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🔗 Links

- [Bulk Fetch Feature Documentation](BULK_FETCH_FEATURE.md)
- [API Documentation](docs/api.md)
- [Contributing Guidelines](CONTRIBUTING.md)

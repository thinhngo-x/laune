# Laune 📰

A modern RSS reader and article summarization platform built with Rust and TypeScript. Laune fetches articles from RSS feeds and provides AI-powered summaries for quick content consumption.

## ✨ Features

- 🌐 **RSS Feed Management**: Add, manage, and organize RSS/Atom feeds
- 📄 **Bulk Article Fetching**: Retrieve articles from multiple feeds with date filtering
- 🤖 **AI Summarization**: Generate intelligent summaries using OpenAI GPT models
- 📱 **Responsive UI**: Modern React interface with Tailwind CSS
- ⚡ **Real-time Updates**: Always fetch latest articles from online sources
- 🔍 **Advanced Filtering**: Filter articles by feed, date range, and pagination
- 📊 **Feed Analytics**: View article counts and feed statistics

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

2. **Environment Configuration**:
   Create `backend/.env` (copy from `backend/.env.example`):

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
- `GET /api/articles` - List articles with filtering
- `POST /api/articles/bulk-fetch` - Bulk fetch articles from selected feeds
- `POST /api/summaries` - Generate article summaries

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

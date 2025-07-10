# Contributing to Laune

Thank you for your interest in contributing to Laune! This document provides guidelines and information for contributors.

## 🚀 Getting Started

1. **Fork the repository** on GitHub
2. **Clone your fork** locally:
   ```bash
   git clone https://github.com/your-username/laune.git
   cd laune
   ```
3. **Create a branch** for your feature or fix:
   ```bash
   git checkout -b feature/your-feature-name
   ```

## 🏗️ Development Setup

### Prerequisites
- Rust 1.70+
- Node.js 18+
- pnpm
- PostgreSQL 13+

### Environment Setup
1. Create `backend/.env` from `backend/.env.example`
2. Set up your PostgreSQL database
3. Install dependencies:
   ```bash
   cd backend && cargo build
   cd ../frontend && pnpm install
   ```

### Running the Application
```bash
# Quick start
./start.sh

# Or manually
cd backend && cargo run &
cd frontend && pnpm dev
```

## 🧪 Testing

### Backend Tests
```bash
cd backend
cargo test
```

### Frontend Tests
```bash
cd frontend
pnpm test
```

### Integration Tests
```bash
./test_bulk_fetch.sh
```

## 📝 Code Style

### Rust (Backend)
- Follow standard Rust conventions
- Use `cargo fmt` for formatting
- Run `cargo clippy` for linting
- Add documentation for public APIs

### TypeScript (Frontend)
- Use TypeScript strict mode
- Follow React best practices
- Use Tailwind CSS for styling
- Add JSDoc comments for complex functions

## 🔄 Pull Request Process

1. **Update your branch** with the latest main:
   ```bash
   git fetch origin
   git rebase origin/main
   ```

2. **Test thoroughly**:
   - Run all tests
   - Test the UI manually
   - Verify database migrations work

3. **Create descriptive commits**:
   ```bash
   git commit -m "feat: add bulk article fetching feature"
   git commit -m "fix: resolve pagination issue in articles API"
   ```

4. **Push and create PR**:
   ```bash
   git push origin feature/your-feature-name
   ```

5. **Fill out the PR template** with:
   - Description of changes
   - Screenshots (for UI changes)
   - Testing steps
   - Breaking changes (if any)

## 🐛 Bug Reports

When filing a bug report, please include:

- **Environment details** (OS, Rust version, Node.js version)
- **Steps to reproduce** the issue
- **Expected behavior** vs. actual behavior
- **Screenshots** or error logs
- **Browser details** (for frontend issues)

## 💡 Feature Requests

For new features:

- **Check existing issues** to avoid duplicates
- **Describe the use case** and motivation
- **Provide examples** of how it would work
- **Consider backward compatibility**

## 🏷️ Commit Convention

We use conventional commits:

- `feat:` - New features
- `fix:` - Bug fixes
- `docs:` - Documentation changes
- `style:` - Code style changes (formatting, etc.)
- `refactor:` - Code refactoring
- `test:` - Adding or updating tests
- `chore:` - Build processes, dependencies, etc.

## 📁 Project Structure

```
laune/
├── backend/                 # Rust backend
│   ├── src/
│   │   ├── routes/         # API endpoints
│   │   ├── models.rs       # Data models
│   │   ├── feeds.rs        # Feed fetching logic
│   │   └── ...
│   ├── migrations/         # Database migrations
│   └── tests/              # Backend tests
├── frontend/               # React frontend
│   ├── src/
│   │   ├── components/     # UI components
│   │   ├── pages/          # Page components
│   │   ├── api/            # API client
│   │   └── ...
│   └── tests/              # Frontend tests
├── shared/                 # Shared types
└── docs/                   # Documentation
```

## 🎯 Focus Areas

We're particularly interested in contributions for:

- **Performance improvements** (especially feed fetching)
- **UI/UX enhancements** 
- **Additional feed formats** (beyond RSS/Atom)
- **Summarization improvements** (better AI prompts)
- **Database optimizations**
- **Mobile responsiveness**
- **Accessibility improvements**
- **Documentation** and examples

## ❓ Questions?

- Open an issue for technical questions
- Join our discussions for general questions
- Check the documentation in the `docs/` folder

Thank you for contributing to Laune! 🙏

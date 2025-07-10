#!/bin/bash

# Pre-commit setup script for Laune project
# This script installs and configures pre-commit hooks

set -e

echo "🔧 Setting up pre-commit hooks for Laune..."

# Check if uvx is available (preferred method)
if command -v uvx &> /dev/null; then
    echo "✅ Using uvx to run pre-commit"
    PRE_COMMIT_CMD="uvx pre-commit"
elif command -v pre-commit &> /dev/null; then
    echo "✅ Using installed pre-commit"
    PRE_COMMIT_CMD="pre-commit"
else
    echo "📦 Installing pre-commit..."

    # Try different installation methods
    if command -v pip &> /dev/null; then
        pip install pre-commit
        PRE_COMMIT_CMD="pre-commit"
    elif command -v pip3 &> /dev/null; then
        pip3 install pre-commit
        PRE_COMMIT_CMD="pre-commit"
    elif command -v brew &> /dev/null; then
        brew install pre-commit
        PRE_COMMIT_CMD="pre-commit"
    else
        echo "❌ Could not install pre-commit. Please install uv first:"
        echo "   curl -LsSf https://astral.sh/uv/install.sh | sh"
        echo "   or"
        echo "   pip install pre-commit"
        exit 1
    fi
fi

echo "✅ pre-commit is available"

# Install the git hook scripts
echo "📋 Installing pre-commit hooks..."
$PRE_COMMIT_CMD install

# Install commit-msg hook for conventional commits (optional)
echo "📝 Installing commit-msg hook..."
$PRE_COMMIT_CMD install --hook-type commit-msg

# Run pre-commit on all files to test
echo "🧪 Running pre-commit on all files (initial setup)..."
$PRE_COMMIT_CMD run --all-files || {
    echo "⚠️  Some pre-commit checks failed. This is normal for the first run."
    echo "   The hooks will automatically fix many issues."
    echo "   Please review the changes and commit them if they look correct."
}

echo ""
echo "🎉 Pre-commit hooks are now set up!"
echo ""
echo "📋 Available hooks:"
echo "   - Essential file checks (trailing whitespace, JSON/YAML validation)"
echo "   - Rust formatting and linting (rustfmt, clippy)"
echo "   - Code formatting (Prettier for TS/JS/CSS/MD)"
echo "   - Security checks (detect private keys)"
echo ""
echo "💡 Tips:"
echo "   - Hooks run automatically before each commit"
echo "   - Run 'uvx pre-commit run --all-files' to check all files manually"
echo "   - Run 'uvx pre-commit autoupdate' to update hook versions"
echo "   - Skip hooks with 'git commit --no-verify' (not recommended)"
echo "   - Using uvx ensures you always have the latest pre-commit version"
echo ""

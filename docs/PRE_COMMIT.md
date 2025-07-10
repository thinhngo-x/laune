# Pre-commit Hooks Configuration

This project uses [pre-commit](https://pre-commit.com/) to ensure code quality and consistency
before commits are made to the repository.

## 🔧 Quick Setup

Run the setup script to install and configure pre-commit hooks:

```bash
./setup-precommit.sh
```

## 📋 Available Hooks

### Essential File Checks

- **Trailing whitespace removal** - Removes trailing spaces
- **End of file fixer** - Ensures files end with a newline
- **File format validation** - Validates YAML and JSON files
- **Merge conflict detection** - Prevents commits with merge conflict markers
- **Large file detection** - Prevents accidental commits of large files (>1MB)
- **Private key detection** - Prevents accidental commits of private keys

### Rust (Backend)

- **rustfmt** - Automatic code formatting
- **clippy** - Rust linter with warnings treated as errors

### Code Formatting

- **Prettier** - Code formatting for TypeScript, JavaScript, JSON, CSS, Markdown, YAML

- **YAML formatting** - Consistent YAML formatting with 2-space indentation

## 🎯 Configuration Files

| File                      | Purpose                           |
| ------------------------- | --------------------------------- |
| `.pre-commit-config.yaml` | Main pre-commit configuration     |
| `.secrets.baseline`       | Baseline for secret detection     |
| `.markdownlint.yaml`      | Markdown linting rules            |
| `.prettierrc.json`        | Prettier formatting configuration |
| `.prettierignore`         | Files to ignore for Prettier      |
| `.sqlfluff`               | SQL linting configuration         |
| `frontend/.eslintrc.json` | ESLint rules for TypeScript/React |

## 🚀 Usage

### Automatic (Recommended)

Pre-commit hooks run automatically before each commit:

```bash
git add .
git commit -m "feat: add new feature"
# Hooks run automatically and may modify files
# If hooks make changes, you'll need to add and commit again
```

### Manual Execution

Run hooks on all files:

```bash
uvx pre-commit run --all-files
```

Run a specific hook:

```bash
uvx pre-commit run prettier --all-files
uvx pre-commit run clippy --all-files
```

### Updating Hooks

Update to the latest versions:

```bash
uvx pre-commit autoupdate
```

## 🔧 Customization

### Skip Hooks (Not Recommended)

Skip all hooks for a commit:

```bash
git commit --no-verify -m "emergency fix"
```

Skip specific hooks:

```bash
SKIP=clippy,eslint git commit -m "skip linting"
```

### Adding New Hooks

Edit `.pre-commit-config.yaml` and add new hooks. Then update:

```bash
pre-commit install
```

## 🐛 Troubleshooting

### Hook Installation Issues

If hooks aren't working:

```bash
# Reinstall hooks
pre-commit uninstall
pre-commit install

# Clear cache and reinstall
rm -rf ~/.cache/pre-commit
pre-commit install
```

### Rust Hook Issues

Ensure you're in the project root and the backend directory exists:

```bash
# Check Rust formatting manually
cd backend && cargo fmt --check

# Check clippy manually
cd backend && cargo clippy -- -D warnings
```

### Frontend Hook Issues

Ensure frontend dependencies are installed:

```bash
cd frontend && pnpm install
```

### Secret Detection False Positives

If `detect-secrets` flags legitimate content:

1. Review the detection to ensure it's not actually a secret
2. Update the baseline file:

   ```bash
   detect-secrets scan --baseline .secrets.baseline
   ```

## 📝 Hook Behavior

### Auto-fixing Hooks

These hooks automatically fix issues:

- `prettier` - Formats code
- `trailing-whitespace` - Removes trailing spaces
- `end-of-file-fixer` - Adds final newline
- `sqlfluff-fix` - Fixes SQL formatting

### Validation Hooks

These hooks only report issues:

- `clippy` - Reports Rust warnings/errors
- `check-yaml` - Validates YAML syntax
- `detect-secrets` - Reports potential secrets

## ✅ Best Practices

1. **Let hooks fix what they can** - Many hooks auto-fix issues
2. **Review auto-fixes** - Always review changes made by hooks
3. **Don't skip hooks** - They catch issues early
4. **Keep configs updated** - Run `pre-commit autoupdate` regularly
5. **Test before pushing** - Run `pre-commit run --all-files` before important commits

## 🔗 Additional Resources

- [Pre-commit Documentation](https://pre-commit.com/)
- [Conventional Commits](https://www.conventionalcommits.org/)
- [Rust Clippy Lints](https://rust-lang.github.io/rust-clippy/master/)
- [ESLint Rules](https://eslint.org/docs/rules/)
- [Prettier Configuration](https://prettier.io/docs/en/configuration.html)

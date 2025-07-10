# Security Policy

## Supported Versions

We provide security updates for the following versions:

| Version | Supported          |
| ------- | ------------------ |
| 1.0.x   | :white_check_mark: |
| < 1.0   | :x:                |

## Reporting a Vulnerability

We take security vulnerabilities seriously. If you discover a security vulnerability, please follow
these steps:

### 🔒 Private Disclosure

**DO NOT** open a public issue for security vulnerabilities.

Instead, please report security issues by emailing us at: **security@laune.dev** (or create a
private GitHub security advisory)

### 📋 Information to Include

When reporting a security vulnerability, please include:

1. **Description** of the vulnerability
2. **Steps to reproduce** the issue
3. **Potential impact** and severity
4. **Suggested fixes** (if you have any)
5. **Your contact information** for follow-up

### 🕐 Response Timeline

- **Initial Response**: Within 24-48 hours
- **Status Update**: Within 7 days
- **Fix Timeline**: Depends on severity
  - Critical: Within 24-72 hours
  - High: Within 1-2 weeks
  - Medium/Low: Next regular release

### 🏆 Recognition

We appreciate security researchers who help keep Laune secure. If you report a valid security
vulnerability:

- We'll acknowledge your contribution (if you wish)
- We'll credit you in our security advisories (unless you prefer to remain anonymous)
- For significant findings, we may offer a small token of appreciation

## 🛡️ Security Best Practices

### For Developers

- **Dependencies**: Keep all dependencies up to date
- **Environment Variables**: Never commit secrets to the repository
- **Database**: Use parameterized queries to prevent SQL injection
- **Input Validation**: Validate and sanitize all user inputs
- **Authentication**: Implement proper session management
- **HTTPS**: Always use HTTPS in production

### For Users

- **Environment Files**: Keep your `.env` files secure and never share them
- **Database Access**: Use strong passwords and limit database access
- **API Keys**: Rotate API keys regularly and keep them confidential
- **Updates**: Keep Laune updated to the latest version

## 🚫 Out of Scope

The following are generally considered out of scope for security reports:

- Issues requiring physical access to a user's device
- Social engineering attacks
- Attacks requiring compromised credentials
- Rate limiting issues (unless they lead to DoS)
- Missing security headers that don't lead to actual vulnerabilities

## 📚 Security Resources

- [OWASP Top 10](https://owasp.org/www-project-top-ten/)
- [Rust Security Guidelines](https://doc.rust-lang.org/stable/book/ch09-03-to-panic-or-not-to-panic.html)
- [Node.js Security Best Practices](https://nodejs.org/en/docs/guides/security/)

Thank you for helping keep Laune secure! 🙏

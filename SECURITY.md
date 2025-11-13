# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

We take the security of BitVMX-Z seriously. If you discover a security vulnerability, please report it responsibly.

### How to Report

**DO NOT** open a public issue for security vulnerabilities.

Instead, please email: **security@bitvmx.org**

### What to Include

- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (if any)

### Response Timeline

- **Initial Response**: Within 48 hours
- **Status Update**: Within 7 days
- **Fix Timeline**: Depends on severity
  - Critical: 24-48 hours
  - High: 7 days
  - Medium: 30 days
  - Low: 90 days

### Disclosure Policy

- We will acknowledge your report within 48 hours
- We will provide a detailed response within 7 days
- We will work on a fix and keep you informed
- Once fixed, we will coordinate disclosure timing with you
- We will credit you in the security advisory (unless you prefer to remain anonymous)

## Security Best Practices

### For Users

- Always verify Docker image signatures
- Use the latest stable version
- Keep dependencies updated
- Never expose backend API directly to the internet without authentication
- Use environment variables for sensitive configuration

### For Developers

- Review code for common vulnerabilities
- Validate all inputs
- Use parameterized queries for database operations
- Keep dependencies updated
- Follow secure coding guidelines
- Run security scanners in CI

## Known Security Considerations

This is a research prototype. Do not use in production without:

1. Complete security audit
2. Formal verification of critical components
3. Penetration testing
4. Production-grade key management
5. Proper monitoring and alerting

## Bug Bounty

Currently, we do not have a formal bug bounty program. However, we deeply appreciate responsible disclosure and will credit researchers appropriately.

## Contact

For non-security issues, please use GitHub Issues.
For security concerns: security@bitvmx.org

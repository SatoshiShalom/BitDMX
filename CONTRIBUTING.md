# Contributing to BitVMX-Z

Thank you for your interest in contributing to BitVMX-Z!

## Development Process

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run tests (`./scripts/test.sh`)
5. Commit your changes (`git commit -m 'Add amazing feature'`)
6. Push to the branch (`git push origin feature/amazing-feature`)
7. Open a Pull Request

## Code Standards

### Rust

- Follow Rust standard formatting (`cargo fmt`)
- Pass all clippy lints (`cargo clippy`)
- Write tests for new functionality
- Document public APIs

### Frontend

- Follow ESLint rules
- Use TypeScript strictly
- Write meaningful component names
- Keep components focused and reusable

## Commit Messages

Use conventional commits:

- `feat:` - New features
- `fix:` - Bug fixes
- `docs:` - Documentation changes
- `test:` - Test additions/changes
- `refactor:` - Code refactoring
- `chore:` - Build/tooling changes

Example: `feat: add STARK proof caching`

## Pull Request Guidelines

- Keep PRs focused on a single feature/fix
- Update documentation as needed
- Add tests for new functionality
- Ensure CI passes
- Request review from maintainers

## Issue Labels

- `protocol` - Protocol layer (STARK, zkVM)
- `backend` - Backend/API issues
- `integration` - Taproot/bridge issues
- `frontend` - UI/UX issues
- `design` - Design/branding
- `research` - Research topics
- `bug` - Bug reports
- `enhancement` - Feature requests
- `good-first-issue` - Good for newcomers

## Testing

```bash
# Run all tests
./scripts/test.sh

# Rust only
cargo test --workspace

# Frontend only
cd frontend && npm test
```

## Questions?

Open an issue or reach out to the maintainers.

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

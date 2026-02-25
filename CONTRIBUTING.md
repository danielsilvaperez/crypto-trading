# Contributing to Crypto Trading Toolkit

Thank you for your interest in contributing! This document provides guidelines for contributing to the project.

## 🎯 Contribution Areas

We welcome contributions in:

- **🐛 Bug fixes**: Fix issues in existing code
- **✨ Features**: Add new functionality to packages
- **📚 Documentation**: Improve docs, examples, tutorials
- **🧪 Tests**: Add test coverage
- **🔧 Tools**: Build new CLI utilities

## 🚀 Getting Started

### Development Setup

```bash
# 1. Fork the repository on GitHub

# 2. Clone your fork
git clone https://github.com/YOUR_USERNAME/crypto-trading.git
cd crypto-trading

# 3. Set up upstream remote
git remote add upstream https://github.com/original/crypto-trading.git

# 4. Build and test
cargo build --workspace
pytest
```

### Project Structure

```
packages/
├── telegram-control/     # Telegram bot framework
├── blockchain-clients/   # Blockchain clients
└── risk-management/      # Risk controls

apps/
└── [trading bots]

tools/
└── [CLI utilities]
```

## 📝 Code Style

### Rust

- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Run `cargo fmt` and `cargo clippy` before committing
- Document all public APIs with doc comments
- Add tests for new functionality

```bash
cargo fmt
cargo clippy --workspace
cargo test --workspace
```

### Python

- Follow [PEP 8](https://pep8.org/)
- Use `black` for formatting, `isort` for imports
- Type hints required for function signatures
- Docstrings in Google style

```bash
black packages/ apps/ tools/
isort packages/ apps/ tools/
mypy packages/ apps/ tools/
pytest
```

### TypeScript

- Use strict TypeScript configuration
- Prefer interfaces over types
- Document exported functions

```bash
pnpm lint
pnpm format
pnpm test
```

## 🔧 Making Changes

### 1. Create a Branch

```bash
git checkout -b feature/description
# or
git checkout -b fix/description
```

Branch naming:
- `feature/` - New features
- `fix/` - Bug fixes
- `docs/` - Documentation
- `refactor/` - Code refactoring
- `test/` - Test additions

### 2. Write Code

- Keep changes focused and atomic
- Add tests for new functionality
- Update documentation
- Follow existing patterns

### 3. Commit Messages

Use [Conventional Commits](https://www.conventionalcommits.org/):

```
feat: add new alert type to telegram-control
fix: handle rate limiting in polymarket client
docs: update API reference for risk-management
test: add circuit breaker tests
refactor: simplify position sizing logic
```

Types:
- `feat:` New feature
- `fix:` Bug fix
- `docs:` Documentation changes
- `style:` Formatting, semicolons, etc.
- `refactor:` Code restructuring
- `test:` Test additions/changes
- `chore:` Build process, dependencies

### 4. Submit Pull Request

1. Push to your fork
2. Open PR against `main` branch
3. Fill out PR template
4. Request review from maintainers

## 🧪 Testing

### Rust Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_feature() {
        // Arrange
        let input = ...;
        
        // Act
        let result = function(input);
        
        // Assert
        assert_eq!(result, expected);
    }
    
    #[tokio::test]
    async fn test_async_feature() {
        // async test
    }
}
```

### Python Tests

```python
import pytest

@pytest.fixture
def setup():
    return create_test_instance()

def test_feature(setup):
    # Arrange
    instance = setup
    
    # Act
    result = instance.method()
    
    # Assert
    assert result == expected

@pytest.mark.asyncio
async def test_async_feature():
    result = await async_function()
    assert result == expected
```

## 📦 Adding a New Package

### Rust Package

1. Create directory: `packages/new-package/rust`
2. Add `Cargo.toml`:

```toml
[package]
name = "new-package"
version.workspace = true
edition.workspace = true
license.workspace = true

[dependencies]
# your deps
```

3. Add to workspace `Cargo.toml`

### Python Package

1. Create directory: `packages/new-package/python`
2. Add `pyproject.toml` with proper structure
3. Follow existing package structure

## 🔍 Code Review Process

1. **Automated checks** must pass (CI/CD)
2. **At least one** maintainer approval required
3. **All comments** must be resolved
4. **Documentation** must be updated

## 🐛 Reporting Bugs

Use GitHub Issues with template:

```markdown
**Description**
Clear description of the bug

**To Reproduce**
Steps to reproduce

**Expected Behavior**
What should happen

**Environment**
- OS: [e.g. Ubuntu 22.04]
- Rust/Python version
- Package version

**Additional Context**
Logs, screenshots, etc.
```

## 💡 Feature Requests

Use GitHub Discussions for:
- New package ideas
- API design discussions
- Architecture changes

## 🏷️ Versioning

We follow [Semantic Versioning](https://semver.org/):

- `MAJOR`: Breaking API changes
- `MINOR`: New features (backward compatible)
- `PATCH`: Bug fixes

## 📜 License

By contributing, you agree that your contributions will be licensed under the MIT License.

## 🙏 Questions?

- Open a [GitHub Discussion](https://github.com/yourusername/crypto-trading/discussions)
- Join our community [Discord/Telegram]

Thank you for contributing! 🎉

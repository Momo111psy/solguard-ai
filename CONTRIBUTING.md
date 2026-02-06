# Contributing to SolGuard AI

First off, thank you for considering contributing to SolGuard AI! It's people like you that make SolGuard AI such a great tool for the Solana ecosystem.

## Code of Conduct

This project and everyone participating in it is governed by our [Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code.

## How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check the existing issues as you might find out that you don't need to create one. When you are creating a bug report, please include as many details as possible:

* **Use a clear and descriptive title**
* **Describe the exact steps which reproduce the problem**
* **Provide specific examples to demonstrate the steps**
* **Describe the behavior you observed after following the steps**
* **Explain which behavior you expected to see instead and why**
* **Include screenshots and animated GIFs if possible**

### Suggesting Enhancements

Enhancement suggestions are tracked as GitHub issues. When creating an enhancement suggestion, please include:

* **Use a clear and descriptive title**
* **Provide a step-by-step description of the suggested enhancement**
* **Provide specific examples to demonstrate the steps**
* **Describe the current behavior and explain which behavior you expected to see instead**
* **Explain why this enhancement would be useful**

### Pull Requests

* Fill in the required template
* Do not include issue numbers in the PR title
* Follow the Rust and Python style guides
* Include thoughtfully-worded, well-structured tests
* Document new code
* End all files with a newline

## Development Process

### Setting Up Your Development Environment

1. Fork the repo
2. Clone your fork: `git clone https://github.com/YOUR_USERNAME/solguard-ai.git`
3. Add upstream remote: `git remote add upstream https://github.com/Momo111psy/solguard-ai.git`
4. Install dependencies:
   ```bash
   # Install Rust and Anchor
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
   avm install latest
   avm use latest
   
   # Install Python dependencies
   pip3 install -r requirements.txt
   ```

### Making Changes

1. Create a new branch: `git checkout -b feature/your-feature-name`
2. Make your changes
3. Write or update tests as needed
4. Run tests: `anchor test`
5. Commit your changes: `git commit -m "Add some feature"`
6. Push to your fork: `git push origin feature/your-feature-name`
7. Open a Pull Request

### Coding Standards

#### Rust/Anchor

* Follow the [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/)
* Use `cargo fmt` before committing
* Run `cargo clippy` and address warnings
* Add doc comments for public APIs
* Write unit tests for new functionality

#### Python

* Follow [PEP 8](https://www.python.org/dev/peps/pep-0008/)
* Use type hints where appropriate
* Add docstrings to functions and classes
* Write unit tests using pytest

### Commit Messages

* Use the present tense ("Add feature" not "Added feature")
* Use the imperative mood ("Move cursor to..." not "Moves cursor to...")
* Limit the first line to 72 characters or less
* Reference issues and pull requests liberally after the first line

Example:
```
Add quantum-resistant signature verification

- Implement CRYSTALS-Dilithium algorithm
- Add tests for signature generation and verification
- Update documentation with usage examples

Closes #123
```

## Project Structure

```
solguard-ai/
├── programs/          # Solana programs (Rust/Anchor)
│   ├── security-oracle/
│   ├── validator-registry/
│   ├── transparency-vault/
│   ├── governance-module/
│   └── solguard-token/
├── app/              # AI systems (Python)
│   ├── ai_detector.py
│   └── economic_oracle.py
├── tests/            # Integration tests
├── docs/             # Documentation
└── scripts/          # Utility scripts
```

## Testing

### Running Tests

```bash
# Run all tests
anchor test

# Run specific test
anchor test --test test_name

# Run Python tests
pytest app/tests/
```

### Writing Tests

* Write tests for all new features
* Ensure tests are deterministic
* Use descriptive test names
* Test edge cases and error conditions

## Documentation

* Update README.md if you change functionality
* Add inline comments for complex logic
* Update ARCHITECTURE.md for structural changes
* Keep ROADMAP.md current with development progress

## Security

* Never commit secrets or private keys
* Report security vulnerabilities privately (see SECURITY.md)
* Follow secure coding practices
* Use the latest stable versions of dependencies

## Community

* Join our [Discord](https://discord.gg/solguard-ai) (coming soon)
* Follow us on [Twitter](https://x.com/beyondtheframe7)
* Check our [Roadmap](ROADMAP.md) for planned features

## Recognition

Contributors will be recognized in:
* README.md contributors section
* Release notes
* Project documentation

## Questions?

Feel free to open an issue with the "question" label or reach out on our community channels.

Thank you for contributing to SolGuard AI! 🚀

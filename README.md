# SolGuard AI: Security Monitoring Protocol for Solana

![Solana](https://img.shields.io/badge/Solana-9945FF?style=for-the-badge&logo=solana&logoColor=white)
![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![Python](https://img.shields.io/badge/Python-3776AB?style=for-the-badge&logo=python&logoColor=white)
![Anchor](https://img.shields.io/badge/Anchor-Framework-purple?style=for-the-badge)
![License](https://img.shields.io/badge/License-MIT-green?style=for-the-badge)
![Status](https://img.shields.io/badge/Status-Test_Ready-yellow?style=for-the-badge)

> An experimental AI-assisted security monitoring and validator health protocol for the Solana ecosystem. Currently in early prototype stage.

---

## Overview

SolGuard AI is a research project exploring how AI-powered monitoring can improve security and validator health on Solana. The project addresses three ecosystem challenges:

1. **Validator Centralization**: Solana has experienced significant validator attrition, threatening network decentralization.
2. **Security Gaps**: Billions lost in DeFi exploits across chains, with limited predictive tooling available.
3. **Transparency**: Many Solana programs lack verifiable IDLs, creating trust issues for users.

This project is in **early development** and is not production-ready. The smart contracts are prototypes demonstrating the architecture and are not audited.

---

## Architecture

The protocol consists of four Anchor programs:

```
programs/
├── security-oracle/       # Threat scoring and AI model integration points
│   ├── lib.rs             # Core oracle logic
│   ├── privacy_protocol.rs # Privacy-preserving verification (experimental)
│   └── quantum_resistant.rs # Post-quantum signature exploration
├── validator-registry/    # Validator health tracking and incentives
├── transparency-vault/    # IDL storage and verification registry
└── governance-module/     # Community governance for parameter updates
```

### Smart Contract Summary

| Program | Lines of Code | Description |
|---------|--------------|-------------|
| Security Oracle | ~930 LOC | Threat detection scoring, AI integration points |
| Validator Registry | ~490 LOC | Validator performance tracking |
| Transparency Vault | ~330 LOC | IDL and audit report storage |
| Governance Module | ~330 LOC | Proposal and voting system |

**Total Rust LOC:** ~2,080 (excluding Python AI components)

### Off-Chain Components

| Component | Language | Description |
|-----------|----------|-------------|
| AI Detector | Python | ML-based vulnerability pattern detection |
| Economic Oracle | Python | Validator economics modeling |

---

## Current Status

This project is an **early-stage prototype**. Here is an honest assessment of where things stand:

| Component | Status | Notes |
|-----------|--------|-------|
| Smart contract architecture | Complete | Designed and implemented |
| Core program logic | Prototype | Compiles but needs thorough testing |
| AI detection models | Experimental | Basic pattern matching, not production ML |
| Testing suite | Complete | 95+ comprehensive test cases covering all programs |
| Devnet deployment | Not deployed | Pending testing completion |
| Security audit | Not started | Required before any mainnet consideration |
| Frontend | Not started | Planned for future phase |

---

## Development Setup

### Prerequisites

- Rust 1.75+
- Solana CLI 1.18+
- Anchor 0.30+
- Python 3.10+ (for AI components)
- Node.js 18+

### Build

```bash
git clone https://github.com/Momo111psy/solguard-ai.git
cd solguard-ai
anchor build
```

### Test

```bash
anchor test
```

---

## Roadmap

| Phase | Goal | Timeline | Status |
|-------|------|----------|--------|
| 1 | Architecture design and prototype programs | Q1 2026 | ✅ Done |
| 2 | Comprehensive test suite and bug fixes | Q1-Q2 2026 | ✅ Done |
| 3 | Devnet deployment and integration testing | Q2 2026 | Planned |
| 4 | AI model training with real Solana data | Q2-Q3 2026 | Planned |
| 5 | Security audit | Q3 2026 | Planned |
| 6 | Mainnet beta launch | Q4 2026 | Planned |

---

## Related Projects

- [Solana Repository Security Scanner](https://github.com/Momo111psy/solana-repo-scanner) — A companion tool for analyzing GitHub repositories in the Solana ecosystem for red flags and scam indicators.
- [MOLTVAULT](https://github.com/Momo111psy/MOLTVAULT) — Multi-signature vault protocol for Solana DAOs and treasuries.

---

## Contributing

This project is open source and contributions are welcome. Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines. Given the early stage, the most valuable contributions right now are:

- Writing tests for the existing programs
- Reviewing the smart contract logic for vulnerabilities
- Improving the AI detection models
- Documentation improvements

---

## Disclaimer

This software is experimental and has not been audited. Do not use it in production or with real funds. The AI components are proof-of-concept and should not be relied upon for actual security decisions.

---

## License

MIT License — see [LICENSE](./LICENSE) for details.

---

## Contact

**Rashik Thapa** — Security researcher and Solana ecosystem builder

- GitHub: [@Momo111psy](https://github.com/Momo111psy)
- Twitter: [@beyondtheframe7](https://x.com/beyondtheframe7)

---

## Documentation

- [Whitepaper](./docs/WHITEPAPER.md)
- [Architecture](./ARCHITECTURE.md)
- [Roadmap](./ROADMAP.md)
- [Security Policy](./SECURITY.md)

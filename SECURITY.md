# Security Policy

## Supported Versions

We release patches for security vulnerabilities. Currently supported versions:

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

The SolGuard AI team takes security bugs seriously. We appreciate your efforts to responsibly disclose your findings.

### Please Do NOT:

* Open a public GitHub issue for security vulnerabilities
* Discuss the vulnerability in public forums, social media, or mailing lists
* Attempt to exploit the vulnerability beyond the minimum necessary to demonstrate it

### Please DO:

1. **Email us directly** at rashikthapa80@gmail.com with:
   * A clear description of the vulnerability
   * Steps to reproduce the issue
   * Potential impact assessment
   * Any suggested fixes (if available)

2. **Use our PGP key** (coming soon) for sensitive communications

3. **Give us reasonable time** to respond and fix the issue before public disclosure (we aim for 90 days)

### What to Expect:

* **Within 24 hours**: Acknowledgment of your report
* **Within 7 days**: Initial assessment and severity classification
* **Within 30 days**: Fix development and testing (for critical issues)
* **Within 90 days**: Public disclosure (coordinated with you)

## Security Measures

### Smart Contract Security

* **Audits**: We are seeking professional security audits from Oak Security, Zellic, or Quantstamp
* **Testing**: Comprehensive unit and integration tests
* **Formal Verification**: Planned for critical components
* **Bug Bounty**: Coming soon after mainnet launch

### Cryptographic Security

* **Quantum Resistance**: Implementation of CRYSTALS-Dilithium and CRYSTALS-Kyber
* **Privacy**: Zero-knowledge proofs, ring signatures, and stealth addresses
* **Randomness**: Secure random number generation using Solana's native RNG

### Operational Security

* **Key Management**: Multi-signature requirements for critical operations
* **Access Control**: Role-based permissions with time-locks
* **Monitoring**: Real-time threat detection via AI Security Oracle
* **Incident Response**: Documented procedures for security incidents

## Vulnerability Disclosure Policy

### Severity Levels:

**Critical** (CVSS 9.0-10.0)
* Loss of funds
* Unauthorized access to private keys
* Complete system compromise

**High** (CVSS 7.0-8.9)
* Partial loss of funds
* Unauthorized state changes
* Denial of service

**Medium** (CVSS 4.0-6.9)
* Information disclosure
* Limited denial of service
* Minor logic errors

**Low** (CVSS 0.1-3.9)
* Cosmetic issues
* Documentation errors
* Minor UX problems

### Disclosure Timeline:

* **Critical**: Immediate private disclosure, public disclosure after fix deployment
* **High**: 30-day embargo before public disclosure
* **Medium**: 60-day embargo before public disclosure
* **Low**: 90-day embargo before public disclosure

## Security Best Practices for Users

### For Developers Integrating SolGuard AI:

1. **Always use the latest version** of SolGuard AI programs
2. **Verify program IDs** before integration
3. **Test thoroughly** on devnet before mainnet deployment
4. **Monitor for security updates** via our GitHub and Twitter
5. **Follow secure coding practices** as outlined in our documentation

### For Validators:

1. **Secure your validator keys** using hardware security modules (HSMs)
2. **Keep software updated** to the latest stable version
3. **Monitor validator performance** using our dashboard
4. **Report suspicious activity** immediately
5. **Follow our validator security guide** (coming soon)

### For Token Holders:

1. **Use hardware wallets** for significant holdings
2. **Verify transaction details** before signing
3. **Be cautious of phishing** attempts
4. **Never share your private keys** or seed phrases
5. **Enable multi-signature** for large transactions

## Known Issues and Limitations

### Current Development Status:

* **Pre-Audit**: Code has not yet undergone professional security audit
* **Testnet Only**: Currently deployed only on devnet/testnet
* **Active Development**: Features and APIs may change

### Planned Security Enhancements:

* Professional security audit (Q2 2026)
* Formal verification of critical components (Q3 2026)
* Bug bounty program launch (Q3 2026)
* Security incident response team (Q3 2026)

## Security Contacts

* **Primary**: rashikthapa80@gmail.com
* **Telegram**: @mrmomo1111
* **Twitter**: @beyondtheframe7

## Acknowledgments

We would like to thank the following security researchers and organizations for their contributions to SolGuard AI's security:

* (List will be updated as contributions are made)

## Bug Bounty Program

A bug bounty program will be launched after our mainnet deployment and professional security audit. Details will be announced on our website and social media channels.

### Planned Rewards:

* **Critical**: Up to $50,000 USDC
* **High**: Up to $10,000 USDC
* **Medium**: Up to $2,500 USDC
* **Low**: Up to $500 USDC

## Security Resources

* [Solana Security Best Practices](https://docs.solana.com/developing/programming-model/security-best-practices)
* [Anchor Security Guidelines](https://www.anchor-lang.com/docs/security)
* [OWASP Smart Contract Security](https://owasp.org/www-project-smart-contract-top-10/)

## Updates

This security policy is subject to change. Last updated: February 7, 2026

---

**Remember: Security is everyone's responsibility. If you see something, say something.** 🔒

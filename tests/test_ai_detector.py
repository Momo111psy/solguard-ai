"""
Test suite for the SolGuard AI Detector module.

Tests vulnerability pattern detection, threat scoring,
and classification accuracy for Solana smart contracts.
"""

import unittest
import sys
import os

# Add parent directory to path for imports
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', 'app'))

from ai_detector import (
    VulnerabilityDetector,
    ThreatClassifier,
    PatternMatcher,
)


class TestPatternMatcher(unittest.TestCase):
    """Tests for the static pattern matching engine."""

    def setUp(self):
        self.matcher = PatternMatcher()

    def test_detects_unchecked_arithmetic(self):
        """Unchecked arithmetic operations should be flagged."""
        code = """
        let result = balance + amount;
        """
        findings = self.matcher.scan(code, language="rust")
        self.assertTrue(
            any(f.category == "unchecked_arithmetic" for f in findings),
            "Should detect unchecked arithmetic"
        )

    def test_detects_missing_signer_check(self):
        """Missing signer verification should be flagged as critical."""
        code = """
        pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
            let vault = &mut ctx.accounts.vault;
            // No signer check before transfer
            transfer(vault, amount)?;
            Ok(())
        }
        """
        findings = self.matcher.scan(code, language="rust")
        self.assertTrue(
            any(f.severity == "critical" for f in findings),
            "Missing signer check should be critical severity"
        )

    def test_detects_reentrancy_pattern(self):
        """State changes after external calls should be flagged."""
        code = """
        invoke(&transfer_ix, &[vault.to_account_info()])?;
        vault.balance -= amount;
        """
        findings = self.matcher.scan(code, language="rust")
        self.assertTrue(
            any(f.category == "reentrancy" for f in findings),
            "Should detect potential reentrancy"
        )

    def test_clean_code_returns_no_findings(self):
        """Well-written code should return minimal or no findings."""
        code = """
        pub fn safe_transfer(ctx: Context<SafeTransfer>, amount: u64) -> Result<()> {
            require!(ctx.accounts.authority.is_signer, ErrorCode::Unauthorized);
            let vault = &mut ctx.accounts.vault;
            vault.balance = vault.balance.checked_sub(amount)
                .ok_or(ErrorCode::InsufficientFunds)?;
            // Transfer after state update
            transfer_tokens(ctx, amount)?;
            Ok(())
        }
        """
        findings = self.matcher.scan(code, language="rust")
        critical = [f for f in findings if f.severity == "critical"]
        self.assertEqual(len(critical), 0, "Clean code should have no critical findings")

    def test_detects_hardcoded_keys(self):
        """Hardcoded public keys should be flagged as suspicious."""
        code = """
        let admin = Pubkey::from_str("HN7cABqLq46Es1jh92dQQisAq662SmxELLLsHHe4YWrH").unwrap();
        """
        findings = self.matcher.scan(code, language="rust")
        self.assertTrue(
            any(f.category == "hardcoded_key" for f in findings),
            "Should detect hardcoded public key"
        )

    def test_handles_empty_input(self):
        """Empty input should return empty findings without error."""
        findings = self.matcher.scan("", language="rust")
        self.assertEqual(len(findings), 0)

    def test_handles_non_rust_language(self):
        """Non-Rust code should still be processed without errors."""
        code = "console.log('hello world');"
        findings = self.matcher.scan(code, language="javascript")
        self.assertIsInstance(findings, list)


class TestThreatClassifier(unittest.TestCase):
    """Tests for the ML-based threat classification system."""

    def setUp(self):
        self.classifier = ThreatClassifier()

    def test_classifies_high_risk_contract(self):
        """Known malicious patterns should classify as high risk."""
        features = {
            "unchecked_transfers": 5,
            "missing_signer_checks": 3,
            "admin_functions_exposed": True,
            "proxy_pattern": True,
            "age_days": 2,
        }
        score = self.classifier.classify(features)
        self.assertGreaterEqual(score, 70, "High-risk contract should score >= 70")

    def test_classifies_low_risk_contract(self):
        """Well-audited patterns should classify as low risk."""
        features = {
            "unchecked_transfers": 0,
            "missing_signer_checks": 0,
            "admin_functions_exposed": False,
            "proxy_pattern": False,
            "age_days": 365,
            "has_audit": True,
        }
        score = self.classifier.classify(features)
        self.assertLessEqual(score, 30, "Low-risk contract should score <= 30")

    def test_score_within_valid_range(self):
        """Threat score must always be between 0 and 100."""
        features = {"unchecked_transfers": 100, "missing_signer_checks": 100}
        score = self.classifier.classify(features)
        self.assertGreaterEqual(score, 0)
        self.assertLessEqual(score, 100)

    def test_handles_missing_features(self):
        """Classifier should handle incomplete feature sets gracefully."""
        features = {"age_days": 30}
        score = self.classifier.classify(features)
        self.assertIsInstance(score, (int, float))


class TestVulnerabilityDetector(unittest.TestCase):
    """Integration tests for the full vulnerability detection pipeline."""

    def setUp(self):
        self.detector = VulnerabilityDetector()

    def test_full_scan_returns_report(self):
        """Full scan should return a structured report."""
        code = """
        pub fn process(ctx: Context<Process>) -> Result<()> {
            let data = &mut ctx.accounts.data;
            data.value += 1;
            Ok(())
        }
        """
        report = self.detector.full_scan(code)
        self.assertIn("findings", report)
        self.assertIn("threat_score", report)
        self.assertIn("summary", report)

    def test_report_includes_recommendations(self):
        """Report should include actionable recommendations."""
        code = """
        pub fn unsafe_withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
            transfer(ctx.accounts.vault, amount)?;
            Ok(())
        }
        """
        report = self.detector.full_scan(code)
        self.assertIn("recommendations", report)
        self.assertGreater(len(report["recommendations"]), 0)

    def test_scan_performance(self):
        """Scan should complete within 5 seconds for typical contracts."""
        import time
        code = "pub fn noop() -> Result<()> { Ok(()) }\n" * 100
        start = time.time()
        self.detector.full_scan(code)
        elapsed = time.time() - start
        self.assertLess(elapsed, 5.0, "Scan should complete within 5 seconds")


if __name__ == "__main__":
    unittest.main()

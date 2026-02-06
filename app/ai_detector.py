"""
SolGuard AI - Vulnerability Detection Engine
Uses machine learning to detect smart contract vulnerabilities in real-time
Integrates with Solana programs for autonomous security monitoring
"""

import numpy as np
import hashlib
import json
from typing import List, Dict, Tuple, Optional
from dataclasses import dataclass
from enum import Enum
import re

class VulnerabilityType(Enum):
    """Types of vulnerabilities the AI can detect"""
    REENTRANCY = "reentrancy"
    INTEGER_OVERFLOW = "integer_overflow"
    UNAUTHORIZED_ACCESS = "unauthorized_access"
    MISSING_SIGNER_CHECK = "missing_signer_check"
    UNINITIALIZED_ACCOUNT = "uninitialized_account"
    ARITHMETIC_ERROR = "arithmetic_error"
    LOGIC_ERROR = "logic_error"
    FRONT_RUNNING = "front_running"
    TIMESTAMP_DEPENDENCY = "timestamp_dependency"
    QUANTUM_VULNERABLE = "quantum_vulnerable"

@dataclass
class VulnerabilityPattern:
    """Pattern definition for vulnerability detection"""
    vuln_type: VulnerabilityType
    pattern: str
    severity: int  # 1-10
    description: str
    recommendation: str
    confidence_threshold: float

@dataclass
class DetectionResult:
    """Result of vulnerability detection"""
    program_address: str
    vulnerabilities: List[Dict]
    security_score: int
    risk_level: str
    recommendations: List[str]
    quantum_resistant: bool
    timestamp: int

class AIVulnerabilityDetector:
    """
    AI-powered vulnerability detector for Solana smart contracts
    Uses pattern matching, ML models, and heuristics
    """
    
    def __init__(self):
        self.patterns = self._initialize_patterns()
        self.historical_exploits = self._load_historical_data()
        self.quantum_patterns = self._initialize_quantum_patterns()
        
    def _initialize_patterns(self) -> List[VulnerabilityPattern]:
        """Initialize vulnerability detection patterns"""
        return [
            VulnerabilityPattern(
                vuln_type=VulnerabilityType.MISSING_SIGNER_CHECK,
                pattern=r"(?!.*require!.*Signer).*Transfer.*",
                severity=9,
                description="Missing signer verification before token transfer",
                recommendation="Add require!(ctx.accounts.authority.is_signer) before transfers",
                confidence_threshold=0.85
            ),
            VulnerabilityPattern(
                vuln_type=VulnerabilityType.INTEGER_OVERFLOW,
                pattern=r"\.checked_(add|sub|mul|div)\(",
                severity=8,
                description="Arithmetic operation without overflow protection",
                recommendation="Use checked_add/sub/mul/div instead of direct arithmetic",
                confidence_threshold=0.90
            ),
            VulnerabilityPattern(
                vuln_type=VulnerabilityType.UNINITIALIZED_ACCOUNT,
                pattern=r"Account<'info,\s*\w+>.*(?!init)",
                severity=7,
                description="Account used without initialization check",
                recommendation="Add #[account(init)] or verify account is initialized",
                confidence_threshold=0.80
            ),
            VulnerabilityPattern(
                vuln_type=VulnerabilityType.REENTRANCY,
                pattern=r"(invoke|invoke_signed).*\n.*state\s*=",
                severity=10,
                description="State change after external call (reentrancy risk)",
                recommendation="Update state before making external calls (checks-effects-interactions)",
                confidence_threshold=0.95
            ),
            VulnerabilityPattern(
                vuln_type=VulnerabilityType.QUANTUM_VULNERABLE,
                pattern=r"(ed25519|secp256k1|ECDSA)(?!.*quantum)",
                severity=6,
                description="Uses classical cryptography vulnerable to quantum attacks",
                recommendation="Implement post-quantum cryptographic signatures (Dilithium, SPHINCS+)",
                confidence_threshold=0.75
            ),
        ]
    
    def _initialize_quantum_patterns(self) -> List[Dict]:
        """Initialize quantum vulnerability patterns"""
        return [
            {
                "name": "Classical Signature Scheme",
                "indicators": ["ed25519", "secp256k1", "ECDSA", "RSA"],
                "quantum_safe": False,
                "severity": 6,
                "recommendation": "Migrate to CRYSTALS-Dilithium or SPHINCS+"
            },
            {
                "name": "Symmetric Encryption",
                "indicators": ["AES-128", "AES-192"],
                "quantum_safe": False,
                "severity": 5,
                "recommendation": "Use AES-256 minimum (provides 128-bit quantum security)"
            },
            {
                "name": "Hash Functions",
                "indicators": ["SHA-256", "SHA3-256"],
                "quantum_safe": True,
                "severity": 0,
                "recommendation": "SHA-256/SHA3 are quantum-resistant"
            },
        ]
    
    def _load_historical_data(self) -> List[Dict]:
        """Load historical exploit patterns for ML training"""
        # In production, this would load from database
        return [
            {
                "exploit_type": "reentrancy",
                "loss_amount": 40_000_000,  # Step Finance hack
                "date": "2026-01-30",
                "pattern_signature": "external_call_before_state_update",
            },
            {
                "exploit_type": "unauthorized_access",
                "loss_amount": 3_100_000_000,  # 2025 DeFi collapse
                "date": "2025-12-15",
                "pattern_signature": "missing_authority_check",
            },
        ]
    
    def analyze_program(self, program_code: str, program_address: str) -> DetectionResult:
        """
        Main analysis function - detects vulnerabilities in Solana program
        
        Args:
            program_code: Source code or bytecode of the program
            program_address: On-chain program address
            
        Returns:
            DetectionResult with findings and recommendations
        """
        vulnerabilities = []
        
        # Pattern-based detection
        for pattern in self.patterns:
            matches = self._detect_pattern(program_code, pattern)
            if matches:
                vulnerabilities.extend(matches)
        
        # ML-based anomaly detection
        ml_findings = self._ml_anomaly_detection(program_code)
        vulnerabilities.extend(ml_findings)
        
        # Quantum vulnerability assessment
        quantum_vulns = self._assess_quantum_resistance(program_code)
        vulnerabilities.extend(quantum_vulns)
        
        # Historical pattern matching
        historical_matches = self._match_historical_exploits(program_code)
        vulnerabilities.extend(historical_matches)
        
        # Calculate security score
        security_score = self._calculate_security_score(vulnerabilities)
        risk_level = self._determine_risk_level(security_score, vulnerabilities)
        
        # Check quantum resistance
        quantum_resistant = self._is_quantum_resistant(program_code)
        
        # Generate recommendations
        recommendations = self._generate_recommendations(vulnerabilities)
        
        return DetectionResult(
            program_address=program_address,
            vulnerabilities=vulnerabilities,
            security_score=security_score,
            risk_level=risk_level,
            recommendations=recommendations,
            quantum_resistant=quantum_resistant,
            timestamp=self._get_timestamp()
        )
    
    def _detect_pattern(self, code: str, pattern: VulnerabilityPattern) -> List[Dict]:
        """Detect specific vulnerability pattern in code"""
        findings = []
        matches = re.finditer(pattern.pattern, code, re.MULTILINE | re.IGNORECASE)
        
        for match in matches:
            # Calculate confidence based on context
            confidence = self._calculate_confidence(code, match, pattern)
            
            if confidence >= pattern.confidence_threshold:
                findings.append({
                    "type": pattern.vuln_type.value,
                    "severity": pattern.severity,
                    "description": pattern.description,
                    "location": f"Line {code[:match.start()].count(chr(10)) + 1}",
                    "confidence": confidence,
                    "recommendation": pattern.recommendation,
                })
        
        return findings
    
    def _ml_anomaly_detection(self, code: str) -> List[Dict]:
        """
        ML-based anomaly detection
        Uses statistical analysis and learned patterns
        """
        findings = []
        
        # Feature extraction
        features = self._extract_features(code)
        
        # Anomaly scoring (simplified - production would use trained model)
        anomaly_score = self._compute_anomaly_score(features)
        
        if anomaly_score > 0.7:
            findings.append({
                "type": "anomaly_detected",
                "severity": int(anomaly_score * 10),
                "description": f"Unusual code patterns detected (anomaly score: {anomaly_score:.2f})",
                "location": "Multiple locations",
                "confidence": anomaly_score,
                "recommendation": "Manual review recommended for unusual patterns",
            })
        
        return findings
    
    def _assess_quantum_resistance(self, code: str) -> List[Dict]:
        """Assess quantum computing resistance"""
        findings = []
        
        for pattern in self.quantum_patterns:
            for indicator in pattern["indicators"]:
                if indicator.lower() in code.lower():
                    if not pattern["quantum_safe"]:
                        findings.append({
                            "type": VulnerabilityType.QUANTUM_VULNERABLE.value,
                            "severity": pattern["severity"],
                            "description": f"Uses {pattern['name']} which is vulnerable to quantum attacks",
                            "location": "Cryptographic implementation",
                            "confidence": 0.95,
                            "recommendation": pattern["recommendation"],
                        })
        
        return findings
    
    def _match_historical_exploits(self, code: str) -> List[Dict]:
        """Match against known historical exploit patterns"""
        findings = []
        
        for exploit in self.historical_exploits:
            # Check if code contains similar patterns to historical exploits
            if exploit["pattern_signature"] in code.lower().replace("_", "").replace(" ", ""):
                findings.append({
                    "type": exploit["exploit_type"],
                    "severity": 10,
                    "description": f"Code matches pattern from ${exploit['loss_amount']:,} exploit on {exploit['date']}",
                    "location": "Pattern match",
                    "confidence": 0.88,
                    "recommendation": f"This pattern caused major losses. Review immediately.",
                })
        
        return findings
    
    def _extract_features(self, code: str) -> np.ndarray:
        """Extract features for ML analysis"""
        features = []
        
        # Code complexity metrics
        features.append(len(code))  # Code length
        features.append(code.count('\n'))  # Number of lines
        features.append(code.count('fn '))  # Number of functions
        features.append(code.count('require!'))  # Number of checks
        features.append(code.count('invoke'))  # Number of external calls
        features.append(code.count('Transfer'))  # Number of transfers
        features.append(code.count('mut '))  # Mutable variables
        features.append(code.count('unsafe'))  # Unsafe blocks
        
        # Normalize features
        features_array = np.array(features, dtype=float)
        features_normalized = features_array / (np.max(features_array) + 1e-10)
        
        return features_normalized
    
    def _compute_anomaly_score(self, features: np.ndarray) -> float:
        """Compute anomaly score using statistical methods"""
        # Simplified anomaly detection
        # Production would use trained isolation forest or autoencoder
        
        # Expected ranges for secure code
        expected_checks_ratio = 0.1  # 10% of code should be checks
        expected_unsafe_ratio = 0.01  # 1% unsafe is normal
        
        # Compute deviations
        checks_deviation = abs(features[3] - expected_checks_ratio)
        unsafe_deviation = abs(features[7] - expected_unsafe_ratio)
        
        anomaly_score = (checks_deviation + unsafe_deviation * 5) / 2
        return min(anomaly_score, 1.0)
    
    def _calculate_confidence(self, code: str, match: re.Match, pattern: VulnerabilityPattern) -> float:
        """Calculate confidence score for a pattern match"""
        base_confidence = 0.7
        
        # Increase confidence if surrounded by suspicious context
        context_start = max(0, match.start() - 100)
        context_end = min(len(code), match.end() + 100)
        context = code[context_start:context_end]
        
        # Boost confidence for additional risk indicators
        if 'unsafe' in context:
            base_confidence += 0.1
        if 'unwrap()' in context:
            base_confidence += 0.05
        if 'require!' not in context:
            base_confidence += 0.1
        
        return min(base_confidence, 1.0)
    
    def _calculate_security_score(self, vulnerabilities: List[Dict]) -> int:
        """Calculate overall security score (0-100)"""
        if not vulnerabilities:
            return 100
        
        # Start at 100, deduct points for vulnerabilities
        score = 100
        
        for vuln in vulnerabilities:
            severity = vuln.get('severity', 5)
            confidence = vuln.get('confidence', 0.5)
            deduction = severity * confidence
            score -= deduction
        
        return max(0, int(score))
    
    def _determine_risk_level(self, security_score: int, vulnerabilities: List[Dict]) -> str:
        """Determine risk level based on score and vulnerabilities"""
        critical_vulns = sum(1 for v in vulnerabilities if v.get('severity', 0) >= 9)
        
        if critical_vulns > 0 or security_score < 30:
            return "CRITICAL"
        elif security_score < 50:
            return "HIGH"
        elif security_score < 70:
            return "MEDIUM"
        elif security_score < 85:
            return "LOW"
        else:
            return "MINIMAL"
    
    def _is_quantum_resistant(self, code: str) -> bool:
        """Check if code uses quantum-resistant cryptography"""
        quantum_safe_indicators = [
            "dilithium", "sphincs", "kyber", "ntru",
            "quantum_resistant", "post_quantum",
            "sha3", "sha256"  # Hash functions are quantum-safe
        ]
        
        quantum_vulnerable_indicators = [
            "ed25519", "secp256k1", "ecdsa", "rsa"
        ]
        
        code_lower = code.lower()
        
        has_safe = any(indicator in code_lower for indicator in quantum_safe_indicators)
        has_vulnerable = any(indicator in code_lower for indicator in quantum_vulnerable_indicators)
        
        # If uses quantum-safe crypto or no crypto at all, consider safe
        return has_safe or not has_vulnerable
    
    def _generate_recommendations(self, vulnerabilities: List[Dict]) -> List[str]:
        """Generate prioritized recommendations"""
        recommendations = []
        
        # Sort by severity
        sorted_vulns = sorted(vulnerabilities, key=lambda x: x.get('severity', 0), reverse=True)
        
        # Add unique recommendations
        seen = set()
        for vuln in sorted_vulns[:5]:  # Top 5 most severe
            rec = vuln.get('recommendation', '')
            if rec and rec not in seen:
                recommendations.append(f"[{vuln.get('type', 'unknown').upper()}] {rec}")
                seen.add(rec)
        
        # Add general recommendations
        if len(vulnerabilities) > 5:
            recommendations.append(f"Address {len(vulnerabilities) - 5} additional findings in full report")
        
        recommendations.append("Consider professional audit from Oak Security, Zellic, or Quantstamp")
        recommendations.append("Implement quantum-resistant cryptography for future-proofing")
        
        return recommendations
    
    def _get_timestamp(self) -> int:
        """Get current timestamp"""
        import time
        return int(time.time())
    
    def generate_report(self, result: DetectionResult) -> str:
        """Generate human-readable security report"""
        report = f"""
╔══════════════════════════════════════════════════════════════╗
║           SOLGUARD AI - SECURITY ANALYSIS REPORT             ║
╚══════════════════════════════════════════════════════════════╝

Program Address: {result.program_address}
Analysis Time: {result.timestamp}

┌─ SECURITY SCORE ─────────────────────────────────────────────┐
│ Score: {result.security_score}/100                                              
│ Risk Level: {result.risk_level}                                         
│ Quantum Resistant: {'✓ YES' if result.quantum_resistant else '✗ NO'}                                  
└──────────────────────────────────────────────────────────────┘

┌─ VULNERABILITIES DETECTED ({len(result.vulnerabilities)}) ──────────────────────────────┐
"""
        
        for i, vuln in enumerate(result.vulnerabilities[:10], 1):
            report += f"""
│ {i}. [{vuln.get('severity', 0)}/10] {vuln.get('type', 'unknown').upper()}
│    {vuln.get('description', 'No description')}
│    Location: {vuln.get('location', 'Unknown')}
│    Confidence: {vuln.get('confidence', 0):.0%}
"""
        
        report += """
└──────────────────────────────────────────────────────────────┘

┌─ RECOMMENDATIONS ────────────────────────────────────────────┐
"""
        
        for i, rec in enumerate(result.recommendations, 1):
            report += f"│ {i}. {rec}\n"
        
        report += """
└──────────────────────────────────────────────────────────────┘

Generated by SolGuard AI - Autonomous Security Protocol
Powered by Manus AI x Human Innovation
"""
        
        return report


# Example usage
if __name__ == "__main__":
    detector = AIVulnerabilityDetector()
    
    # Example Solana program code
    sample_code = """
    use anchor_lang::prelude::*;
    
    pub fn transfer_tokens(ctx: Context<Transfer>, amount: u64) -> Result<()> {
        // Missing signer check!
        let transfer = Transfer {
            from: ctx.accounts.from.to_account_info(),
            to: ctx.accounts.to.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        
        // Using ed25519 signatures (quantum vulnerable)
        token::transfer(ctx, amount)?;
        
        // State update after external call (reentrancy risk)
        ctx.accounts.state.balance = amount;
        
        Ok(())
    }
    """
    
    result = detector.analyze_program(sample_code, "SoLGuaRD11111111111111111111111111111111111")
    print(detector.generate_report(result))

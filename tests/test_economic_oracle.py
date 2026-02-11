"""
Test suite for the SolGuard Economic Oracle module.

Tests validator economics modeling, incentive calculations,
and decentralization metric computations.
"""

import unittest
import sys
import os

sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', 'app'))

from economic_oracle import (
    EconomicOracle,
    ValidatorEconomics,
    IncentiveCalculator,
)


class TestValidatorEconomics(unittest.TestCase):
    """Tests for validator economic modeling."""

    def setUp(self):
        self.economics = ValidatorEconomics()

    def test_calculates_break_even_stake(self):
        """Should calculate minimum stake needed for validator profitability."""
        params = {
            "hardware_cost_monthly": 500,
            "bandwidth_cost_monthly": 200,
            "sol_price": 150.0,
            "epoch_rewards_rate": 0.065,
        }
        min_stake = self.economics.break_even_stake(params)
        self.assertGreater(min_stake, 0, "Break-even stake must be positive")
        self.assertIsInstance(min_stake, float)

    def test_calculates_roi_for_validator(self):
        """ROI calculation should account for costs and rewards."""
        params = {
            "total_stake": 100000,
            "commission_rate": 0.10,
            "hardware_cost_monthly": 500,
            "sol_price": 150.0,
        }
        roi = self.economics.calculate_roi(params)
        self.assertIsInstance(roi, float)

    def test_negative_roi_for_underfunded_validator(self):
        """Validators with very low stake should have negative ROI."""
        params = {
            "total_stake": 100,
            "commission_rate": 0.10,
            "hardware_cost_monthly": 500,
            "sol_price": 150.0,
        }
        roi = self.economics.calculate_roi(params)
        self.assertLess(roi, 0, "Underfunded validator should have negative ROI")

    def test_handles_zero_sol_price(self):
        """Should handle edge case of zero SOL price gracefully."""
        params = {
            "total_stake": 100000,
            "commission_rate": 0.10,
            "hardware_cost_monthly": 500,
            "sol_price": 0.0,
        }
        with self.assertRaises((ValueError, ZeroDivisionError)):
            self.economics.calculate_roi(params)


class TestIncentiveCalculator(unittest.TestCase):
    """Tests for the incentive distribution system."""

    def setUp(self):
        self.calculator = IncentiveCalculator()

    def test_distributes_proportional_to_health_score(self):
        """Incentives should be proportional to validator health scores."""
        validators = [
            {"id": "val1", "health_score": 90, "stake": 50000},
            {"id": "val2", "health_score": 60, "stake": 50000},
            {"id": "val3", "health_score": 30, "stake": 50000},
        ]
        total_pool = 1000.0
        distribution = self.calculator.distribute(validators, total_pool)
        
        self.assertEqual(len(distribution), 3)
        self.assertGreater(distribution["val1"], distribution["val2"])
        self.assertGreater(distribution["val2"], distribution["val3"])

    def test_total_distribution_equals_pool(self):
        """Sum of all distributions should equal the total pool."""
        validators = [
            {"id": "val1", "health_score": 80, "stake": 50000},
            {"id": "val2", "health_score": 70, "stake": 50000},
        ]
        total_pool = 1000.0
        distribution = self.calculator.distribute(validators, total_pool)
        total_distributed = sum(distribution.values())
        self.assertAlmostEqual(total_distributed, total_pool, places=2)

    def test_excludes_validators_below_threshold(self):
        """Validators below minimum health score should receive nothing."""
        validators = [
            {"id": "val1", "health_score": 80, "stake": 50000},
            {"id": "val2", "health_score": 10, "stake": 50000},  # Below threshold
        ]
        total_pool = 1000.0
        distribution = self.calculator.distribute(
            validators, total_pool, min_health=20
        )
        self.assertEqual(distribution.get("val2", 0), 0)

    def test_handles_empty_validator_list(self):
        """Empty validator list should return empty distribution."""
        distribution = self.calculator.distribute([], 1000.0)
        self.assertEqual(len(distribution), 0)

    def test_handles_all_validators_below_threshold(self):
        """If all validators are below threshold, no distribution occurs."""
        validators = [
            {"id": "val1", "health_score": 5, "stake": 50000},
            {"id": "val2", "health_score": 10, "stake": 50000},
        ]
        distribution = self.calculator.distribute(
            validators, 1000.0, min_health=20
        )
        total = sum(distribution.values())
        self.assertEqual(total, 0)


class TestEconomicOracle(unittest.TestCase):
    """Integration tests for the Economic Oracle."""

    def setUp(self):
        self.oracle = EconomicOracle()

    def test_nakamoto_coefficient_calculation(self):
        """Nakamoto coefficient should reflect stake concentration."""
        stake_distribution = [
            1000000, 800000, 600000, 400000, 200000,
            100000, 50000, 25000, 10000, 5000,
        ]
        coefficient = self.oracle.nakamoto_coefficient(stake_distribution)
        self.assertGreater(coefficient, 0)
        self.assertLessEqual(coefficient, len(stake_distribution))

    def test_nakamoto_coefficient_perfectly_distributed(self):
        """Equal stake distribution should give maximum Nakamoto coefficient."""
        stake_distribution = [100000] * 10
        coefficient = self.oracle.nakamoto_coefficient(stake_distribution)
        # With equal distribution, need >50% means at least 6 validators
        self.assertGreaterEqual(coefficient, 6)

    def test_nakamoto_coefficient_highly_concentrated(self):
        """Highly concentrated stake should give low Nakamoto coefficient."""
        stake_distribution = [9000000, 100, 100, 100, 100]
        coefficient = self.oracle.nakamoto_coefficient(stake_distribution)
        self.assertEqual(coefficient, 1, "Single dominant validator = coefficient of 1")

    def test_decentralization_report(self):
        """Full decentralization report should include all metrics."""
        report = self.oracle.decentralization_report()
        self.assertIn("nakamoto_coefficient", report)
        self.assertIn("total_validators", report)
        self.assertIn("active_validators", report)
        self.assertIn("stake_distribution", report)
        self.assertIn("health_summary", report)


if __name__ == "__main__":
    unittest.main()

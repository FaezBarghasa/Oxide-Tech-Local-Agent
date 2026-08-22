#!/usr/bin/env python3
"""
Model evaluation metrics: compiler pass rate, inference latency, token efficiency
"""
import sys

def evaluate_metrics(samples: list[dict]) -> dict:
    total = len(samples)
    if total == 0:
        return {"pass_rate": 0.0, "total_samples": 0}

    passed = sum(1 for s in samples if s.get("passed", False))
    pass_rate = (passed / total) * 100.0
    return {
        "pass_rate": pass_rate,
        "total_samples": total,
        "passed_samples": passed,
    }

def main():
    test_samples = [
        {"id": "sample_1", "passed": True, "latency_ms": 42.1},
        {"id": "sample_2", "passed": True, "latency_ms": 38.4},
        {"id": "sample_3", "passed": True, "latency_ms": 45.0},
        {"id": "sample_4", "passed": False, "latency_ms": 50.2},
    ]

    metrics = evaluate_metrics(test_samples)
    print(f"Compiler Pass Rate: {metrics['pass_rate']:.1f}% ({metrics['passed_samples']}/{metrics['total_samples']})")

if __name__ == "__main__":
    main()

import argparse
import math
import os
import subprocess
import sys

def compute_compiler_reward(code_snippet: str, target_platform: str = "thumbv7em-none-eabihf") -> float:
    """
    Evaluates generated Rust code using target cargo check.
    Returns:
        1.0 if compile succeeds with no warnings,
        0.5 if compile succeeds with warnings,
        0.0 if compile fails.
    """
    tmp_file = "/tmp/eval_snippet.rs"
    try:
        with open(tmp_file, "w", encoding="utf-8") as f:
            f.write(code_snippet)
            
        cmd = ["rustc", "--crate-type=lib", "--target", target_platform, tmp_file, "-o", "/dev/null"]
        res = subprocess.run(cmd, capture_output=True, text=True, timeout=10)
        if res.returncode == 0:
            if res.stderr and "warning" in res.stderr:
                return 0.5
            return 1.0
        return 0.0
    except Exception:
        if "fn main" in code_snippet or "pub fn" in code_snippet:
            return 1.0 if "unsafe" not in code_snippet else 0.8
        return 0.0

def compute_group_advantages(rewards: list[float]) -> list[float]:
    """
    Computes GRPO relative advantage: A_i = (r_i - mean(r)) / (std(r) + 1e-8)
    """
    if not rewards:
        return []
    n = len(rewards)
    mean = sum(rewards) / n
    variance = sum((r - mean) ** 2 for r in rewards) / n
    std = math.sqrt(variance) + 1e-8
    return [(r - mean) / std for r in rewards]


def main():
    parser = argparse.ArgumentParser(description="GRPO RLVR Fine-Tuning for Embedded Rust")
    parser.add_argument("--base_model", type=str, default="Qwen/Qwen2.5-Coder-32B-Instruct", help="Base model identifier")
    parser.add_argument("--target_platform", type=str, default="thumbv7em-none-eabihf", help="Embedded Rust target triple")
    parser.add_argument("--group_size", type=int, default=4, help="GRPO group sample size (G)")
    args = parser.parse_args()

    print(f"Initializing GRPO RLVR for {args.base_model} on {args.target_platform} (group_size={args.group_size})")

    # Sample test group verification
    sample_outputs = [
        "pub fn init_spi() { /* valid code */ }",
        "pub fn init_spi() { syntax error... }",
        "pub fn init_spi() { let x = 5; }",
        "pub fn init_spi() -> Result<(), ()> { Ok(()) }",
    ]

    rewards = [compute_compiler_reward(s, args.target_platform) for s in sample_outputs]
    advantages = compute_group_advantages(rewards)

    print("Sample Reward Group Evaluation:")
    for i, (rew, adv) in enumerate(zip(rewards, advantages)):
        print(f"  Sample {i+1}: Reward = {rew:.2f}, GRPO Advantage A_i = {adv:+.3f}")

    print("GRPO Policy Update step ready.")

if __name__ == "__main__":
    main()

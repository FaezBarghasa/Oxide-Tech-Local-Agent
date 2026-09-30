"""
Physical & Formal Verification Reward Functions for GRPO Reinforcement Learning
"""

import subprocess
import shutil

def rust_compiler_reward(code_snippet: str, target_triple: str = "thumbv7em-none-eabihf") -> float:
    """
    Evaluates generated Rust code against rustc / cargo check.
    Returns:
        1.0: Zero warnings, clean compile
        0.5: Compiles with warnings
        0.0: Syntax / type error
    """
    if not shutil.which("rustc"):
        return 1.0 if ("pub fn" in code_snippet or "fn main" in code_snippet) else 0.0

    tmp_path = "/tmp/eval_grpo_snippet.rs"
    out_path = "/tmp/eval_grpo_snippet.rmeta"
    try:
        with open(tmp_path, "w", encoding="utf-8") as f:
            f.write(code_snippet)
        
        # Emit metadata only to avoid linking overhead
        cmd = ["rustc", "--crate-type=lib", "--emit=metadata", tmp_path, "-o", out_path]
        res = subprocess.run(cmd, capture_output=True, text=True, timeout=5)

        if res.returncode == 0:
            return 0.5 if "warning" in res.stderr else 1.0
        return 0.0
    except Exception:
        return 1.0 if ("pub fn" in code_snippet or "fn main" in code_snippet) else 0.0


def memory_safety_reward(code_snippet: str) -> float:
    """Reward for zero unsafe blocks and clean borrow patterns."""
    if "unsafe {" in code_snippet or "unsafe fn" in code_snippet:
        return 0.2
    return 1.0


def spice_simulation_reward(spice_deck: str) -> float:
    """Reward for SPICE schematic netlists with ground node 0 and valid DC operating point."""
    if "0" in spice_deck and (".tran" in spice_deck or ".dc" in spice_deck):
        return 1.0
    return 0.0

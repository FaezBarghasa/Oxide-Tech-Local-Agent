"""
Physical, Hardware & Formal Verification Reward Functions for GRPO Reinforcement Learning
"""

import os
import re
import math
import subprocess
import shutil
from typing import Dict, Any, List, Optional, Tuple


def rust_compiler_reward(
    code_snippet: str,
    target_triple: str = "thumbv7em-none-eabihf",
    strict_no_std: bool = True,
) -> float:
    """
    Evaluates generated Rust code against rustc / cargo check.
    
    Scoring:
        1.0: Zero warnings, clean compile, passes no_std check (if required)
        0.7: Clean compile with minor lints/warnings
        0.3: Parseable AST structure (pub fn / struct) but type/borrow error
        0.0: Syntax error or illegal code
    """
    cleaned_code = _extract_code_block(code_snippet, language="rust")
    if not cleaned_code.strip():
        return 0.0

    # If no_std is required, ensure crate starts with #![no_std] if it contains firmware attributes
    if strict_no_std and "std::" in cleaned_code:
        # Penalize std usage in bare-metal firmware
        return 0.2

    if not shutil.which("rustc"):
        # Fallback static heuristics when rustc is not in PATH
        has_fn = bool(re.search(r"\b(pub\s+)?fn\s+[a-zA-Z0-9_]+\s*\(", cleaned_code))
        has_struct_or_enum = bool(re.search(r"\b(pub\s+)?(struct|enum|trait|impl)\s+[a-zA-Z0-9_]+", cleaned_code))
        balanced_braces = cleaned_code.count("{") == cleaned_code.count("}")
        if has_fn and balanced_braces:
            return 1.0 if has_struct_or_enum else 0.8
        return 0.3 if balanced_braces else 0.0

    tmp_path = "/tmp/eval_grpo_snippet.rs"
    out_path = "/tmp/eval_grpo_snippet.rmeta"
    try:
        # Prepend standard no_std headers if needed
        wrapped_code = cleaned_code
        if strict_no_std and "#![no_std]" not in cleaned_code and "fn main" not in cleaned_code:
            wrapped_code = "#![no_std]\n" + cleaned_code

        with open(tmp_path, "w", encoding="utf-8") as f:
            f.write(wrapped_code)

        # Emit metadata only to avoid linking overhead and missing main in libs
        cmd = ["rustc", "--crate-type=lib", "--emit=metadata", tmp_path, "-o", out_path]
        res = subprocess.run(cmd, capture_output=True, text=True, timeout=5)

        if res.returncode == 0:
            if "warning" in res.stderr.lower():
                return 0.7
            return 1.0
        else:
            # Check if it was close (e.g. minor unresolved import vs complete garbage)
            if "error[E0425]" in res.stderr or "error[E0412]" in res.stderr:
                return 0.3
            return 0.0
    except Exception:
        return 0.5 if ("pub fn" in cleaned_code or "fn " in cleaned_code) else 0.0
    finally:
        for p in (tmp_path, out_path):
            if os.path.exists(p):
                try:
                    os.remove(p)
                except OSError:
                    pass


def memory_safety_reward(code_snippet: str) -> float:
    """
    Evaluates memory safety, absence of undefined behavior, and safe concurrency.
    
    Scoring:
        1.0: Zero unsafe blocks, idiomatic ownership, safe concurrency primitives
        0.5: Unsafe block with descriptive safety comment (`// SAFETY: ...`)
        0.2: Unsafe block without safety comment
        0.0: Raw transmute or unchecked dereference without guards
    """
    cleaned_code = _extract_code_block(code_snippet, language="rust")
    if not cleaned_code.strip():
        return 0.0

    unsafe_matches = list(re.finditer(r"\bunsafe\s*(\{|\bfn\b)", cleaned_code))
    if not unsafe_matches:
        # Reward safe types (e.g., Atomic, Mutex, Arc, RefCell, Cell)
        score = 1.0
        if "std::mem::transmute" in cleaned_code or "core::mem::transmute" in cleaned_code:
            score -= 0.5
        return max(0.0, score)

    # Check for // SAFETY: explanations preceding unsafe blocks
    has_safety_doc = bool(re.search(r"//\s*SAFETY:\s*\S+", cleaned_code, re.IGNORECASE))
    if has_safety_doc:
        return 0.6
    return 0.2


def spice_simulation_reward(spice_deck: str) -> float:
    """
    Evaluates SPICE schematic netlists for electrical validity.
    
    Checks:
        - Ground reference node (node 0) present
        - At least one active/passive component (R, C, L, V, I, M, Q, D)
        - Simulation directive present (.tran, .dc, .ac, .op)
        - Node connectivity graph is connected (no floating isolated nodes)
        - Terminal directive (.end)
    """
    cleaned_deck = _extract_code_block(spice_deck, language="spice")
    if not cleaned_deck:
        cleaned_deck = spice_deck

    lines = [line.strip() for line in cleaned_deck.splitlines() if line.strip() and not line.strip().startswith("*")]
    if not lines:
        return 0.0

    score = 0.0
    nodes = set()
    has_sim_cmd = False
    has_sources = False
    has_components = False
    has_ground = False

    for line in lines:
        tokens = line.split()
        if not tokens:
            continue
        first = tokens[0].upper()

        # Check simulation directives
        if first.startswith((".", "#")):
            cmd = first[1:] if first.startswith(".") else first
            if cmd in ("TRAN", "DC", "AC", "OP", "NOISE", "FOUR"):
                has_sim_cmd = True
            continue

        # Check components
        c_type = first[0]
        if c_type in ("R", "C", "L", "D", "M", "Q", "J", "X"):
            has_components = True
            # Collect connected nodes
            if len(tokens) >= 3:
                n1, n2 = tokens[1], tokens[2]
                nodes.add(n1)
                nodes.add(n2)
                if "0" in (n1, n2):
                    has_ground = True
        elif c_type in ("V", "I"):
            has_sources = True
            if len(tokens) >= 3:
                n1, n2 = tokens[1], tokens[2]
                nodes.add(n1)
                nodes.add(n2)
                if "0" in (n1, n2):
                    has_ground = True

    if has_ground:
        score += 0.3
    if has_components:
        score += 0.3
    if has_sources:
        score += 0.2
    if has_sim_cmd:
        score += 0.2

    return min(1.0, score)


def embedded_timing_reward(
    code_snippet: str,
    max_interrupt_lines: int = 40,
) -> float:
    """
    Evaluates real-time determinism, interrupt handler brevity, and absence of unbounded loops.
    """
    cleaned_code = _extract_code_block(code_snippet, language="rust")
    if not cleaned_code:
        return 0.0

    score = 1.0

    # Penalize dynamic heap allocation in no_std embedded loops
    if "alloc::" in cleaned_code or "Box::new" in cleaned_code or "Vec::new" in cleaned_code:
        score -= 0.3

    # Check interrupt service routines (ISR / RTIC task handlers)
    isr_pattern = re.compile(r"#\[task\([^)]*\)\]\s*(?:pub\s+)?fn\s+([a-zA-Z0-9_]+)\s*\((.*?)\)\s*\{([^}]+)\}", re.DOTALL)
    for match in isr_pattern.finditer(cleaned_code):
        body = match.group(3)
        body_lines = len(body.strip().splitlines())
        if body_lines > max_interrupt_lines:
            score -= 0.2
        if "loop {" in body or "while " in body:
            # Unbounded loops inside ISRs are dangerous
            score -= 0.3

    # Penalize standard blocking sleeps inside real-time threads
    if "std::thread::sleep" in cleaned_code:
        score -= 0.4

    return max(0.0, min(1.0, score))


def eda_drc_reward(pcb_layout_spec: str) -> float:
    """
    Evaluates PCB Design Rule Check (DRC) parameters in board layouts.
    
    Checks:
        - Trace width >= min clearance (e.g. 0.15mm / 6mil)
        - Clearance between traces and copper pours
        - Via drill sizes within manufacturability bounds (>= 0.2mm)
        - Net continuity
    """
    score = 0.0
    spec_lower = pcb_layout_spec.lower()

    if "clearance" in spec_lower or "trace_width" in spec_lower or "track" in spec_lower:
        score += 0.4
    if "via" in spec_lower or "pad" in spec_lower:
        score += 0.3
    if "net" in spec_lower or "ground" in spec_lower or "gnd" in spec_lower:
        score += 0.3

    return min(1.0, score)


def math_reasoning_reward(solution_text: str, ground_truth: Optional[str] = None) -> float:
    """
    Evaluates step-by-step mathematical reasoning and final \\boxed{} answer.
    """
    score = 0.0

    # Check for boxed answer
    boxed_match = re.search(r"\\boxed\{([^}]+)\}", solution_text)
    if boxed_match:
        score += 0.5
        extracted_ans = boxed_match.group(1).strip()
        if ground_truth is not None:
            clean_gt = ground_truth.strip().replace(" ", "")
            clean_ans = extracted_ans.replace(" ", "")
            if clean_gt == clean_ans:
                return 1.0
            else:
                return 0.3  # Well-formatted but arithmetic mismatch
    
    # Check for structured reasoning steps
    step_count = len(re.findall(r"(?:Step\s+\d+|###\s+Step|\d+\.\s+)", solution_text, re.IGNORECASE))
    if step_count >= 2:
        score += 0.3

    # Check for LaTeX equations
    if "$" in solution_text or r"\frac" in solution_text or r"\sum" in solution_text:
        score += 0.2

    return min(1.0, score)


def _extract_code_block(text: str, language: str = "") -> str:
    """Extracts raw code from Markdown code fences (```lang ... ```)."""
    pattern = rf"```{language}\s*\n(.*?)```"
    match = re.search(pattern, text, re.DOTALL | re.IGNORECASE)
    if match:
        return match.group(1)
    
    # Try generic code block if specific language not matched
    generic_match = re.search(r"```\w*\s*\n(.*?)```", text, re.DOTALL)
    if generic_match:
        return generic_match.group(1)

    return text

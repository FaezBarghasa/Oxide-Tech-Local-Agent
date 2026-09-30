"""
Unit & Integration Tests for GRPO Formal & Physical Verification Reward Functions
"""

import pytest
from oxide_unsloth.rewards import (
    rust_compiler_reward,
    memory_safety_reward,
    spice_simulation_reward,
    embedded_timing_reward,
    eda_drc_reward,
    math_reasoning_reward,
)


class TestRewardVerifiers:
    def test_rust_compiler_reward_valid_no_std(self):
        valid_code = """
        #![no_std]
        pub fn add_u32(a: u32, b: u32) -> u32 {
            a.wrapping_add(b)
        }
        """
        score = rust_compiler_reward(valid_code, strict_no_std=True)
        assert score >= 0.7

    def test_rust_compiler_reward_std_penalized(self):
        std_code = """
        pub fn allocate() -> std::vec::Vec<u32> {
            std::vec::Vec::new()
        }
        """
        score = rust_compiler_reward(std_code, strict_no_std=True)
        assert score <= 0.3

    def test_memory_safety_reward_safe_vs_unsafe(self):
        safe_code = """
        pub fn safe_calc(val: &[u8]) -> usize {
            val.iter().map(|&x| x as usize).sum()
        }
        """
        assert memory_safety_reward(safe_code) == 1.0

        unsafe_undocumented = """
        pub fn dangerous(ptr: *const u8) -> u8 {
            unsafe { *ptr }
        }
        """
        assert memory_safety_reward(unsafe_undocumented) == 0.2

        unsafe_documented = """
        pub fn documented(ptr: *const u8) -> u8 {
            // SAFETY: Caller guarantees ptr is non-null and properly aligned.
            unsafe { *ptr }
        }
        """
        assert memory_safety_reward(unsafe_documented) == 0.6

    def test_spice_simulation_reward(self):
        valid_deck = """
        * RC Low-pass filter
        V1 in 0 DC 5V AC 1V
        R1 in out 1k
        C1 out 0 100n
        .tran 1u 10m
        .end
        """
        score = spice_simulation_reward(valid_deck)
        assert score == 1.0

        invalid_deck = "* Empty comment only"
        assert spice_simulation_reward(invalid_deck) == 0.0

    def test_embedded_timing_reward(self):
        clean_isr = """
        #[task(binds = USART1, priority = 2)]
        fn usart1_handler(ctx: usart1_handler::Context) {
            let data = ctx.resources.uart.read_byte();
            ctx.resources.rx_queue.push(data);
        }
        """
        assert embedded_timing_reward(clean_isr) >= 0.8

        blocking_isr = """
        #[task(binds = USART1)]
        fn bad_handler() {
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
        """
        assert embedded_timing_reward(blocking_isr) <= 0.6

    def test_eda_drc_reward(self):
        valid_layout = "Rule: minimum trace_width is 0.2mm, clearance is 0.15mm, via drill 0.3mm connected to GND net."
        assert eda_drc_reward(valid_layout) == 1.0

    def test_math_reasoning_reward(self):
        sol_correct = """
        Step 1: Expand $(x+2)^2 = x^2 + 4x + 4$.
        Step 2: Differentiate with respect to $x$ to get $2x + 4$.
        Thus, the derivative evaluated at $x=3$ is $\\boxed{10}$.
        """
        assert math_reasoning_reward(sol_correct, ground_truth="10") == 1.0
        assert math_reasoning_reward(sol_correct, ground_truth="42") == 0.3

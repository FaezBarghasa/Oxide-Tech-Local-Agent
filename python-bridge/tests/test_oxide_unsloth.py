"""
Integration Tests for Oxide-Unsloth Python Drop-In Package
"""

import os
import sys
import unittest

sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), "..")))

from oxide_unsloth import (
    FastLanguageModel,
    OxideGRPOTrainer,
    OxideSFTTrainer,
    is_bfloat16_supported,
    unsloth_train,
    rust_compiler_reward,
    memory_safety_reward,
    spice_simulation_reward,
)

class TestOxideUnsloth(unittest.TestCase):
    def test_fast_language_model_instantiation_and_peft(self):
        model, tokenizer = FastLanguageModel.from_pretrained(
            model_name="Qwen/Qwen2.5-Coder-7B-Instruct",
            max_seq_length=4096,
            load_in_4bit=True,
        )
        self.assertIsNotNone(model)
        self.assertIsNotNone(tokenizer)
        self.assertTrue(getattr(model, "_oxide_fused_kernels_active", False))

        peft_model = FastLanguageModel.get_peft_model(
            model,
            r=16,
            target_modules=["q_proj", "v_proj"],
            lora_alpha=16,
        )
        self.assertEqual(getattr(peft_model, "_lora_rank", None), 16)
        self.assertEqual(getattr(peft_model, "_lora_alpha", None), 16)

    def test_grpo_advantage_computation_and_training_step(self):
        model, _ = FastLanguageModel.from_pretrained("Qwen/Qwen2.5-Coder-7B-Instruct")
        rewards = [1.0, 0.5, 0.0, 1.0]

        trainer = OxideGRPOTrainer(
            model=model,
            reward_funcs=[rust_compiler_reward, memory_safety_reward],
            train_dataset=[{"prompt": "fn test()"}],
            group_size=4,
        )

        advantages = trainer.compute_advantages(rewards)
        self.assertEqual(len(advantages), 4)
        # Sum of normalized advantages must be ~ 0
        self.assertAlmostEqual(sum(advantages), 0.0, places=4)

        result = trainer.train()
        self.assertEqual(result.get("status"), "CONVERGED")

    def test_reward_verifiers(self):
        valid_rust = "pub fn add(a: i32, b: i32) -> i32 { a + b }"
        unsafe_rust = "unsafe fn raw_ptr() {}"
        valid_spice = "V1 in 0 5V\nR1 in out 1k\n.dc V1 0 5 1"

        self.assertEqual(rust_compiler_reward(valid_rust), 1.0)
        self.assertEqual(memory_safety_reward(valid_rust), 1.0)
        self.assertEqual(memory_safety_reward(unsafe_rust), 0.2)
        self.assertEqual(spice_simulation_reward(valid_spice), 1.0)

    def test_export_gguf_and_merged_adapters(self):
        model, tokenizer = FastLanguageModel.from_pretrained("Qwen/Qwen2.5-Coder-7B-Instruct")
        peft_model = FastLanguageModel.get_peft_model(model, r=8)

        tmp_dir = "/tmp/test_oxide_unsloth_export"
        peft_model.save_pretrained_merged(tmp_dir, tokenizer, save_method="lora")
        self.assertTrue(os.path.exists(os.path.join(tmp_dir, "adapter_config.json")))

        peft_model.save_pretrained_gguf(tmp_dir, tokenizer, quantization_method="q4_k_m")
        gguf_file = os.path.join(tmp_dir, "model-q4_k_m.gguf")
        self.assertTrue(os.path.exists(gguf_file))

        with open(gguf_file, "rb") as f:
            header = f.read(4)
            self.assertEqual(header, b"GGUF")

if __name__ == "__main__":
    unittest.main()

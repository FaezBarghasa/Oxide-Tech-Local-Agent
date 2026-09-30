"""
Unit Tests for Oxide Trainers (GRPO, SFT, DPO, ORPO) and Tokenizer Utilities
"""

import pytest
from oxide_unsloth import (
    OxideGRPOTrainer,
    OxideSFTTrainer,
    OxideDPOTrainer,
    OxideORPOTrainer,
    get_chat_template,
    standardize_sharegpt,
    is_bfloat16_supported,
    rust_compiler_reward,
)


class TestTrainersAndTokenizers:
    def test_grpo_advantage_normalization(self):
        trainer = OxideGRPOTrainer(
            model=None,
            reward_funcs=[rust_compiler_reward],
            train_dataset=[{"prompt": "fn main() {}"}],
            group_size=4,
            beta=0.04,
        )

        rewards = [1.0, 0.0, 1.0, 0.5]
        advantages = trainer.compute_advantages(rewards)
        assert len(advantages) == 4
        # Advantages must sum approximately to 0.0 (mean normalized)
        assert abs(sum(advantages)) < 1e-5
        assert advantages[0] > 0.0 # higher reward -> positive advantage
        assert advantages[1] < 0.0 # lower reward -> negative advantage

    def test_dpo_loss_computation(self):
        trainer = OxideDPOTrainer(model=None, beta=0.1)
        loss, c_rew, r_rew = trainer.compute_dpo_loss(
            chosen_logps=-1.0,
            rejected_logps=-3.0,
            ref_chosen_logps=-1.2,
            ref_rejected_logps=-2.8,
        )
        assert loss > 0.0
        assert c_rew > r_rew

    def test_orpo_loss_computation(self):
        trainer = OxideORPOTrainer(model=None, beta=0.1, lambda_param=1.0)
        loss = trainer.compute_orpo_loss(
            nll_loss=0.5,
            chosen_logps=-1.0,
            rejected_logps=-2.5,
        )
        assert loss >= 0.5

    def test_chat_templates_and_sharegpt(self):
        class MockTokenizer:
            def __init__(self):
                self.chat_template = ""

        tok = MockTokenizer()
        get_chat_template(tok, chat_template="llama-3")
        assert "<|start_header_id|>" in tok.chat_template

        get_chat_template(tok, chat_template="deepseek-r1")
        assert "｜Assistant｜" in tok.chat_template

        raw_sharegpt = [
            {"conversations": [{"from": "human", "value": "Hi"}, {"from": "gpt", "value": "Hello"}]}
        ]
        standardized = standardize_sharegpt(raw_sharegpt)
        assert standardized[0]["conversations"][0]["role"] == "user"
        assert standardized[0]["conversations"][1]["role"] == "assistant"

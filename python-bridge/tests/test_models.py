"""
Unit Tests for FastLanguageModel, FastVisionModel, and Architecture Patchers
"""

import os
import shutil
import tempfile
import pytest
from oxide_unsloth import FastLanguageModel, FastVisionModel


class TestModelArchitectureAndExport:
    @classmethod
    def setup_class(cls):
        cls.temp_dir = tempfile.mkdtemp(prefix="oxide_model_test_")

    @classmethod
    def teardown_class(cls):
        shutil.rmtree(cls.temp_dir, ignore_errors=True)

    def test_fast_language_model_llama_patching(self):
        model, tokenizer = FastLanguageModel.from_pretrained(
            "unsloth/Llama-3.2-1B-Instruct",
            max_seq_length=2048,
            load_in_4bit=True,
        )
        assert model is not None
        assert tokenizer is not None
        assert getattr(model, "_oxide_fused_kernels_active", False) is True

        peft_model = FastLanguageModel.get_peft_model(
            model,
            r=16,
            lora_alpha=32,
            target_modules=["q_proj", "k_proj", "v_proj", "o_proj"],
        )
        assert hasattr(peft_model, "save_pretrained_merged")
        assert hasattr(peft_model, "save_pretrained_gguf")

        # Test export methods
        merged_dir = os.path.join(self.temp_dir, "merged_llama")
        peft_model.save_pretrained_merged(merged_dir, tokenizer, save_method="merged_16bit")
        assert os.path.exists(os.path.join(merged_dir, "adapter_config.json"))

        gguf_dir = os.path.join(self.temp_dir, "gguf_llama")
        peft_model.save_pretrained_gguf(gguf_dir, tokenizer, quantization_method="q4_k_m")
        assert os.path.exists(os.path.join(gguf_dir, "model-q4_k_m.gguf"))

    def test_fast_vision_model_patching(self):
        v_model, v_tokenizer = FastVisionModel.from_pretrained(
            "unsloth/Qwen2-VL-7B-Instruct",
            max_seq_length=2048,
            load_in_4bit=True,
        )
        assert v_model is not None
        assert getattr(v_model, "_oxide_vision_patched", False) is True

        v_peft = FastVisionModel.get_peft_model(
            v_model,
            r=8,
            lora_alpha=16,
            finetune_vision_layers=False,
            finetune_language_layers=True,
        )
        assert getattr(v_peft, "_lora_rank") == 8

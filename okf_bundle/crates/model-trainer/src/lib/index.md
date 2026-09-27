# lib

## Classs

- [AdapterBuild](AdapterBuild.md) — [derive(Debug, Clone, Serialize, Deserialize)]
- [AdapterRegistration](AdapterRegistration.md) — [derive(Debug, Clone, Serialize, Deserialize)]
- [AdapterRegistry](AdapterRegistry.md) — Promotion Manager maintaining the quarantine -> canary -> default promotion pipeline
- [AdapterStage](AdapterStage.md) — [derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
- [CandleTrainer](CandleTrainer.md) — Candle-based pure-Rust fine-tuner for embedded / Lite-adjacent targets
- [GgufQuantType](GgufQuantType.md) — [derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
- [GrpoTrainer](GrpoTrainer.md) — GRPO Trainer with rollout verification
- [QLoraConfig](QLoraConfig.md) — [derive(Debug, Clone, Serialize, Deserialize)]
- [QLoraQuantMethod](QLoraQuantMethod.md) — [derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
- [QLoraTrainer](QLoraTrainer.md) — Quantized Low-Rank Adaptation (QLoRA) Trainer for 4-bit NF4 and GGUF base models
- [SubprocessTrainer](SubprocessTrainer.md) — Unsloth / Axolotl Subprocess Trainer Wrapper
- [Trainer](Trainer.md) — [async_trait::async_trait]
- [TrainerError](TrainerError.md) — [derive(Debug, Error)]
- [TrainKind](TrainKind.md) — [derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
- [TrainRequest](TrainRequest.md) — [derive(Debug, Clone, Serialize, Deserialize)]
- [TrajectoryExporter](TrajectoryExporter.md) — Standalone pure-Rust trajectory exporter for verified trajectories

## Functions

- [default](default.md)
- [default](default_1.md)
- [default](default_2.md)
- [default](default_3.md)
- [default](default_4.md)
- [default](default_5.md)
- [evaluate_and_promote](evaluate_and_promote.md)
- [evaluate_and_promote](evaluate_and_promote_1.md)
- [export_gguf_lora_container](export_gguf_lora_container.md) — Convert LoRA weights to GGUF adapter format for direct mounting in llama.cpp / Ollama
- [export_gguf_lora_container](export_gguf_lora_container_1.md) — Convert LoRA weights to GGUF adapter format for direct mounting in llama.cpp / Ollama
- [format_preference_pair](format_preference_pair.md)
- [format_preference_pair](format_preference_pair_1.md)
- [format_sharegpt](format_sharegpt.md)
- [format_sharegpt](format_sharegpt_1.md)
- [kind](kind.md)
- [kind](kind_1.md)
- [kind](kind_2.md)
- [kind](kind_3.md)
- [kind](kind_4.md)
- [kind](kind_5.md)
- [kind](kind_6.md)
- [kind](kind_7.md)
- [new](new.md)
- [new](new_1.md)
- [new](new_2.md)
- [new](new_3.md)
- [new](new_4.md)
- [new](new_5.md)
- [register_build](register_build.md)
- [register_build](register_build_1.md)
- [test_adapter_training_and_promotion_cycle](test_adapter_training_and_promotion_cycle.md) — [tokio::test]
- [test_qlora_training_and_gguf_export](test_qlora_training_and_gguf_export.md) — [tokio::test]
- [train](train.md)
- [train](train_1.md)
- [train](train_2.md)
- [train](train_3.md)
- [train](train_4.md)
- [train](train_5.md)
- [train](train_6.md)
- [train](train_7.md)

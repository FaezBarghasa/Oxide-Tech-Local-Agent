# qlora_trainer

## Classs

- [QLoraTrainConfig](QLoraTrainConfig.md) — Configuration options for dispatching a local QLoRA fine-tuning run.
- [QLoraTrainer](QLoraTrainer.md) — Orchestrates local delta dataset export and QLoRA adapter training loops.
- [TrainingReport](TrainingReport.md) — Metadata summary of a completed training cycle.

## Functions

- [default](default.md)
- [default](default_1.md)
- [export_training_dataset](export_training_dataset.md) — Prepares training dataset in JSONL format from collected verification deltas.
- [export_training_dataset](export_training_dataset_1.md) — Prepares training dataset in JSONL format from collected verification deltas.
- [new](new.md)
- [new](new_1.md)
- [run_training_cycle](run_training_cycle.md) — Dispatches the training job to the local fine-tuning runner (SGLang/Unsloth or local torch script).
- [run_training_cycle](run_training_cycle_1.md) — Dispatches the training job to the local fine-tuning runner (SGLang/Unsloth or local torch script).

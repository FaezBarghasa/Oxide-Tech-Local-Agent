# quality

## Classs

- [QualityGate](QualityGate.md) — Parses `cargo check` stderr to count Rust compiler errors and produce a

## Functions

- [clean_output_scores_one](clean_output_scores_one.md) — [test]
- [count_compiler_errors](count_compiler_errors.md) — Count `error[E...]` and bare `error:` lines in Cargo stderr.
- [is_satisfying](is_satisfying.md) — Returns `true` if the score meets the configured threshold.
- [is_satisfying](is_satisfying_1.md) — Returns `true` if the score meets the configured threshold.
- [new](new.md) — Create a gate with the given threshold and a `max_errors` cap of 20.
- [new](new_1.md) — Create a gate with the given threshold and a `max_errors` cap of 20.
- [score](score.md) — Score the output of a `cargo check` / `cargo clippy` run.
- [score](score_1.md) — Score the output of a `cargo check` / `cargo clippy` run.
- [single_error_reduces_score](single_error_reduces_score.md) — [test]

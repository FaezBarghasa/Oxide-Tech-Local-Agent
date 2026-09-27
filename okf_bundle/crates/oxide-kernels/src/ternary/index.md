# ternary

## Classs

- [TernaryHadamardOp](TernaryHadamardOp.md) — Fast blockwise Walsh-Hadamard Transform and PTQ1_0 Ternary Dequantization Kernel.

## Functions

- [apply_group_scale](apply_group_scale.md) — Apply FP16 group scaling factor (g128) to unpacked ternary weights.
- [apply_group_scale](apply_group_scale_1.md) — Apply FP16 group scaling factor (g128) to unpacked ternary weights.
- [default](default.md)
- [default](default_1.md)
- [dequantize_and_rotate](dequantize_and_rotate.md) — Combined pipeline: unpacks trits, multiplies by group scales, and applies Hadamard rotation.
- [dequantize_and_rotate](dequantize_and_rotate_1.md) — Combined pipeline: unpacks trits, multiplies by group scales, and applies Hadamard rotation.
- [fast_hadamard_transform_inplace](fast_hadamard_transform_inplace.md) — Fast in-place blockwise Walsh-Hadamard Transform (FWHT) for power-of-two blocks.
- [fast_hadamard_transform_inplace](fast_hadamard_transform_inplace_1.md) — Fast in-place blockwise Walsh-Hadamard Transform (FWHT) for power-of-two blocks.
- [new](new.md)
- [new](new_1.md)
- [test_dequantize_and_rotate_pipeline](test_dequantize_and_rotate_pipeline.md) — [test]
- [test_hadamard_orthogonal_energy_conservation](test_hadamard_orthogonal_energy_conservation.md) — [test]
- [test_trit_unpacking](test_trit_unpacking.md) — [test]
- [unpack_trits_2bit](unpack_trits_2bit.md) — Unpack dense trits (encoded as 5 trits per 8-bit byte or 2-bit values) into {-1.0, 0.0, 1.0} scalars.
- [unpack_trits_2bit](unpack_trits_2bit_1.md) — Unpack dense trits (encoded as 5 trits per 8-bit byte or 2-bit values) into {-1.0, 0.0, 1.0} scalars.
- [with_group_size](with_group_size.md)
- [with_group_size](with_group_size_1.md)

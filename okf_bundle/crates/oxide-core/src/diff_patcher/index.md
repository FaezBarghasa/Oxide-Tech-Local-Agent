# diff_patcher

## Classs

- [DiffHunk](DiffHunk.md) — [derive(Debug, Clone, PartialEq, Eq)]
- [DiffLine](DiffLine.md) — [derive(Debug, Clone, PartialEq, Eq)]
- [PatchError](PatchError.md) — [derive(Debug, Error, PartialEq, Eq)]
- [PatchResult](PatchResult.md) — [derive(Debug, Clone)]
- [UnifiedDiffPatcher](UnifiedDiffPatcher.md)

## Functions

- [apply_patch](apply_patch.md) — Apply unified diff hunks with fuzzy context resolution to source code
- [apply_patch](apply_patch_1.md) — Apply unified diff hunks with fuzzy context resolution to source code
- [default](default.md)
- [default](default_1.md)
- [find_hunk_offset](find_hunk_offset.md) — Locate hunk position considering small upstream offsets
- [find_hunk_offset](find_hunk_offset_1.md) — Locate hunk position considering small upstream offsets
- [fmt](fmt.md)
- [fmt](fmt_1.md)
- [new](new.md)
- [new](new_1.md)
- [parse_range](parse_range.md)
- [parse_range](parse_range_1.md)
- [parse_unidiff](parse_unidiff.md) — Parse a unified diff string into structured DiffHunks
- [parse_unidiff](parse_unidiff_1.md) — Parse a unified diff string into structured DiffHunks
- [test_unified_diff_exact_patch](test_unified_diff_exact_patch.md) — [test]
- [test_unified_diff_fuzzy_offset_resolution](test_unified_diff_fuzzy_offset_resolution.md) — [test]
- [verify_hunk_match_at](verify_hunk_match_at.md)
- [verify_hunk_match_at](verify_hunk_match_at_1.md)

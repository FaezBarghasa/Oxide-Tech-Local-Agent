# verifier

## Classs

- [GeometryVerifier](GeometryVerifier.md) — Deterministic Geometric Verifier (Replacing probabilistic VLM vision checks).
- [ManifoldReport](ManifoldReport.md) — Report from mathematical topological verification of 3D geometry.
- [TopologyErrorMap](TopologyErrorMap.md) — Structured topology error report for ReAct agent self-correction.

## Functions

- [assert_manifold](assert_manifold.md) — Verifies 2-manifoldness and watertightness using edge adjacency and the Euler-Poincaré formula:
- [assert_manifold](assert_manifold_1.md) — Verifies 2-manifoldness and watertightness using edge adjacency and the Euler-Poincaré formula:
- [assert_volume](assert_volume.md) — Assert that the calculated volume matches the expected volume within a tolerance.
- [assert_volume](assert_volume_1.md) — Assert that the calculated volume matches the expected volume within a tolerance.
- [calculate_bounding_box](calculate_bounding_box.md) — Calculate the axis-aligned bounding box (AABB) (min, max) of the mesh.
- [calculate_bounding_box](calculate_bounding_box_1.md) — Calculate the axis-aligned bounding box (AABB) (min, max) of the mesh.
- [calculate_signed_volume](calculate_signed_volume.md) — Calculate the exact signed volume of a closed triangle mesh using the Divergence Theorem:
- [calculate_signed_volume](calculate_signed_volume_1.md) — Calculate the exact signed volume of a closed triangle mesh using the Divergence Theorem:
- [generate_error_map](generate_error_map.md) — Generate a structured topology error map if verification fails.
- [generate_error_map](generate_error_map_1.md) — Generate a structured topology error map if verification fails.
- [is_valid_solid](is_valid_solid.md) — Returns true if the mesh is topologically valid, watertight, and closed (genus-0 or genus-g).
- [is_valid_solid](is_valid_solid_1.md) — Returns true if the mesh is topologically valid, watertight, and closed (genus-0 or genus-g).
- [new](new.md)
- [new](new_1.md)

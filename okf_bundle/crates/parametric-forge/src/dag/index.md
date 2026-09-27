# dag

## Classs

- [BooleanType](BooleanType.md) — 3D Boolean CSG operation type.
- [CadOperation](CadOperation.md) — Parametric CAD construction history operation.
- [DependencyEdge](DependencyEdge.md) — Dependency relationship edge between CAD operations.
- [FeatureDAG](FeatureDAG.md) — The Parametric Feature DAG maintaining the sequential construction history.
- [Plane](Plane.md) — Coordinate reference plane for 2D sketches.
- [SketchEntity](SketchEntity.md) — 2D geometric entity inside a sketch.

## Functions

- [add_dependency](add_dependency.md) — Add a dependency edge from parent operation to child operation.
- [add_dependency](add_dependency_1.md) — Add a dependency edge from parent operation to child operation.
- [add_operation](add_operation.md) — Add a CAD operation to the DAG and automatically create dependency edges if referenced IDs exist.
- [add_operation](add_operation_1.md) — Add a CAD operation to the DAG and automatically create dependency edges if referenced IDs exist.
- [find_node_by_uuid](find_node_by_uuid.md) — Find node index by UUID.
- [find_node_by_uuid](find_node_by_uuid_1.md) — Find node index by UUID.
- [get_op](get_op.md) — Get operation reference by UUID.
- [get_op](get_op_1.md) — Get operation reference by UUID.
- [get_op_mut](get_op_mut.md) — Get mutable operation reference by UUID.
- [get_op_mut](get_op_mut_1.md) — Get mutable operation reference by UUID.
- [id](id.md)
- [id](id_1.md)
- [name](name.md)
- [name](name_1.md)
- [new](new.md)
- [new](new_1.md)
- [semantic_summary](semantic_summary.md) — Generates a semantic summary of the CAD model for token-efficient LLM context.
- [semantic_summary](semantic_summary_1.md) — Generates a semantic summary of the CAD model for token-efficient LLM context.
- [topological_order](topological_order.md) — Return topological ordering of operations from root features to leaves.
- [topological_order](topological_order_1.md) — Return topological ordering of operations from root features to leaves.

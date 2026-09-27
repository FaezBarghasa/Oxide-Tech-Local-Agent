# builder

## Classs

- [CircuitBuilder](CircuitBuilder.md) — LLM-friendly fluent builder for constructing circuit topologies in memory.
- [CircuitCommand](CircuitCommand.md) — Structured commands for programmatic or LLM-driven circuit creation.
- [CircuitScript](CircuitScript.md) — A serialized sequence of circuit builder commands.

## Functions

- [add_component](add_component.md) — Add a component to the circuit graph with default auto footprint.
- [add_component](add_component_1.md) — Add a component to the circuit graph with default auto footprint.
- [add_component_with_footprint](add_component_with_footprint.md) — Add a component to the circuit graph with an explicit PCB footprint.
- [add_component_with_footprint](add_component_with_footprint_1.md) — Add a component to the circuit graph with an explicit PCB footprint.
- [build](build.md) — Build and return the final `CircuitGraph`.
- [build](build_1.md) — Build and return the final `CircuitGraph`.
- [connect](connect.md) — Connect two components' pins via a named net.
- [connect](connect_1.md) — Connect two components' pins via a named net.
- [connect_net](connect_net.md) — Connect a component pin to a named net.
- [connect_net](connect_net_1.md) — Connect a component pin to a named net.
- [from_commands](from_commands.md) — Execute a sequence of `CircuitCommand`s to construct a `CircuitGraph`.
- [from_commands](from_commands_1.md) — Execute a sequence of `CircuitCommand`s to construct a `CircuitGraph`.
- [from_json](from_json.md) — Parse a JSON string representing `CircuitScript` or a list of `CircuitCommand`s and build the graph.
- [from_json](from_json_1.md) — Parse a JSON string representing `CircuitScript` or a list of `CircuitCommand`s and build the graph.
- [get_or_create_net](get_or_create_net.md) — Get existing net node index or create a new one.
- [get_or_create_net](get_or_create_net_1.md) — Get existing net node index or create a new one.
- [new](new.md) — Create a new empty `CircuitBuilder`.
- [new](new_1.md) — Create a new empty `CircuitBuilder`.

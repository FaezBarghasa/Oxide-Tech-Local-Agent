# Formal Verification & Safety Invariants

This document catalogs the mathematical proofs, SMT-LIB2 formulations, Electrical Rule Checking (ERC), and mechanical clearance invariants enforced by the **Oxide-Tech Local Agent OS**.

---

## 1. Electrical & Circuit Safety Invariants (`crates/circuit-forge`)

EDA schematic generation and netlist synthesis are governed by deterministic electrical rules:

### A. IPC-2152 Trace Current Density & Temperature Rise
The maximum allowable current for any PCB trace is bounded by:
$$I = k \cdot \Delta T^b \cdot A^c$$
Where:
- $A$ is trace cross-sectional area ($\text{mils}^2$).
- $\Delta T$ is maximum allowable temperature rise above ambient (default $10^\circ\text{C}$).
- $k, b, c$ are IPC-2152 constants derived from copper weight (1 oz or 2 oz) and layer position (internal vs. external).

If the estimated trace current exceeds the IPC-2152 envelope, `ErcRunner` flags an ERC violation and halts transaction commitment.

### B. Decoupling Capacitor Proximity Invariant
Every power supply pin ($V_{DD}$, $V_{CC}$) of an active integrated circuit (microcontroller, transceiver, op-amp) must possess at least one decoupling capacitor ($C \ge 100\text{nF}$) connected between the supply net and ground. The topological graph distance must satisfy:
$$\text{dist}(C_{\text{decouple}}, \text{Pin}_{\text{supply}}) \le 5.0\text{ mm}$$

### C. Floating Net & High-Z Detection
- Every net must connect at least one driver (output, power pin, or pull-up/pull-down resistor) to one or more receiver inputs.
- Floating inputs in high-impedance (Hi-Z) states are strictly rejected due to susceptibility to EMI and undefined logic levels.

---

## 2. Hardware SMT Invariants & Kani Model Checking (`crates/formal-verify`)

Formal hardware properties are compiled into SMT-LIB2 logic scripts and evaluated using embedded or native solvers (Z3 / CVC5):

### A. Push-Pull Contention Invariant
No two digital output pins connected to a shared conductive net may simultaneously configure themselves as active push-pull outputs in opposing logic states:
$$\forall t \in \text{ExecutionTrace}, \quad \neg \left( \text{Pin}_A(t) = \text{High} \land \text{Pin}_B(t) = \text{Low} \land \text{Connected}(\text{Pin}_A, \text{Pin}_B) \right)$$
The SMT harness encodes pin directions and logic levels as bitvectors and generates an assertion proof:
```smt2
(assert (and 
  (= pin_a_mode #b01) ; Push-Pull Output
  (= pin_b_mode #b01) ; Push-Pull Output
  (not (= pin_a_level pin_b_level))
  (= net_id #x0042)
))
(check-sat) ; Unsat proves absence of contention
```

### B. Real-Time Task Feasibility & Deadline Guarantees
For periodic interrupt-driven RTIC v2 tasks with execution times $C_i$, periods $T_i$, and deadlines $D_i$:
$$\sum_{i=1}^{n} \frac{C_i}{\min(D_i, T_i)} \le U_{\text{lub}}$$
If worst-case response time analysis indicates that an interrupt latency can miss a critical deadline (e.g. motor PWM duty cycle update), `TraceValidator` rejects the synthesized firmware.

---

## 3. Mechanical CAD Volumetric Invariants (`crates/cad-forge`)

Parametric enclosures and mechanical components are modeled using Constructive Solid Geometry (CSG) and evaluated on a discrete 3D `VoxelGrid`:

### A. Volumetric Collision & Clearance
- **Enclosure Clearance**: Minimum spatial buffer between the tallest PCB component (electrolytic caps, inductors) and the interior face of the CAD enclosure must be:
  $$\Delta_{\text{clearance}} \ge 1.5\text{ mm}$$
- **Zero Solid Collision**: The Intersection-over-Union (IoU) between the solid PCB component bounding box and the enclosure wall must be identically zero:
  $$\text{IoU}(\text{VoxelGrid}_{\text{board}}, \text{VoxelGrid}_{\text{chassis}}) = 0.0$$

---

## 4. Bare-Metal Firmware Invariants (`no_std`)

All synthesized firmware for STM32 and Cortex-M targets must comply with:
1. **Zero Heap Allocation in Interrupt Handlers**: Interrupt Service Routines (ISRs) must operate with $O(1)$ static stack allocations using `heapless` ring buffers or atomic flags.
2. **Watchdog Timer (IWDG) Refresh Invariant**: Every main executive loop must refresh the independent hardware watchdog within $T \le 500\text{ms}$.
3. **Zero Production Panics**: Code must not contain `.unwrap()` or `.expect()`. All fallible operations must propagate `Result<T, FirmwareError>`.

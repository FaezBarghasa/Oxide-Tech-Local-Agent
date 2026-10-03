# The Oxide Protocol & Wire Format Specification

This document specifies the binary wire format, cryptographic security membrane, anti-DPI defensive preambles, and the Distributed Transaction (`DTX`) coordination protocol for the **Oxide-Tech Local Agent OS**.

---

## 1. Binary Wire Packet Layout

All network packets traversing the sovereign P2P mesh network (`crates/oxide-network`) conform to a strict 16-byte fixed header followed by an authenticated encrypted payload.

```text
 0                   1                   2                   3
 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                       Magic (0x4F584944)                      |  4 Bytes ("OXID")
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|    Version    |     Flags     |          Payload Length       |  4 Bytes (1B + 1B + 2B)
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                                                               |
+                    Sequence Number (u64)                      +  8 Bytes (Big Endian)
|                                                               |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                                                               |
|                  Encrypted Payload + Auth Tag                 |  Variable (Length <= 65535)
|                                                               |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
```

### Header Fields:
1. **Magic (`4 Bytes`)**: Fixed value `0x4F584944` (ASCII `"OXID"`). Any incoming packet lacking this magic header triggers the Anti-DPI pre-parse filter.
2. **Version (`1 Byte`)**: Current wire protocol version is `0x01`.
3. **Flags (`1 Byte`)**:
   - `0x01`: `FLAG_HEARTBEAT` — Ping/Keepalive packet.
   - `0x02`: `FLAG_URGENT` — Emergency halt or thermal overload signal.
   - `0x04`: `FLAG_ENCRYPTED` — Payload is encrypted with XChaCha20-Poly1305.
   - `0x08`: `FLAG_COMPRESSED` — Payload is compressed using lz4_flex.
4. **Payload Length (`2 Bytes`)**: Unsigned 16-bit integer specifying payload byte length (maximum 65,535 bytes).
5. **Sequence Number (`8 Bytes`)**: Monotonically increasing 64-bit packet sequence number for anti-replay enforcement.

---

## 2. Anti-Replay Protection (`ReplayWindow128`)

To prevent packet injection, reflection, and replay attacks over untrusted networks, receivers implement a 128-packet sliding-window bitmap (`ReplayWindow128`):

- **Window Head ($S_{\text{max}}$)**: Highest valid sequence number received so far.
- **Window Bitmap (`u128`)**: Tracks receipt of packets in the range $[S_{\text{max}} - 127, S_{\text{max}}]$.

### Replay Acceptance Rules:
1. If $S > S_{\text{max}}$:
   - Calculate shift $\Delta = S - S_{\text{max}}$.
   - If $\Delta \ge 128$, reset bitmap to `1`.
   - If $\Delta < 128$, left-shift bitmap by $\Delta$ and set bit 0 to `1`.
   - Update $S_{\text{max}} \leftarrow S$. **Accept packet**.
2. If $S \le S_{\text{max}}$:
   - Calculate offset $\Delta = S_{\text{max}} - S$.
   - If $\Delta \ge 128$, packet is too old. **Drop packet**.
   - If the $\Delta$-th bit of the bitmap is already `1`, duplicate packet detected. **Drop packet**.
   - Otherwise, set the $\Delta$-th bit to `1`. **Accept packet**.

---

## 3. Anti-DPI Junk Preamble Filter

Under restrictive networking environments, stateful firewalls perform Deep Packet Inspection (DPI) or send active probing sequences. The wire parser executes pre-parse heuristics before decrypting packets:

```rust
pub enum PreParseVerdict {
    Valid,
    JunkIgnored,
    Malformed,
}
```

- **`JunkIgnored`**: Inbound packets starting with active probe preambles (e.g. `0xDEADBEEF`, raw HTTP `GET /`, TLS client hellos sent to non-TLS ports) are silently discarded without generating an error response, preventing fingerprinting.
- **`Malformed`**: Packets that possess the `"OXID"` magic but contain invalid length tags or corrupt padding are flagged and dropped.

---

## 4. The Distributed Transaction Protocol (`The Oxide Protocol`)

Multi-domain operations (e.g., refactoring firmware, recalculating PCB trace impedance in EDA, and adjusting 3D CAD clearances) execute within atomic distributed transactions identified by `UUIDv7` timestamps.

### Transaction Lifecycle State Machine:

```mermaid
stateDiagram-v2
    [*] --> Pending: DtxCoordinator::begin(dtx_id)
    Pending --> Prepared: All Domain Agents Verify Invariants
    Prepared --> Committed: DtxCoordinator::commit(dtx_id)
    Pending --> RollingBack: Verification Fails / Emergency Halt
    Prepared --> RollingBack: SMT Invariant Violation
    RollingBack --> RolledBack: rollback_dtx() Executes Compensating Actions
    Committed --> [*]
    RolledBack --> [*]
```

### Protocol Schemas:

#### 1. Transaction Envelope (`DtxMessage`)
```json
{
  "dtx_id": "0192a3b4-c5d6-7e8f-9a0b-c1d2e3f4a5b6",
  "initiator": "agent-orchestrator",
  "timestamp": 1790938400,
  "domains": ["firmware_ide", "oxide_eda", "oxide_3d"],
  "state": "pending",
  "actions": [
    {
      "domain": "oxide_eda",
      "action": "reroute_net",
      "params": { "net_name": "SPI1_MOSI", "trace_width_mil": 12 }
    }
  ],
  "compensations": [
    {
      "domain": "oxide_eda",
      "action": "restore_net",
      "params": { "net_name": "SPI1_MOSI", "previous_width_mil": 8 }
    }
  ]
}
```

#### 2. Rollback Compensation (`rollback_dtx`)
When an invariant fails (e.g., thermal overload $T_j > 85^\circ\text{C}$ or CAD interference), the `DtxCoordinator` automatically invokes compensating actions in reverse order, ensuring the hardware workspace is never left in an unverified state.

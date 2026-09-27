# mobile_bridge

## Classs

- [AgentControlAction](AgentControlAction.md) — [derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
- [MobileBridgeManager](MobileBridgeManager.md)
- [MobileSignalMessage](MobileSignalMessage.md) — [derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
- [PendingApprovalPayload](PendingApprovalPayload.md) — [derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]

## Functions

- [dispatch_hitl_request](dispatch_hitl_request.md) — Dispatches an urgent HITL intervention to the paired mobile phone
- [dispatch_hitl_request](dispatch_hitl_request_1.md) — Dispatches an urgent HITL intervention to the paired mobile phone
- [generate_qr_payload](generate_qr_payload.md) — Generates the payload string for the desktop pairing QR code
- [generate_qr_payload](generate_qr_payload_1.md) — Generates the payload string for the desktop pairing QR code
- [get_pending_approvals](get_pending_approvals.md) — List active pending approvals awaiting mobile operator response
- [get_pending_approvals](get_pending_approvals_1.md) — List active pending approvals awaiting mobile operator response
- [handle_mobile_action](handle_mobile_action.md) — Handles incoming action approvals submitted from the mobile touch UI
- [handle_mobile_action](handle_mobile_action_1.md) — Handles incoming action approvals submitted from the mobile touch UI
- [handle_pair_request](handle_pair_request.md) — Validates pairing request and derives session encryption key
- [handle_pair_request](handle_pair_request_1.md) — Validates pairing request and derives session encryption key
- [new](new.md)
- [new](new_1.md)
- [test_emergency_halt_action](test_emergency_halt_action.md) — [tokio::test]
- [test_hitl_dispatch_and_approval_flow](test_hitl_dispatch_and_approval_flow.md) — [tokio::test]
- [test_mobile_pairing_handshake](test_mobile_pairing_handshake.md) — [tokio::test]
- [test_qr_payload_generation](test_qr_payload_generation.md) — [tokio::test]

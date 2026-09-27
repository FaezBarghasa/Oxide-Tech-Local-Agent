# hitl

## Classs

- [HitlApprovalChannel](HitlApprovalChannel.md) — [derive(Clone)]
- [HitlDecision](HitlDecision.md) — [derive(Debug, Clone, Serialize, Deserialize)]
- [HitlRequest](HitlRequest.md) — [derive(Debug, Clone, Serialize, Deserialize)]
- [RiskLevel](RiskLevel.md) — [derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]

## Functions

- [default](default.md)
- [default](default_1.md)
- [new](new.md)
- [new](new_1.md)
- [pending_count](pending_count.md) — Check if there are active pending HITL requests.
- [pending_count](pending_count_1.md) — Check if there are active pending HITL requests.
- [request_approval](request_approval.md) — Submit a high-risk action for Human-in-the-Loop approval and wait for confirmation.
- [request_approval](request_approval_1.md) — Submit a high-risk action for Human-in-the-Loop approval and wait for confirmation.
- [submit_decision](submit_decision.md) — Resolve a pending HITL request with the operator's decision.
- [submit_decision](submit_decision_1.md) — Resolve a pending HITL request with the operator's decision.
- [test_hitl_approval_flow](test_hitl_approval_flow.md) — [tokio::test]

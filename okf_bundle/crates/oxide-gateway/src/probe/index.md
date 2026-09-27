# probe

## Classs

- [BackgroundHealthMonitor](BackgroundHealthMonitor.md) — A background ticker that periodically tests endpoint health to allow zero-latency routing lookups.
- [LatencyProber](LatencyProber.md) — Connection-pooled latency prober that avoids allocating a new reqwest::Client on each call.

## Functions

- [default](default.md)
- [default](default_1.md)
- [get_prober](get_prober.md)
- [is_primary_healthy](is_primary_healthy.md) — [inline(always)]
- [is_primary_healthy](is_primary_healthy_1.md) — [inline(always)]
- [is_reachable](is_reachable.md) — Returns `true` if the endpoint is reachable within `timeout_ms`.
- [is_reachable](is_reachable_1.md) — Returns `true` if the endpoint is reachable within `timeout_ms`.
- [is_reachable](is_reachable_2.md)
- [is_secondary_healthy](is_secondary_healthy.md) — [inline(always)]
- [is_secondary_healthy](is_secondary_healthy_1.md) — [inline(always)]
- [new](new.md)
- [new](new_1.md)
- [new](new_2.md)
- [new](new_3.md)
- [primary_latency](primary_latency.md)
- [primary_latency](primary_latency_1.md)
- [probe_latency](probe_latency.md) — Probe the online API endpoint for reachability and measure round-trip latency.
- [probe_latency](probe_latency_1.md) — Probe the online API endpoint for reachability and measure round-trip latency.
- [probe_latency](probe_latency_2.md) — Backwards compatible functions using the pooled prober
- [prober](prober.md)
- [prober](prober_1.md)
- [secondary_latency](secondary_latency.md)
- [secondary_latency](secondary_latency_1.md)

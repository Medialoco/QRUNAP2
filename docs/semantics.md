# Runtime semantics

These are explicit QRUNAP2 0.1 policies. Historical equivalence is only claimed to the extent supported in the [inventory](language-inventory.md).

## Events and customers

The clock starts at zero. Initial customers enter their stations in declaration order. Sources schedule their first arrival after one sampled interarrival time. A source creates one customer at each arrival and schedules the next. Service completion routes a customer immediately to its destination; OUT removes it. FIFO servers take the oldest waiting customer, sharing one queue when capacity exceeds one.

Events use a min-priority calendar ordered by time then insertion serial number. At a service completion, the next queued service is scheduled before the completed customer is routed. Events exactly at TMAX are processed. No time beyond TMAX is measured. Customers still present at the horizon are retained in the remaining count, rather than drained. Positive delays that fail to advance floating-point time are errors.

## Randomness

Seed is a nonzero 32-bit integer. One shared xorshift32 stream uses shifts 13, 17, 5; each exponential draw uses `u=(state+0.5)/2^32` and `-mean*ln(u)`. Constant services do not draw. The same model and seed reproduce a run on the same build; native/WASM values are checked within floating-point tolerance. Cross-platform logarithm rounding can differ. This is a new generator policy, not reproduction of historical random streams.

## Measurements

All times use the model's chosen common unit. No warm-up period is discarded.

| Report | Definition |
| --- | --- |
| mean_number (L) | Integral of waiting plus in-service customers, divided by TMAX |
| mean_queue (Lq) | Integral of waiting customers only, divided by TMAX |
| utilization | Integral of busy servers, divided by capacity × TMAX |
| throughput | Completed visits / TMAX; source rows report generated arrivals / TMAX |
| mean_wait (Wq) | Waiting time of completed station visits / completed visits |
| mean_response (W) | Waiting plus service time of completed station visits / completed visits |
| remaining | Customers still queued or in service at TMAX |

W and Wq are null if there are no completed visits. Source utilization and residence times have no queueing meaning and are displayed as dashes. A customer visiting a closed-network station repeatedly contributes one observation per completed visit. These are station response times, not whole-network journey times.

Completed-visit averages omit unfinished visits, so finite-horizon results can be biased. Little's law is a steady-state consistency guide, not an exact finite-run equality between differently censored estimators. A trace samples network population at 201 regular points; it is not a complete event log. The conservation identity `initial + generated = exited + remaining` is exact.

## Analytical teaching reference

For one FIFO server with independent exponential interarrivals of mean a and services of mean s, λ=1/a, μ=1/s and ρ=s/a. If ρ<1: L=ρ/(1−ρ), Lq=ρ²/(1−ρ), W=1/(μ−λ), Wq=W−s. The first example has a=2, s=1, hence ρ=0.5, L=1 and W=2. These are mathematical reference values, not a historical QNAP2 solver. When ρ≥1 there is no finite stationary M/M/1 mean.

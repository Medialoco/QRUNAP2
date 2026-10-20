# Compatibility and adaptation

QRUNAP2 0.1 is an educational Rust interpreter for a small, evidence-linked QNAP2 subset. It is not a replacement for the complete historical environment. The [inventory](language-inventory.md) separates evidence from runtime policy.

| Area | Historical QNAP2 | QRUNAP2 0.1 |
| --- | --- | --- |
| Model text | Stations, customers, algorithms and control sections | Historical section spelling; numeric literals and deterministic routes only |
| Execution | Simulation and multiple solution methods | Discrete-event simulation only |
| Stations | Rich types, disciplines and synchronization | Sources, single/multiple servers, FIFO, initial populations |
| Programming | Variables, expressions, procedures, loops, flags and references | Not implemented; explicit error |
| Randomness | Historical generator and stream rules | Seeded xorshift32; no stream equivalence claimed |
| Statistics | Historical controls, replication and precision facilities | Time averages and completed-visit averages; no warm-up removal or confidence intervals |
| Interface | Historical interactive commands | New CLI and English/French browser playground |
| Distribution | Historical runtime | Same Rust core compiled natively and to WebAssembly |

## No silent migration

An unsupported construct is rejected. In particular, an original program using PRINT, GET, variables or parallel replications does not silently run with those instructions removed. The bundled historical adaptation has an explicit change record in [sources](sources.md).

## Limits

One QUEUE declaration; at most 128 stations; capacities 1–1,024; source text up to 64 KiB; population up to 100,000; at most 1,000,000 events per run. Durations and TMAX must be positive and no greater than 1e9. Only decimal literals, no exponent notation. No zero-time service, probabilistic branching, customer classes, variable assignment, arrays, priorities, preemption, processor sharing, infinite servers, nested SERVICE code, file access or analytical solver.

Limits produce errors, not plausible-looking partial results. The browser can terminate a worker at any time. Model errors include source locations where detected; post-parse consistency errors name the relevant station when available.

## Verification scope

Tests cover deterministic event counts, occupancy integration, population conservation, closed-network population, fixed-seed replay, multiple-server effects, invalid input and resource limits. Eight independent long M/M/1 runs are compared with theoretical means using documented broad regression tolerances. These tests establish internal and mathematical checks for this subset; they do not certify QNAP2 9.3 conformance.

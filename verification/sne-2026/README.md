# QRUNAP2 verification package — 20 October 2026

This package documents the behavior of the **0.1.0 simulation engine**. It is a software verification artifact, not a preprint. No manuscript is included.

## What was checked

1. Seven Rust integration tests in `tests/runtime.rs`.
2. Native/WebAssembly report agreement on five examples at seed 42 (`scripts/test-wasm.mjs`), numerical tolerance `1e-8 * max(1, abs(native))`.
3. Eight deterministic finite-horizon models compared to separately implemented SimPy Resource/process models; counters exact, numerical tolerance `1e-10 * max(1, abs(native))`.
4. Six stochastic configurations, 32 trajectories per engine, horizon 200,000: four M/M/1 arrival rates, M/M/2 and a symmetric closed two-station network of 40 customers. Both sets of 192 reports are included.

The code in `src/` is unchanged from commit `bd708b41d7c30bdaaf9a4499a2d659d9de2f8254` (release v0.1.0). This package adds verification material and an MIT licence; it does not change simulation semantics. The `engine-sha256.json` file records the source hashes used.

## Reproduce

From the repository root, using Rust/Cargo and Python 3:

```sh
cargo test --locked
python3 verification/sne-2026/run_native.py
python3 -m venv .verification-venv
.verification-venv/bin/python -m pip install -r verification/sne-2026/requirements.txt
.verification-venv/bin/python verification/sne-2026/reference_simpy.py
```

For just the quick deterministic cross-engine tests, pass `--deterministic-only` to `reference_simpy.py`. First run `cargo build --release --locked` to provide the native executable. Stochastic reference runs take substantially longer than the native Rust runs.

For the WebAssembly checks, build the site as described in the repository README, then run:

```sh
node scripts/test-wasm.mjs
```

The numerical script overwrites its generated data directory. Original results, environment metadata and checksums are available in the versioned package. Re-running under another platform or dependency set may change floating-point rounding; do not silently replace the original artifact when reviewing it.

## Files

- `run_native.py`: six native configurations and analytical formulas, standard-library Python.
- `native-results/`: models, 192 complete reports, CSV measurements, descriptive summaries and environment metadata.
- `reference_simpy.py`: separate SimPy 4.1.2 model implementation; it does not call the Rust parser, calendar, measurements or random generator.
- `reference-results/`: eight exact comparison records and model files, 192 stochastic reports, descriptive summaries and environment metadata.
- `requirements.txt`: pinned reference simulator version.
- `engine-sha256.json`: source-level provenance.
- `SHA256SUMS`: package file checksums, excluding this manifest itself.

## Interpretation and limits

M/M/1 uses service rate 1 and arrival rates 0.25, 0.50, 0.80 and 0.95. M/M/2 uses arrival rate 4/3 and service rate 1 per server. The closed network uses two single servers with mean service time 1 and starts with 40 customers in A. The native script computes the stationary references independently of the engine. All configurations use an inclusive horizon, no draining and no warm-up removal.

QRUNAP2 uses one xorshift32 stream. SimPy reference models use Python `random.Random` (Mersenne Twister) with exponential sampling. Seeds 1–32 are used for each engine. Equal seeds are **not paired random trajectories**, and neither set is claimed to establish independent substreams. Summary standard deviations are descriptive, not confidence intervals or equivalence tests. Statistics refer to completed station visits and time integrals, not end-to-end network response.

The eight deterministic cases agree at the stated tolerances. This does not prove agreement for every event tie or network. The numerical comparisons and conservation checks support only the cases tested. Both models were prepared in the same development effort with AI assistance; a separate engine does not mean an independent human audit.

No original QNAP2 executable was used. The historical example is an explicitly documented reconstruction. No claim of full historical-language compatibility, superior performance or measured learning benefit follows from these experiments.

## Licence

The original code and verification material are covered by the repository's MIT licence. SimPy and cited historical material retain their respective licences and rights.

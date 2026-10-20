# QRUNAP2

[![crates.io](https://img.shields.io/crates/v/qrunap2.svg)](https://crates.io/crates/qrunap2)

A documented QNAP2 subset for queueing network simulation.

An independent Rust reconstruction of a small, evidence-linked subset of the historical QNAP2 queueing-network language. English first, with a French learning guide and interface.

- [Simulation interface](https://medialoco.github.io/QRUNAP2/)
- [Rust package on crates.io](https://crates.io/crates/qrunap2) · [API documentation](https://docs.rs/qrunap2)
- [English user guide](docs/guide.md) · [Guide français](docs/guide.fr.md)
- [Compatibility](docs/compatibility.md) · [Sources](docs/sources.md) · [Language inventory](docs/language-inventory.md)
- [Grammar](docs/grammar.md) · [Runtime semantics](docs/semantics.md)
- [Original research plan](QRUNAP2_PLAN.md)

## What works

Five editable examples: M/M/1, constant durations, two servers, overload, and a reconstruction of an INRIA 1995 closed-network example. The same Rust interpreter runs natively and as WebAssembly. No simulation backend, third-party JavaScript CDN or visitor account is needed. Unsupported language features produce errors.

This is **v0.1, an educational subset**, not full QNAP2 9.3 compatibility. Historical programs are not generally drop-in compatible. No historical RNG equivalence or confidence intervals are claimed. The complete 9.3 manual remains to be recovered.

## Run locally

### Install the command-line tool

```sh
cargo install qrunap2 --version 0.1.1 --locked
qrunap2 --version
qrunap2 check model.qnp
qrunap2 run model.qnp 42
```

`model.qnp` is a local model file. Download a [complete M/M/1 example](https://raw.githubusercontent.com/Medialoco/QRUNAP2/main/examples/original/first-queue.qnp)
or use the examples in this repository. `run` writes a JSON report; the seed defaults to `42` if omitted.

### Use the Rust library

```sh
cargo add qrunap2
```

```rust,no_run
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let source = std::fs::read_to_string("model.qnp")?;
    let report = qrunap2::run(&source, 42)?;
    println!("Simulated {} events", report.events);
    Ok(())
}
```

The library exposes `parse`, `simulate` and `run`, together with model and report types.
See the [API documentation](https://docs.rs/qrunap2).

### Build from source

```sh
cargo test --locked
cargo run -- check examples/original/first-queue.qnp
cargo run -- run examples/original/first-queue.qnp 42
rustup target add wasm32-unknown-unknown
python3 scripts/build.py
python3 -m http.server 8000 --directory dist
```

Open http://localhost:8000. Use HTTP rather than opening index.html directly: the worker and WASM are fetched as local web assets. Requires Rust/Cargo, Python 3 and Node.js for the WASM smoke test. Rust dependencies are pinned by Cargo.lock; the web interface has no package dependencies.

```sh
node scripts/test-wasm.mjs
```

GitHub Actions builds, tests and deploys to Pages on pushes to main. Repository Settings → Pages must use **GitHub Actions** as the build source.

## Layout

`src/syntax.rs` parses and validates the supported language; `src/engine.rs` runs events and statistics; `src/lib.rs` exposes the shared native/WASM core; `web/` contains the bilingual interface; `docs/` contains sources and explanations. The build generates documentation HTML from Markdown and copies the examples.

## Provenance and licensing

QRUNAP2 is an independent implementation, not an official INRIA, Bull or Simulog release. Historical documents remain with their respective rights holders. The historical example is labelled as an adaptation. The original QRUNAP2 code is available under the MIT License; this does not relicense historical source material.

## License

QRUNAP2 code, original examples, documentation and verification scripts are available under the [MIT License](LICENSE). Historical documents and third-party software retain their respective rights and licences; citing them does not relicense them.

[Reproducible verification package](verification/sne-2026/README.md): analytical checks, eight exact SimPy comparisons, and 192 stochastic trajectories per engine.

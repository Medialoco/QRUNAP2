# QRUNAP2

**Remember QNAP2. Edit a model. Run it in your browser.**

An independent Rust reconstruction of a small, evidence-linked subset of the historical QNAP2 queueing-network language. English first, with a French learning guide and interface.

- [Live playground](https://medialoco.github.io/QRUNAP2/)
- [English learning guide](docs/guide.md) · [Guide français](docs/guide.fr.md)
- [Compatibility](docs/compatibility.md) · [Sources](docs/sources.md) · [Language inventory](docs/language-inventory.md)
- [Grammar](docs/grammar.md) · [Runtime semantics](docs/semantics.md)
- [Original research plan](QRUNAP2_PLAN.md)

## What works

Five editable examples: M/M/1, constant durations, two servers, overload, and a reconstruction of an INRIA 1995 closed-network example. The same Rust interpreter runs natively and as WebAssembly. No simulation backend, third-party JavaScript CDN or visitor account is needed. Unsupported language features produce errors.

This is **v0.1, an educational subset**, not full QNAP2 9.3 compatibility. Historical programs are not generally drop-in compatible. No historical RNG equivalence or confidence intervals are claimed. The complete 9.3 manual remains to be recovered.

## Run locally

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

QRUNAP2 is an independent implementation, not an official INRIA, Bull or Simulog release. Historical documents remain with their respective rights holders. The historical example is labelled as an adaptation. A license for the new code has not yet been selected; a public repository alone does not grant an open-source license.

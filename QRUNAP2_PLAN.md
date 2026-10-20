# QRUNAP2 — Project plan for Codex

> **Project:** QRUNAP2 — A Rust revival of the QNAP2 queueing-network modelling language  
> **Status:** Research and implementation plan, 19 October 2026  
> **Goal:** Independently reimplement the documented QNAP2 language and modelling semantics in Rust, preserving original concepts, names, and syntax wherever evidence permits; add modern tooling without silently changing historical semantics.

## 0. Instruction to Codex: read this first

QRUNAP2 is **not** a new queueing DSL loosely inspired by QNAP2. It is an **evidence-driven reconstruction** of the historical QNAP2 modelling language and execution environment (INRIA / Simulog). Maintain a strict distinction between:

1. **Documented QNAP2 behavior** (cite source, version, page/section),
2. **Independently corroborated behavior** (real programs, papers, archived output),
3. **Hypotheses requiring validation**, and
4. **QRUNAP2-only extensions** (explicitly named and versioned; preferably opt-in).

Do **not** invent keywords, grammar, defaults, distribution parameterizations, queue disciplines, solver names or semantics. Examples in this document are research fixtures, not a complete authoritative grammar. Do not claim complete QNAP2 compatibility until it has been tested.

**First Codex task:** collect primary sources; produce `docs/sources.md`, `docs/language-inventory.md`, and a source-linked compatibility matrix before implementing a full parser or simulator.

## 1. Scope and historical rationale

QNAP2 (*Queueing Network Analysis Package 2*) was an environment for constructing and solving queueing-network models, developed in the French INRIA research ecosystem and distributed commercially by Simulog. It combined an algorithmic model-description language (Pascal/Simula lineage), command/control language, queueing-network objects, discrete-event simulation, and analytical / approximate solution techniques. It is important not to reduce the historical system to `SOURCE -> QUEUE -> SINK`: stations could execute algorithms, create and route customers, and synchronize processes.

This reconstruction must preserve three separate layers:

- **Language:** original source form, declarations, expressions, instructions, built-in objects and commands.
- **Semantics:** queues, customers, stations, scheduling, distribution draws, synchronization, routing, statistics and solver applicability.
- **Analysis engines:** discrete-event simulation first; analytical, approximate, and Markovian methods later, when identified precisely in the manuals.

### Principles

- Original QNAP2 terms in code and documentation; do not translate names into a fashionable new DSL.
- Deterministic simulations where a seed, event tie-breaking and random-stream policy are specified and controlled.
- Separation of parser, semantic model, execution runtime, statistical analysis and solvers.
- Small, testable vertical slices and gold-standard reference examples.
- Rust workspace, open source, documented limitations, portable CLI.
- No forced GUI, Python, distributed execution or WASM in the MVP.

## 2. Source catalogue (read in this order)

**S1 — PRIMARY: Simulog, _QNAP2 Version 9.3 User's Guide_ (1996).** Historical language specification and examples. Web transcription may have formatting errors; where possible locate the original scan and verify symbols, punctuation, tables and page numbers.

- https://manualzilla.com/doc/6892832/qnap2-version-9.3-user-s-guide
- Also check alternative mirror (availability not guaranteed): https://manualzz.com/doc/4169276/simulog-qnap2-software-user-s-guide

**S2 — PRIMARY foundational research: M. Véran and D. Potier, _QNAP2: A Portable Environment for Queueing Systems Modelling_, INRIA Research Report RR-0314 (1984).** Explains goals and original design; historical version differs from v9.3. Publication metadata and downloadable manuscript:

- https://hal.inria.fr/inria-00076243
- https://www.researchgate.net/publication/239535208_QNAP_2A_portable_environment_for_queueing_systems_modelling

**S3 — Published programs: L. Pasini and S. Feliziani, _Networks of Queues for the Simulation of Urban Flow Systems: An Intermodal Traffic System_ (2004).** Includes original QNAP2 sections using `/DECLARE/`, `/STATION/`, `QUEUE`, `FLAG`, `REF CUSTOMER`, `TYPE=SOURCE`, `TYPE=SERVER,MULTIPLE(20)`, `SERVICE`, `TRANSIT`, `WAIT`, `UNSET`, and `EXP`. Use as a verification corpus, checking its transcription.

- https://www.researchgate.net/publication/242279825_NETWORKS_OF_QUEUES_FOR_THE_SIMULATION_OF_URBAN_FLOW_SYSTEMS_AN_INTERMODAL_TRAFFIC_SYSTEM

**S4 — Teaching material: Carleton University QNAP2 short guide and example output.** Potentially useful independent reference; **links were not accessible to automated verification at plan creation**. Codex should retry, find mirrors or use archived copies; do not assume the content is present.

- https://www.sce.carleton.ca/courses/94511/handouts03/qnap-instructions/ShortGuide
- https://www.sce.carleton.ca/courses/94511/handouts03/qnap-instructions/ex2.out
- Parent directory: https://www.sce.carleton.ca/courses/94511/handouts03/qnap-instructions/
- Internet Archive: https://web.archive.org/

**S5 — QNAP2 models in dissertations and literature.** Contains an appendix with a real model and description of command-line invocation; inspect appendices for executable fixtures.

- https://bradscholars.brad.ac.uk/bitstreams/4236c400-beb6-4a1d-9f5c-50db1576aad9/download

**S6 — Independent retrospective comparison: _Specification and Simulation of Queuing Network Models using Domain-Specific Languages_.** Helps situate QNAP2 among other tools; *not* a source of authoritative QNAP2 grammar.

- https://www.researchgate.net/publication/259868027_Specification_and_Simulation_of_Queuing_Network_Models_using_Domain-Specific_Languages

**S7 — Background about language architecture.** Historical technical overview that describes a control language plus an algorithmic language; source may be thesis/report and is secondary to the user guide.

- https://citeseerx.ist.psu.edu/document?doi=857f0bd7788efe11a07da6155a1e36595ca6b811&repid=rep1&type=pdf

### Modern comparators and tools (for later evaluation; not syntax authorities)

- SimPy: https://simpy.readthedocs.io/
- Ciw: https://ciw.readthedocs.io/
- JMT (Java Modelling Tools): https://jmt.sourceforge.net/
- OMNeT++: https://omnetpp.org/
- Rust documentation: https://doc.rust-lang.org/book/
- Cargo workspaces: https://doc.rust-lang.org/cargo/reference/workspaces.html
- Criterion benchmarks: https://bheisler.github.io/criterion.rs/book/
- proptest: https://proptest-rs.github.io/proptest/intro.html
- Cargo fuzz: https://rust-fuzz.github.io/book/cargo-fuzz.html
- mdBook: https://rust-lang.github.io/mdBook/
- WSC: https://www.wintersim.org/
- ACM TOMACS: https://dl.acm.org/journal/tomacs
- Journal of Simulation: https://www.tandfonline.com/journals/tjsm20
- SIMULATION (SAGE): https://journals.sagepub.com/home/sim

**Access and provenance:** Record for every source its URL, authors, year, QNAP2 version, date accessed, retrieval outcome, local file hash if legally archived, and rights/reuse note. Cite page or section for each reconstructed feature. Broken links are research tasks, not proofs that a feature does not exist.

## 3. Language and feature inventory to reconstruct

The following are **research targets**, not promises of complete support. Verify exact spelling, syntactic position, default value, parameter order and runtime semantics against S1 before marking them supported.

| Domain | Historical tokens / constructs to investigate | Questions for the spec |
|---|---|---|
| Source sections | `/DECLARE/`, `/STATION/`, `/EXEC/`, `/CONTROL/` | Which section names actually exist in v9.3? Required ordering? Optionality? |
| Types and objects | `QUEUE`, `CUSTOMER`, `REF`, `FLAG`, `INTEGER`, arrays and references | Lifetime, reference/null semantics, attribute lookup, ownership |
| Station definition | `NAME`, `TYPE`, `SOURCE`, `SERVER`, `SINGLE`, `MULTIPLE`, `SERVICE`, `TRANSIT` | Queue vs station identity, service execution, scheduling |
| Algorithmic language | `BEGIN`, `END`, `IF`, `THEN`, `ELSE`, `FOR`, `STEP`, `UNTIL`, `DO`, `NEW`, `:=` | Precedence, scope, iteration order, side effects |
| Routing / synchronization | `TRANSIT(...)`, `WAIT(...)`, `UNSET(...)` | Blocking behavior, wake-up and event order |
| Random variables | `EXP(...)`; then inventory all documented distributions | Is the argument mean, rate or another convention? RNG behavior |
| Queueing disciplines | FIFO, LIFO, priority, processor-sharing etc. | **Verify actual keywords and exact semantics** before implementing |
| Solution methods | discrete-event simulation; analytical/approximate/Markov methods | **Extract exact command names and applicability** from manual |
| Results and statistics | attributes, `PRINT`, queue statistics, confidence intervals | Definitions, warmup, run termination, estimators |
| Embedded control | compilation, execution, repeated runs, parameter sweeps | Side effects, commands vs algorithmic expressions |

### Historically attested code fragments — **not a complete executable model**

From the 2004 published program (S3; excerpted only to establish genuine spelling):

```text
/DECLARE/
QUEUE A, B, C, D, INSCALA, SCALA;
FLAG FERMO;
REF CUSTOMER RC;
INTEGER N;

/STATION/
NAME=A;
TYPE=SOURCE;
SERVICE=IF (TIME>=T1) AND (TIME<T2)
        THEN EXP(0.32)
        ELSE EXP(2.037);
TRANSIT=B;

/STATION/
NAME=B;
TYPE=SERVER,MULTIPLE(20);
```

These fragments omit declarations and subsequent sections. **Do not insert them as an executable passing test** until the source has been completed and checked. The parameterization of `EXP` must be determined from S1 rather than guessed.

## 4. Proposed repository layout

```text
qrunap2/
  README.md
  LICENSE                     # only after choosing and confirming a license
  Cargo.toml                  # workspace
  rust-toolchain.toml
  AGENTS.md                   # Codex instructions, source-first methodology
  docs/
    sources.md                # traceability, versions, hashes, access status
    language-inventory.md     # authoritative feature inventory + citations
    grammar.md                # grammar reconstructed from sources
    semantics.md              # event scheduling / queue / statistics / RNG
    compatibility.md          # feature-by-feature implementation status
    architecture.md
    research-notes.md         # unresolved questions and deviations
    publication.md
  crates/
    qrunap2-syntax/          # lexer, parser, source spans, AST
    qrunap2-semantics/       # name/type checking and lowered IR
    qrunap2-engine/          # deterministic DES kernel, event calendar
    qrunap2-model/           # customers, stations, queues, distributions
    qrunap2-stats/           # estimators, confidence intervals, reporting
    qrunap2-solvers/         # analytical/approximate methods (later)
    qrunap2-cli/             # qrunap2 command
  examples/
    historical/              # attributable and redistributable examples
    original/                # independently authored equivalent models
  tests/
    fixtures/
    golden/
    analytical/
    property/
  benches/
  scripts/
```

Avoid excessive fragmentation in the first iteration: start with `syntax`, `engine`, `model`, `cli`; split further when needed.

## 5. Semantic and runtime design

- **Lexer/parser:** faithful syntax including case conventions, comments, semicolon rules, source locations, error recovery. AST keeps historical names; do not silently normalize or rewrite unsupported constructs.
- **Semantic analysis:** name resolution; station declarations; type constraints; references; input diagnostics; lower to versioned IR.
- **Discrete-event engine:** virtual simulation clock, min-priority event calendar, stable deterministic ordering for simultaneous events, explicit queue and customer IDs, event causality and termination semantics.
- **Station execution:** `SOURCE` arrivals; `SERVER` capacity, queues and service completion; `TRANSIT`; station algorithms and synchronization once supported.
- **Randomness:** seeded independent streams (policy documented); distribution samples; sampling and seeding tests; preserve historical distribution **parameterization**, while documenting that reproducing historical pseudorandom streams may be impossible without original algorithm.
- **Statistics:** time-weighted occupancy, throughput, utilization, residence time, waiting time, dropped/blocked customers if historically supported; confidence intervals and warm-up only when semantics are well defined.
- **Diagnostics:** lexer/parser failures include file and span; unsupported historical features are explicit errors (not skipped).
- **CLI:** `qrunap2 check FILE`, `qrunap2 run FILE`, `qrunap2 --version`; treat these command names as **new CLI design**, not historical QNAP2 commands.
- **Output:** human report first, machine-readable JSON later; schema version and seed in metadata.
- **Solvers:** implement simple analytical baselines independently; only expose QNAP2 historical solver commands after the grammar and applicability conditions have been established.

## 6. Implementation roadmap and exit criteria

### M0 — Evidence package and spec skeleton (first milestone)

- [ ] Collect S1, S2 and S3; preserve URLs and metadata.
- [ ] Extract table of contents, glossary, station types, language keywords, commands and example directory from S1.
- [ ] Produce `docs/language-inventory.md` with `feature | syntax | version | source page | semantics | confidence | status`.
- [ ] Transcribe **at least three** complete models (not partial fragments) with provenance and rights annotations.
- [ ] Write test matrix differentiating parser acceptance, simulated numeric validation and uncertain behavior.
- [ ] Record license/IP/trademark questions; do not redistribute full proprietary manuals.

**Done when:** Codex can point to a primary source for every token planned for M1 and identifies any outstanding uncertainties.

### M1 — Parser and semantic model

- [ ] Establish Rust workspace, CI, formatting/lint/tests.
- [ ] Build lexer and parser for a narrowly specified subset of declarations and stations.
- [ ] Parse historical examples with stable AST snapshots and source spans.
- [ ] Build an initial type/name validator and IR; reject unsupported constructs precisely.
- [ ] Document known mismatches in compatibility matrix.

**Done when:** three complete historical or historically faithful models parse, have semantic checks, and pass regression fixtures without unverified extensions.

### M2 — Minimum discrete-event simulator

- [ ] Event calendar with specified tie-breaking policy.
- [ ] Source → server → exit for a documented open network.
- [ ] Server capacity and FIFO *if confirmed by source*; seeded exponential times with confirmed parameter convention.
- [ ] Collect queue length, utilization, throughput and response time.
- [ ] Unit tests for event causality and conservation of customers.

**Done when:** simple queue models execute reproducibly; empirical means converge to theoretical references within documented statistical tolerances.

### M3 — Historical language behaviors

- [ ] Multiple servers, routing and multi-class customers.
- [ ] Algorithmic `SERVICE` bodies and dynamic `TRANSIT` as verified.
- [ ] Object references, customer creation, flags and synchronization.
- [ ] Parameterized examples, termination, control commands.
- [ ] Extended validation against additional historical published programs.

**Done when:** nontrivial historically attested models run, with known limitations explicitly listed.

### M4 — Multiple analysis engines

- [ ] Implement and validate a mathematically scoped basic solver (for example M/M/1, M/M/c or Jackson networks as applicable).
- [ ] Compare DES vs analytical solutions on the **same normalized model**.
- [ ] Add the QNAP2-specific analytical/approximate solution methods based on manual evidence.
- [ ] Detect unsupported solver/model combinations, never return plausible-looking unsupported answers.

**Done when:** paper-ready numeric verification includes confidence intervals and documented assumptions.

### M5 — Modern ergonomics (optional; after scientific core)

- [ ] JSON/CSV report, library Rust API, Python bindings if demand exists.
- [ ] WASM runtime for browser examples (mind RNG/clock and file I/O restrictions).
- [ ] Benchmarks, documented reproducibility bundle, visualizations.
- [ ] Compatibility-focused documentation site and migration guide.

## 7. Test and verification strategy

**Three independent test classes:**

1. **Language conformance:** inputs from S1 and published programs: accepted/rejected grammar, name resolution, evaluation semantics, station definitions.
2. **Mathematical validity:** M/M/1 reference when `rho = lambda/mu < 1`: `L = rho/(1-rho)`, `W = 1/(mu-lambda)`, `Lq = rho^2/(1-rho)`, `Wq = rho/(mu-lambda)` (under conventional exponential *rate* parameterization). **Translate rates to QNAP2 `EXP` arguments only after consulting S1.** Include Little's law `L=lambda W` and network flow conservation.
3. **Reproducibility and performance:** fixed seed repeatability; stable event ordering; no negative event delays; memory usage, events/second and benchmarks vs SimPy/Ciw for equivalent models and comparable instrumentation.

Test run-to-run determinism separately from matching historic QNAP2's random number stream. For stochastic means, use independent replications, error estimates and statistical tolerance, not exact equality. Add parser fuzzing and property-based tests.

Recommended provenance for each test: `source`, `manual_page`, `model_file`, `behavior`, `expected_result`, `tolerance`, `independent_crosscheck`, `limitations`.

## 8. Scientific dissemination

Possible working paper title: **“QRUNAP2: Reconstructing the QNAP2 Queueing-Network Simulation Language in Rust.”**

Contributions to *earn* (do not claim before demonstrated):

- Faithful, source-traceable reconstruction of a historic queueing-network DSL.
- Validated DES implementation with documented fidelity and compatibility limits.
- Shared modelling IR supporting both stochastic simulation and analytical methods.
- Open, reproducible benchmark and historical model corpus (subject to rights).
- Modern performance-engineering applications, not just historical preservation.

Publication plan: release a documented repository + reproducibility package; write a technical preprint once real results exist; then consider Winter Simulation Conference, *ACM Transactions on Modeling and Computer Simulation (TOMACS)*, *Journal of Simulation*, or *SIMULATION*, choosing venue based on actual contribution. A port alone is insufficient for a strong simulation-methodology paper: provide model semantics, empirical validation, meaningful benchmarks and a distinctive technical contribution.

Suggested paper outline: background and history; corpus/provenance; reconstructed grammar/semantics; Rust architecture; verification vs theory and historical references; benchmarks; limitations/IP; outlook.

## 9. Legal, ethical and project-name checklist

- Historical QNAP2 documentation is **available online** but not thereby open licensed. Do not paste an entire manual into the repository or use its proprietary implementation code without rights.
- Pursue independent implementation from documented behavior; attribute the INRIA/Simulog designers and distinguish QRUNAP2 from the original product.
- Check any surviving Simulog/Inria rights and trademarks and whether **QRUNAP2** collides with an existing project/name; an informal web search is not trademark clearance.
- Review copyright status of published model examples before redistributing them; otherwise retain links and make original test models.
- License QRUNAP2's **new code** under MIT or Apache-2.0 (or dual license) only after deciding the policy; do not imply the original QNAP2 is under that license.

## 10. First concrete Codex prompt

```text
You are building QRUNAP2, an independent Rust reconstruction of the
historical INRIA/Simulog QNAP2 language. Read QRUNAP2_PLAN.md completely.

FIRST ITERATION ONLY (M0):
1. Inspect the primary QNAP2 9.3 user's guide and INRIA RR-0314, plus
   published historical QNAP2 programs linked in the plan.
2. Create docs/sources.md with source IDs, version, URLs, access status,
   bibliographic data and usage/right-to-redistribute notes.
3. Create docs/language-inventory.md as a citation-backed table of actual
   documented sections, keywords, types, stations, attributes, distributions,
   routing/synchronization instructions and solver commands.
4. Create docs/grammar.md with only verified productions and explicit TBDs.
5. Find at least three complete historical model examples or explain exactly
   why fewer are available; do not invent missing code.
6. Create docs/compatibility.md with unsupported and unclear semantics.
7. Propose a minimal Rust workspace and M1 implementation issues, but do
   not build a broad simulation framework yet.

Rules: never substitute a new syntax for an original term; record exact
manual pages/sections; separate documented facts from assumptions; do not
copy proprietary source code or full manuals; report inaccessible URLs.
```

## 11. Decisions to retain

- **Project name:** QRUNAP2 (working name; check clearance).
- **Implementation:** Rust.
- **Language philosophy:** historical QNAP2 spelling and naming first.
- **Priorities:** source acquisition → verified subset → faithful runtime → comparison/solvers → ergonomic extensions.
- **Success criterion for v0.1:** a documented subset of historical QNAP2 models parses and executes reproducibly in Rust with independently validated results; incompatibilities are clearly stated.

---

**Research note:** This plan references historical primary material but is not itself a full language specification. Preserve uncertainty rather than generating an attractive, incorrect pseudo-QNAP2.

## 12. Implemented educational slice — 19 October 2026

Following the owner's request, browser delivery is brought forward from M5. English is the default documentation/UI language, with a French guide and interface. Target: `Medialoco/QRUNAP2`, managed using `thepriben`.

A small Rust parser and DES engine now share native and WASM builds, with five editable browser examples. See README.md and docs/guide.md for the entry points. This does not complete M0–M4: one complete historical source was inspected, not three; full v9.3 manuals remain inaccessible; source-level uncertainties are recorded in docs/sources.md and docs/language-inventory.md. No broad compatibility claim is made. The original research priorities above remain applicable to expanding the subset.

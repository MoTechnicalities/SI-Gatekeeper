# Contributing to SI-Gatekeeper

Thank you for your interest in contributing to **SI-Gatekeeper**.

> *"Open-sourced by the people for the people of the world."*

## Core Architectural Principles for Contributors

To preserve deterministic replayability across platforms and languages, all contributions must adhere to these rules:

1. **Integer-Exact Evaluation (`i32` / Checked Arithmetic):**
   - Do not introduce floating-point types (`f32`, `f64`) or non-deterministic random number generators into coordinate derivation, gate evaluation, or resonance scoring.
   - Use checked integer arithmetic (`checked_add`, `checked_sub`, `checked_mul`, `checked_abs`) so overflow conditions fail explicitly rather than wrapping silently.
2. **Separation of Generator and Governor:**
   - Candidate generators (whether human-authored, search-based, or LLM-proposed) must never be able to mutate a `Constitution` or `ResonanceField` during evaluation.
3. **Explicit Provenance & Attestation Boundaries:**
   - External empirical or legal facts must be supplied via `ExternalFactAttestation` and never conflated with internal geometric validity.
4. **Canonical Signature Stability:**
   - Any change to an existing domain tournament or demonstration must either preserve its canonical replay signature or explicitly document and justify the versioned signature change in tests.

## Development & Verification Workflow

Before opening a Pull Request, verify that all checks pass locally:

```bash
# 1. Lint with zero warnings
cargo clippy --all-targets -- -D warnings

# 2. Run all unit and integration tests
cargo test

# 3. Verify all standalone binaries
cargo run --bin si_gatekeeper
cargo run --bin compliance_tournament
cargo run --bin infrastructure_arbitration
cargo run --bin legal_reasoning_trace
cargo run --bin self_improvement_gate

# 4. Verify Python-to-Rust signature equivalence
python3 examples/prototype_compliance.py
```

## License

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in `SI-Gatekeeper` by you, as defined in the Apache-2.0 license, shall be dual-licensed under `MIT OR Apache-2.0`, without any additional terms or conditions.

# Security Policy & Threat-Model Scope

## Threat Model & Architectural Boundaries

`SI-Gatekeeper` is a **deterministic constitutional verification and arbitration harness**. Understanding what it guarantees—and what remains the responsibility of the integrating system—is essential for secure deployment:

1. **Deterministic Replayability vs. Oracle Truth:**
   - `SI-Gatekeeper` guarantees that for any declared `Constitution`, `ExternalFactAttestation`, and `ObservableFeatures`, the gate verdict, fallback selection, and canonical receipt signature (`gatekeeper:v1|...`) are 100% reproducible without floating-point drift.
   - `SI-Gatekeeper` is **not** an omniscient oracle of external truth. If an untrusted or compromised component supplies falsified `ObservableFeatures` or a forged `ExternalFactAttestation`, the gatekeeper will deterministically evaluate those falsified inputs.
2. **Mitigating Input Gaming in Production:**
   - **Independent Feature Extraction:** The candidate generator must never be allowed to self-report its own `ObservableFeatures` (`intermediaries`, `rights`, `duties`, `protection`, `exposure`). Feature counts must be computed by an independent static analyzer, telemetry monitor, or regulatory auditor.
   - **Cryptographic Attestation:** In production deployments, `ExternalFactAttestation` records and `Constitution` manifests should be cryptographically signed by authorized institutional keys or hardware sensors before being passed into `ConstitutionalGatekeeper::evaluate_candidate`.
   - **Checked Integer Bounds:** All arithmetic inside the gatekeeper uses checked `i32` operations (`ArithmeticError`) to prevent integer overflow/underflow attacks from bypassing gate predicates.

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.2.x   | :white_check_mark: |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

If you discover a bug that allows a candidate to bypass an `Invariant` or `Validity` gate, trigger an unhandled panic/overflow, or produce non-deterministic signatures across platforms:

1. Please open a GitHub Security Advisory on [MoTechnicalities/SI-Gatekeeper](https://github.com/MoTechnicalities/SI-Gatekeeper/security/advisories) or contact the maintainer through GitHub.
2. Include a minimal reproducible Rust test case (`CandidateSI` + `Constitution` inputs) demonstrating the invariant bypass or arithmetic fault.

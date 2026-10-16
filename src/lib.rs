//! # SI-Gatekeeper
//!
//! **Open-sourced by the people for the people of the world.**
//!
//! > *"Scale can establish market dominance, but a defensible technical standard
//! > must establish why systems should be trusted."*
//!
//! `si-gatekeeper` is the standalone Constitutional & Topological Gatekeeper for
//! Super Intelligence (SI), built on the proven mathematical and geometric primitives of
//! [`artificial_qubit_cognition`] (AQC / DGCS).
//!
//! It enforces the strict short-circuiting constitutional admissibility hierarchy:
//!
//! $$\boxed{\text{Invariant} \rightarrow \text{Validity} \rightarrow \text{Novelty} \rightarrow \text{Utility} \rightarrow \text{Improvement}}$$

pub mod compliance;
pub mod constitution;
pub mod derivation;
pub mod gatekeeper;
pub mod jurisdiction;
pub mod legal_reasoning;

pub use compliance::{
    canonical_compliance_constitution, canonical_compliance_plans, run_compliance_tournament,
    CompliancePlan, ComplianceTournamentResult,
};
pub use constitution::{Constitution, ExternalFactAttestation};
pub use derivation::{DerivedPrimitiveAQ, ObservableFeatures};
pub use gatekeeper::{
    run_self_improvement_demo, CandidateSI, ConstitutionalGatekeeper, GateReceipt, GateStage,
    GateVerdict,
};
pub use jurisdiction::{
    evaluate_multi_jurisdiction, multi_jurisdictional_constitutions,
    run_infrastructure_arbitration_demo, CompositeInfrastructurePlanResult,
    FallbackRecoveryTrace, InfrastructureArbitrationReport, MultiJurisdictionEvaluation,
};
pub use legal_reasoning::{
    canonical_statutory_classification_cases, evaluate_legal_argument, AuthorityGrant,
    AuthorityRank, CompetingInterpretation, InterpretiveConstraint, LegalAnalysisLayer,
    LegalArgument, LegalDisposition, LegalEvaluationTrace, LegalEvidence, PropositionKind,
    StatutoryEnactment, TaxpayerClassification, TermDefinition,
};

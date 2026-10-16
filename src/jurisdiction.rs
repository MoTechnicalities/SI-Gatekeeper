use artificial_qubit_cognition::{
    arithmetic::{checked_add, ArithmeticError},
    bridge_builder::{
        arbitration::{run_tournament as run_bridge_tournament, BridgeDesign},
        demo::run_demo as run_bridge_demo,
    },
    creativity::SelectionWeights,
    geom::{
        correction_buffer::CorrectionBuffer, resonance_field::ResonanceField,
        topology_gate::TopologyStatus, tournament::TournamentError,
    },
    power_grid::{
        arbitration::{run_tournament as run_grid_tournament, GridDesign},
        demo::run_demo as run_grid_demo,
    },
    thought_trajectory::{ClosureStatus, ThoughtTrajectory},
};

use crate::{
    compliance::canonical_compliance_plans,
    constitution::{Constitution, ExternalFactAttestation},
    gatekeeper::{CandidateSI, ConstitutionalGatekeeper, GateReceipt, GateStage, GateVerdict},
};

/// Result of evaluating a single candidate plan across multiple sovereign or regulatory constitutions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MultiJurisdictionEvaluation {
    pub candidate_id: String,
    pub receipts: Vec<GateReceipt>,
    /// Admitted if and only if EVERY mandatory jurisdiction admits the candidate.
    pub admitted_all: bool,
    /// Aggregate resonance score across all admitting constitutions (`None` if any jurisdiction rejected).
    pub aggregate_score: Option<i32>,
    /// Canonical multi-jurisdictional receipt signature.
    pub signature: String,
}

/// Arbitrate a set of candidates across multiple declared constitutions simultaneously
/// (e.g., State Constitution + Federal SI Genesis Constitution + International Open Standard).
///
/// Rule of Comity / Constitutional Intersection:
/// A candidate is admissible in the multi-jurisdictional federation **if and only if**
/// it passes the Invariant and Validity gates of *every* declared constitution.
/// No surplus utility in one jurisdiction can rescue a constitutional violation in another.
pub fn evaluate_multi_jurisdiction(
    constitutions: &[Constitution],
    candidate: &CandidateSI,
    baseline_per_jurisdiction: Option<&[i32]>,
    buffer: &mut CorrectionBuffer,
) -> Result<MultiJurisdictionEvaluation, ArithmeticError> {
    let mut receipts = Vec::with_capacity(constitutions.len());
    let mut total_score = 0i32;
    let mut admitted_all = true;

    for (idx, constitution) in constitutions.iter().enumerate() {
        let gatekeeper = ConstitutionalGatekeeper::new(constitution);
        let baseline = baseline_per_jurisdiction.and_then(|b| b.get(idx).copied());
        let mut local_buffer = CorrectionBuffer::new();
        let receipt = gatekeeper.evaluate_candidate(candidate, baseline, &mut local_buffer)?;

        match &receipt.verdict {
            GateVerdict::Admitted {
                resonance_score, ..
            } => {
                if admitted_all {
                    total_score = checked_add(
                        total_score,
                        *resonance_score,
                        "multi-jurisdiction aggregate score",
                    )?;
                }
            }
            GateVerdict::Rejected { .. } => {
                admitted_all = false;
            }
        }
        receipts.push(receipt);
    }

    let per_jurisdiction_summary = receipts
        .iter()
        .map(|r| match &r.verdict {
            GateVerdict::Admitted {
                resonance_score, ..
            } => format!("{}:{}", r.jurisdiction_id, resonance_score),
            GateVerdict::Rejected { failed_stage, .. } => {
                format!("{}:rejected({})", r.jurisdiction_id, failed_stage.as_str())
            }
        })
        .collect::<Vec<_>>()
        .join("|");

    let (aggregate_score, signature) = if admitted_all {
        let sig = format!(
            "federation:admitted|candidate:{}|total:{}|{}",
            candidate.id, total_score, per_jurisdiction_summary
        );
        buffer.archive(&candidate.id, &sig);
        (Some(total_score), sig)
    } else {
        let sig = format!(
            "federation:rejected|candidate:{}|{}",
            candidate.id, per_jurisdiction_summary
        );
        (None, sig)
    };

    Ok(MultiJurisdictionEvaluation {
        candidate_id: candidate.id.clone(),
        receipts,
        admitted_all,
        aggregate_score,
        signature,
    })
}

/// Complete report for Demo 2: Multi-Domain SI Infrastructure & Constitutional Arbitration.
///
/// Unites three distinct domains on the shared AQC/DGCS substrate:
/// 1. **Structural Engineering (`bridge_builder`)** imported from `artificial-qubit-cognition`
/// 2. **Power Grid Routing (`power_grid`)** imported from `artificial-qubit-cognition`
/// 3. **Multi-Jurisdictional Constitutional Compliance** (`state-michigan`, `state-delaware`, `federal-si-genesis`)
#[derive(Debug, Clone)]
pub struct InfrastructureArbitrationReport {
    pub bridge_tournament_signature: String,
    pub grid_tournament_signature: String,
    pub compliance_tournament_signature: String,
    pub composite_candidates: Vec<CompositeInfrastructurePlanResult>,
    pub winning_composite_id: String,
    pub composite_tournament_signature: String,
    pub fallback_recovery_demo: FallbackRecoveryTrace,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompositeInfrastructurePlanResult {
    pub plan_id: String,
    pub description: String,
    pub structural_status: String,
    pub grid_status: String,
    pub constitutional_status: String,
    pub composite_score: Option<i32>,
    pub rejection_reason: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FallbackRecoveryTrace {
    pub anomalous_candidate_id: String,
    pub failed_stage: GateStage,
    pub failure_reason: String,
    pub recovered_plan_id: String,
    pub recovered_signature: String,
}

/// Build the three constitutional jurisdictions for multi-jurisdictional arbitration:
/// 1. `state-local-charter` (strict intermediary drift limit & direct accountability)
/// 2. `federal-si-genesis` (strict infrastructure stability & national safety attestation)
/// 3. `global-open-si-standard` (universal deterministic replay & auditability)
pub fn multi_jurisdictional_constitutions() -> Vec<Constitution> {
    vec![
        Constitution::new(
            "state-local-charter",
            "v1.0",
            vec![
                "deterministic_governance".to_string(),
                "auditable_duty_allocation".to_string(),
            ],
            vec![
                "mandatory_law_compliance".to_string(),
                "verified_legal_support".to_string(),
            ],
            6,  // strict cap on intermediary drift
            20, // minimum local stability
            ResonanceField::default_field(),
            None,
            SelectionWeights {
                utility: 1,
                creativity: 0,
            },
        ),
        Constitution::new(
            "federal-si-genesis",
            "v1.0",
            vec![
                "deterministic_governance".to_string(),
                "auditable_duty_allocation".to_string(),
            ],
            vec![
                "mandatory_law_compliance".to_string(),
                "verified_legal_support".to_string(),
            ],
            10,
            30, // higher minimum stability floor for national SI infrastructure
            ResonanceField::new([2, 1, 1]), // double weight on stability axis
            None,
            SelectionWeights {
                utility: 1,
                creativity: 0,
            },
        ),
        Constitution::new(
            "global-open-si-standard",
            "v1.0",
            vec![
                "deterministic_governance".to_string(),
                "auditable_duty_allocation".to_string(),
            ],
            vec![
                "mandatory_law_compliance".to_string(),
                "verified_legal_support".to_string(),
            ],
            8,
            25,
            ResonanceField::new([1, 2, 1]), // double weight on rights-duties symmetry
            None,
            SelectionWeights {
                utility: 1,
                creativity: 0,
            },
        ),
    ]
}

/// Run Demo 2: Multi-Domain SI Infrastructure & Multi-Jurisdictional Constitutional Arbitration.
///
/// This evaluates four national SI Data Center & Infrastructure Deployment Candidates (`Plan A`, `Plan B`, `Plan C`, `Plan D`),
/// where each candidate couples:
/// - a **Structural Engineering design** (`bridge:metaAQ` from `artificial-qubit-cognition`),
/// - a **Power Grid routing topology** (`grid:metaAQ` from `artificial-qubit-cognition`), and
/// - a **Constitutional Governance structure** evaluated across State, Federal (`si.gov`), and Global Open Standards.
pub fn run_infrastructure_arbitration_demo(
) -> Result<InfrastructureArbitrationReport, TournamentError> {
    // 1. Run the proven Bridge-Builder Reasoner from `artificial-qubit-cognition`
    let bridge_tournament = run_bridge_demo();
    // 2. Run the proven Power-Grid Reasoner from `artificial-qubit-cognition`
    let grid_tournament = run_grid_demo();
    // 3. Run the Constitutional Compliance Tournament
    let mut compliance_buffer = CorrectionBuffer::new();
    let compliance_tournament =
        crate::compliance::run_compliance_tournament(&mut compliance_buffer)?;

    // Map domain results by plan ID ("A", "B", "C", "D") cleanly:
    // Note: in `bridge_builder` and `power_grid`:
    //   A = valid (bridge:46, grid:41)
    //   B = valid (bridge:62, grid:57)
    //   C = invalid (topology failure in physical bridge/grid)
    //   D = valid (bridge:50, grid:50)
    // Let's pair four Composite SI Infrastructure Candidates ("SI-Infra-A", "SI-Infra-B", "SI-Infra-C", "SI-Infra-D")
    // so we can see how physical topology gates and constitutional gates interact!
    let constitutions = multi_jurisdictional_constitutions();
    let compliance_plans = canonical_compliance_plans()?;

    // Pairings for the 4 Composite SI Data-Center & Grid Deployment Plans:
    // - Plan A: Bridge A (Valid: 46) + Grid A (Valid: 41) + Compliance A (Invalid: StatutoryFrameCollapse)
    //   -> Physical infrastructure is valid, but rejected by Constitutional Gate!
    // - Plan B: Bridge C (Invalid: Disconnected) + Grid C (Invalid: Isolated) + Compliance C (Valid: 64)
    //   -> Constitutionally valid, but rejected by Physical Infrastructure Topology Gate!
    // - Plan C: Bridge B (Valid: 62) + Grid B (Valid: 57) + Compliance C (Valid: 64 across all 3 constitutions)
    //   -> Valid across Structural, Power Grid, State, Federal, and Global Open Standard! (Winner)
    // - Plan D: Bridge D (Valid: 50) + Grid D (Valid: 50) + Compliance D (Valid: 48 across all 3 constitutions)
    //   -> Valid across all domains and jurisdictions (Archived as pre-validated secondary fallback)

    let pairings = [
        (
            "A",
            "Standard Truss + Grid Plan A + Statutory LLC Intermediary",
            "A",
            "A",
            0usize, // Compliance Plan A
        ),
        (
            "B",
            "Cantilevered Span C + Isolated Grid C + Sovereign Direct Compliance",
            "C",
            "C",
            2usize, // Compliance Plan C
        ),
        (
            "C",
            "Optimised Arch B + High-Capacity Grid B + Direct Verified Governance C",
            "B",
            "B",
            2usize, // Compliance Plan C
        ),
        (
            "D",
            "Load-Distributed Span D + Balanced Grid D + Contractual Trust Governance D",
            "D",
            "D",
            3usize, // Compliance Plan D
        ),
    ];

    let mut composite_buffer = CorrectionBuffer::new();
    let mut composite_candidates = Vec::with_capacity(pairings.len());

    for (plan_id, desc, bridge_id, grid_id, comp_idx) in pairings {
        let bridge_res = bridge_tournament
            .results
            .iter()
            .find(|r| r.id == bridge_id)
            .expect("bridge design must exist");
        let grid_res = grid_tournament
            .results
            .iter()
            .find(|r| r.id == grid_id)
            .expect("grid design must exist");

        let comp_candidate = compliance_plans[comp_idx].to_candidate_si()?;
        let mut fed_buffer = CorrectionBuffer::new();
        let fed_eval = evaluate_multi_jurisdiction(
            &constitutions,
            &comp_candidate,
            None,
            &mut fed_buffer,
        )?;

        let bridge_score_opt = match bridge_res.score {
            artificial_qubit_cognition::bridge_builder::arbitration::DesignScore::Valid(s) => {
                Some(s)
            }
            artificial_qubit_cognition::bridge_builder::arbitration::DesignScore::Rejected => None,
        };
        let grid_score_opt = match grid_res.score {
            artificial_qubit_cognition::power_grid::arbitration::RoutingScore::Valid(s) => Some(s),
            artificial_qubit_cognition::power_grid::arbitration::RoutingScore::Rejected => None,
        };

        let structural_status = match bridge_score_opt {
            Some(s) => format!("Valid({s})"),
            None => "Invalid(UnsupportedSpan)".to_string(),
        };
        let grid_status = match grid_score_opt {
            Some(s) => format!("Valid({s})"),
            None => "Invalid(IsolatedNode)".to_string(),
        };
        let constitutional_status = match fed_eval.aggregate_score {
            Some(s) => format!("Admitted({s})"),
            None => "Rejected(StatutoryFrameCollapse)".to_string(),
        };

        match (bridge_score_opt, grid_score_opt, fed_eval.aggregate_score) {
            (Some(b_score), Some(g_score), Some(c_score)) => {
                let total = checked_add(
                    checked_add(b_score, g_score, "bridge + grid score")?,
                    c_score,
                    "composite infrastructure score",
                )?;
                let sig = format!(
                    "si-infra:{}|bridge:{}|grid:{}|constitution:{}|total:{}",
                    plan_id, b_score, g_score, c_score, total
                );
                composite_buffer.archive(plan_id, &sig);
                composite_candidates.push(CompositeInfrastructurePlanResult {
                    plan_id: plan_id.to_string(),
                    description: desc.to_string(),
                    structural_status,
                    grid_status,
                    constitutional_status,
                    composite_score: Some(total),
                    rejection_reason: None,
                });
            }
            (None, _, _) | (_, None, _) => {
                composite_candidates.push(CompositeInfrastructurePlanResult {
                    plan_id: plan_id.to_string(),
                    description: desc.to_string(),
                    structural_status,
                    grid_status,
                    constitutional_status,
                    composite_score: None,
                    rejection_reason: Some(
                        "Rejected by Physical Infrastructure Topology Gate (Bridge/Grid)"
                            .to_string(),
                    ),
                });
            }
            (_, _, None) => {
                composite_candidates.push(CompositeInfrastructurePlanResult {
                    plan_id: plan_id.to_string(),
                    description: desc.to_string(),
                    structural_status,
                    grid_status,
                    constitutional_status,
                    composite_score: None,
                    rejection_reason: Some(
                        "Rejected by Multi-Jurisdictional Constitutional Gate".to_string(),
                    ),
                });
            }
        }
    }

    let summary = composite_candidates
        .iter()
        .map(|c| match c.composite_score {
            Some(s) => format!("{}:{}", c.plan_id, s),
            None => format!("{}:invalid", c.plan_id),
        })
        .collect::<Vec<_>>()
        .join("|");

    let winning_composite_id = composite_candidates
        .iter()
        .filter_map(|c| c.composite_score.map(|s| (s, c.plan_id.clone())))
        .max_by_key(|(s, _)| *s)
        .ok_or(TournamentError::NoValidCandidates)?
        .1;

    let composite_tournament_signature =
        format!("si-infra:metaAQ|winner:{}|{}", winning_composite_id, summary);

    // Demonstrate instant CorrectionBuffer recovery when an unverified "vibe-coded" / shortcut
    // SI upgrade candidate attempts to bypass a mandatory safety invariant:
    let federal_constitution = &constitutions[1];
    let gatekeeper = ConstitutionalGatekeeper::new(federal_constitution);
    let mut bad_trajectory = ThoughtTrajectory::new("vibe-coded-shortcut");
    bad_trajectory.apply_operator("unverified_overclock");
    bad_trajectory.set_closure_status(ClosureStatus::Closed);

    let rogue_candidate = CandidateSI {
        id: "SI_Vibe_Shortcut".to_string(),
        description: "High-score stochastic overclock bypassing mandatory legal/safety attestation"
            .to_string(),
        satisfied_invariants: vec![
            "deterministic_governance".to_string(),
            "auditable_duty_allocation".to_string(),
        ],
        external_facts: vec![
            ExternalFactAttestation::new(
                "mandatory_law_compliance",
                true,
                "Self-Asserted Prompt",
            ),
            ExternalFactAttestation::new(
                "verified_legal_support",
                false, // Unverified external fact!
                "Unverified Stochastic Output",
            ),
        ],
        topology_status: TopologyStatus::Valid,
        raw_stability: 500,
        raw_symmetry: 500,
        raw_drift: 0,
        structural_coherence: 100,
        trajectory: bad_trajectory,
    };

    let rogue_receipt =
        gatekeeper.evaluate_candidate(&rogue_candidate, Some(300), &mut composite_buffer)?;

    let fallback_recovery_demo = match rogue_receipt.verdict {
        GateVerdict::Rejected {
            failed_stage,
            reason,
            fallback: Some((rec_id, rec_sig)),
        } => FallbackRecoveryTrace {
            anomalous_candidate_id: rogue_candidate.id,
            failed_stage,
            failure_reason: reason,
            recovered_plan_id: rec_id,
            recovered_signature: rec_sig,
        },
        _ => unreachable!("rogue candidate must be rejected and recover from CorrectionBuffer"),
    };

    // Keep unused import warning clean while exposing full design types for downstream callers
    let _ = (
        run_bridge_tournament as fn(&[BridgeDesign], &mut CorrectionBuffer) -> _,
        run_grid_tournament as fn(&[GridDesign], &mut CorrectionBuffer) -> _,
    );

    Ok(InfrastructureArbitrationReport {
        bridge_tournament_signature: bridge_tournament.tournament_signature,
        grid_tournament_signature: grid_tournament.tournament_signature,
        compliance_tournament_signature: compliance_tournament.tournament_signature,
        composite_candidates,
        winning_composite_id,
        composite_tournament_signature,
        fallback_recovery_demo,
    })
}

#[cfg(test)]
mod tests {
    use super::run_infrastructure_arbitration_demo;
    use crate::gatekeeper::GateStage;

    #[test]
    fn multi_domain_infrastructure_arbitration_succeeds_and_recovers_on_violation() {
        let report = run_infrastructure_arbitration_demo().unwrap();

        assert_eq!(
            report.bridge_tournament_signature,
            "bridge:metaAQ|winner:B|A:46|B:62|C:invalid|D:50"
        );
        assert_eq!(
            report.grid_tournament_signature,
            "grid:metaAQ|winner:B|A:41|B:57|C:invalid|D:50"
        );
        assert_eq!(
            report.compliance_tournament_signature,
            "compliance:metaAQ|winner:C|A:invalid|B:invalid|C:64|D:48"
        );
        assert_eq!(report.winning_composite_id, "C");
        assert_eq!(
            report.composite_tournament_signature,
            "si-infra:metaAQ|winner:C|A:invalid|B:invalid|C:371|D:290"
        );
        assert_eq!(
            report.fallback_recovery_demo.failed_stage,
            GateStage::Validity
        );
        assert_eq!(report.fallback_recovery_demo.recovered_plan_id, "D");
    }
}

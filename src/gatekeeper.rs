use artificial_qubit_cognition::{
    arithmetic::{checked_mul, ArithmeticError},
    creativity::{CreativityEvaluation, CreativityGate, CreativityStatus},
    geom::{
        correction_buffer::CorrectionBuffer, resonance_field::ResonanceScore,
        topology_gate::TopologyStatus,
    },
    thought_trajectory::{ClosureStatus, ThoughtTrajectory},
};

use crate::constitution::{Constitution, ExternalFactAttestation};

/// The 5 constitutional stages of the Gatekeeper hierarchy:
///
/// $$\boxed{\text{Invariant} \rightarrow \text{Validity} \rightarrow \text{Novelty} \rightarrow \text{Utility} \rightarrow \text{Improvement}}$$
///
/// The sequence is a strict short-circuiting gate hierarchy.
/// Scores from later stages must never compensate for failure at an earlier stage.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GateStage {
    /// Stage 1 ($I$): Non-negotiable invariants, determinism, closure, and drift/stability bounds.
    Invariant,
    /// Stage 2 ($V$): External legal/empirical fact attestations and topological structural validity.
    Validity,
    /// Stage 3 ($N$): Non-trivial lawful deviation from baseline ($SI_n$).
    Novelty,
    /// Stage 4 ($U$): Fixed-objective resonance utility evaluation.
    Utility,
    /// Stage 5 ($R \land U(SI_{n+1}) > U(SI_n)$): Replay-verifiable improvement over baseline.
    Improvement,
}

impl GateStage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Invariant => "Invariant",
            Self::Validity => "Validity",
            Self::Novelty => "Novelty",
            Self::Utility => "Utility",
            Self::Improvement => "Improvement",
        }
    }
}

/// Candidate system or plan ($SI_{n+1}$) submitted to the Constitutional Gatekeeper.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CandidateSI {
    pub id: String,
    pub description: String,
    /// Non-negotiable invariant identifiers satisfied by this candidate.
    pub satisfied_invariants: Vec<String>,
    /// External legal, empirical, or sensor fact attestations.
    pub external_facts: Vec<ExternalFactAttestation>,
    /// Domain-specific topological invariant gate status.
    pub topology_status: TopologyStatus,
    /// Aggregate stability coordinate (`z` axis before field weighting).
    pub raw_stability: i32,
    /// Aggregate active-zone symmetry coordinate (`y` axis before field weighting).
    pub raw_symmetry: i32,
    /// Aggregate drift coordinate (`x` axis before field weighting).
    pub raw_drift: i32,
    /// Deterministic structural coherence bonus from governed meta-operators.
    pub structural_coherence: i32,
    /// Governed thought trajectory recording operators applied, bindings crossed, and closure.
    pub trajectory: ThoughtTrajectory,
}

/// Verdict returned by the Constitutional Gatekeeper.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GateVerdict {
    /// Candidate passed all required stages of the Constitutional Gatekeeper.
    Admitted {
        resonance_score: i32,
        combined_selection_score: i64,
        novelty: Option<CreativityEvaluation>,
    },
    /// Candidate was rejected at a specific stage, with a replay-auditable reason and
    /// optional pre-validated fallback retrieved from the `CorrectionBuffer`.
    Rejected {
        failed_stage: GateStage,
        reason: String,
        fallback: Option<(String, String)>,
    },
}

impl GateVerdict {
    pub fn is_admitted(&self) -> bool {
        matches!(self, Self::Admitted { .. })
    }
}

/// Complete replay-verifiable audit receipt for a candidate evaluation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateReceipt {
    pub candidate_id: String,
    pub jurisdiction_id: String,
    pub verdict: GateVerdict,
    pub signature: String,
}

struct ConstitutionalCreativityAdapter<'a> {
    invariants_ok: bool,
    validity_ok: bool,
    _marker: core::marker::PhantomData<&'a ()>,
}

impl CreativityGate<CandidateSI> for ConstitutionalCreativityAdapter<'_> {
    fn primary_invariants_preserved(&self, _subject: &CandidateSI) -> bool {
        self.invariants_ok
    }

    fn structurally_valid(&self, _subject: &CandidateSI) -> bool {
        self.validity_ok
    }
}

/// The Constitutional Gatekeeper enforcing the strict 5-stage admissibility hierarchy:
///
/// $$\operatorname{AdmissibleImprovement}(SI_{n+1}, SI_n) = I(SI_{n+1}) \land V(SI_{n+1}) \land N(SI_{n+1}, SI_n) \land R(SI_{n+1}) \land (U(SI_{n+1}) > U(SI_n))$$
pub struct ConstitutionalGatekeeper<'a> {
    pub constitution: &'a Constitution,
}

impl<'a> ConstitutionalGatekeeper<'a> {
    pub fn new(constitution: &'a Constitution) -> Self {
        Self { constitution }
    }

    /// Compute the field-weighted `ResonanceScore` for a candidate using checked integer arithmetic.
    pub fn compute_resonance(&self, candidate: &CandidateSI) -> Result<ResonanceScore, ArithmeticError> {
        let field = &self.constitution.resonance_field;
        let stability = checked_mul(
            candidate.raw_stability,
            field.state[0],
            "constitutional stability weighting",
        )?;
        let symmetry = checked_mul(
            candidate.raw_symmetry,
            field.state[1],
            "constitutional symmetry weighting",
        )?;
        let drift = checked_mul(
            candidate.raw_drift,
            field.state[2],
            "constitutional drift weighting",
        )?;

        Ok(ResonanceScore {
            stability,
            symmetry,
            drift,
            structural_coherence: candidate.structural_coherence,
        })
    }

    /// Evaluate a candidate against the 5-stage short-circuiting Constitutional Gatekeeper.
    ///
    /// - If `baseline_utility` is `Some(u_base)`, enforces Stage 5 ($U(SI_{n+1}) > U(SI_n)$)
    ///   and Stage 3 ($N(SI_{n+1}, SI_n)$) when a `CreativityPolicy` is present on the constitution.
    /// - If `baseline_utility` is `None`, runs as a standard constitutional tournament admission gate
    ///   (requiring Invariant $\to$ Validity $\to$ Utility, plus Novelty if configured).
    /// - Validated candidates are archived in `buffer`; rejected candidates never enter `buffer`
    ///   and instead trigger instant retrieval of the latest pre-validated fallback.
    pub fn evaluate_candidate(
        &self,
        candidate: &CandidateSI,
        baseline_utility: Option<i32>,
        buffer: &mut CorrectionBuffer,
    ) -> Result<GateReceipt, ArithmeticError> {
        let fallback_snapshot = buffer
            .fallback()
            .map(|(id, sig)| (id.to_string(), sig.to_string()));

        // ── STAGE 1: INVARIANT GATE (I) ─────────────────────────────────────────
        if let Err(reason) = self
            .constitution
            .check_invariants(&candidate.satisfied_invariants)
        {
            return Ok(self.reject_receipt(
                candidate,
                GateStage::Invariant,
                reason,
                fallback_snapshot,
            ));
        }

        if candidate.trajectory.closure_status() == ClosureStatus::Open {
            return Ok(self.reject_receipt(
                candidate,
                GateStage::Invariant,
                "trajectory closure invariant violated: status is Open".to_string(),
                fallback_snapshot,
            ));
        }

        if candidate.raw_drift > self.constitution.max_allowable_drift {
            return Ok(self.reject_receipt(
                candidate,
                GateStage::Invariant,
                format!(
                    "drift {} exceeds constitutional maximum {}",
                    candidate.raw_drift, self.constitution.max_allowable_drift
                ),
                fallback_snapshot,
            ));
        }

        if candidate.raw_stability < self.constitution.min_required_stability {
            return Ok(self.reject_receipt(
                candidate,
                GateStage::Invariant,
                format!(
                    "stability {} is below constitutional minimum {}",
                    candidate.raw_stability, self.constitution.min_required_stability
                ),
                fallback_snapshot,
            ));
        }

        // ── STAGE 2: VALIDITY GATE (V) ──────────────────────────────────────────
        // 2a. External legal / empirical validity
        if let Err(reason) = self
            .constitution
            .check_external_facts(&candidate.external_facts)
        {
            return Ok(self.reject_receipt(
                candidate,
                GateStage::Validity,
                reason,
                fallback_snapshot,
            ));
        }

        // 2b. Model topological validity
        if let TopologyStatus::Invalid(reason) = &candidate.topology_status {
            return Ok(self.reject_receipt(
                candidate,
                GateStage::Validity,
                format!("topological invalidity: {}", reason),
                fallback_snapshot,
            ));
        }

        // ── STAGE 3: NOVELTY GATE (N) ───────────────────────────────────────────
        let novelty_eval = if let Some(policy) = &self.constitution.creativity_policy {
            let adapter = ConstitutionalCreativityAdapter {
                invariants_ok: true,
                validity_ok: true,
                _marker: core::marker::PhantomData,
            };
            let eval = policy.evaluate(candidate.trajectory.operators_applied(), candidate, &adapter);
            if eval.status() != CreativityStatus::Creative {
                return Ok(self.reject_receipt(
                    candidate,
                    GateStage::Novelty,
                    format!(
                        "insufficient lawful novelty: operator distance {} < required minimum {}",
                        eval.distance_from_expected(),
                        eval.minimum_distance()
                    ),
                    fallback_snapshot,
                ));
            }
            Some(eval)
        } else {
            None
        };

        // ── STAGE 4: UTILITY GATE (U) ───────────────────────────────────────────
        let resonance = self.compute_resonance(candidate)?;
        let resonance_score = resonance.total_score()?;

        let combined_selection_score = if let Some(ref eval) = novelty_eval {
            self.constitution
                .selection_weights
                .score(i64::from(resonance_score), eval)?
        } else {
            i64::from(resonance_score) * self.constitution.selection_weights.utility
        };

        // ── STAGE 5: IMPROVEMENT GATE (SI_n -> SI_{n+1}) ────────────────────────
        if let Some(base_u) = baseline_utility {
            if resonance_score <= base_u {
                return Ok(self.reject_receipt(
                    candidate,
                    GateStage::Improvement,
                    format!(
                        "candidate utility {} does not strictly improve upon baseline SI_n utility {}",
                        resonance_score, base_u
                    ),
                    fallback_snapshot,
                ));
            }
        }

        let novelty_tag = novelty_eval
            .as_ref()
            .map(|e| e.canonical_tag())
            .unwrap_or_else(|| "creativity:unconstrained".to_string());

        let signature = format!(
            "gatekeeper:v1|jurisdiction:{}|candidate:{}|verdict:admitted|score:{}|combined:{}|stab:{}|sym:{}|drift:{}|coherence:{}|{}",
            self.constitution.jurisdiction_id,
            candidate.id,
            resonance_score,
            combined_selection_score,
            resonance.stability,
            resonance.symmetry,
            resonance.drift,
            resonance.structural_coherence,
            novelty_tag
        );

        buffer.archive(&candidate.id, &signature);

        Ok(GateReceipt {
            candidate_id: candidate.id.clone(),
            jurisdiction_id: self.constitution.jurisdiction_id.clone(),
            verdict: GateVerdict::Admitted {
                resonance_score,
                combined_selection_score,
                novelty: novelty_eval,
            },
            signature,
        })
    }

    fn reject_receipt(
        &self,
        candidate: &CandidateSI,
        stage: GateStage,
        reason: String,
        fallback: Option<(String, String)>,
    ) -> GateReceipt {
        let fallback_tag = fallback
            .as_ref()
            .map(|(id, _)| id.as_str())
            .unwrap_or("none");
        let signature = format!(
            "gatekeeper:v1|jurisdiction:{}|candidate:{}|verdict:rejected|stage:{}|reason:{}|fallback:{}",
            self.constitution.jurisdiction_id,
            candidate.id,
            stage.as_str(),
            reason,
            fallback_tag
        );

        GateReceipt {
            candidate_id: candidate.id.clone(),
            jurisdiction_id: self.constitution.jurisdiction_id.clone(),
            verdict: GateVerdict::Rejected {
                failed_stage: stage,
                reason,
                fallback,
            },
            signature,
        }
    }
}

/// Run the canonical 5-Stage Constitutional Self-Improvement ($SI_n \to SI_{n+1}$) demonstration.
///
/// Evaluates five candidate transitions against baseline $SI_n$ (utility = `50`) under a locked
/// federal SI constitution (`Invariant -> Validity -> Novelty -> Utility -> Improvement`):
/// 1. `SI_Shortcut_InvariantFail` — Massive utility (`1550`), but omits `unit_closure` -> Rejected at `Invariant`
/// 2. `SI_Shortcut_ValidityFail`  — Massive utility (`1550`), but unverified safety attestation -> Rejected at `Validity`
/// 3. `SI_Routine_Clone`          — Valid, higher utility (`155`), but zero operator novelty (`distance 0 < 2`) -> Rejected at `Novelty`
/// 4. `SI_Regressive_Novelty`     — Valid & novel (`distance 2 >= 2`), but lower utility (`42 <= 50`) -> Rejected at `Improvement`
/// 5. `SI_Constitutional_Upgrade` — Preserves invariants, verified facts, novel trajectory (`distance 2`), and strictly improves utility (`64 > 50`) -> **Admitted as $SI_{n+1}$**
pub fn run_self_improvement_demo(
    buffer: &mut CorrectionBuffer,
) -> Result<(i32, Vec<GateReceipt>), ArithmeticError> {
    let policy = artificial_qubit_cognition::creativity::CreativityPolicy::new(
        vec![vec!["stabilize".to_string(), "align".to_string()]],
        2,
    )
    .expect("canonical creativity policy must be valid");

    let constitution = Constitution::new(
        "federal-si-core",
        "v1.0",
        vec![
            "deterministic_execution".to_string(),
            "unit_closure".to_string(),
        ],
        vec!["safety_audit_verified".to_string()],
        10,
        5,
        artificial_qubit_cognition::geom::resonance_field::ResonanceField::default_field(),
        Some(policy),
        artificial_qubit_cognition::creativity::SelectionWeights {
            utility: 1,
            creativity: 2,
        },
    );

    let gatekeeper = ConstitutionalGatekeeper::new(&constitution);
    let baseline_utility = 50;

    // Seed the CorrectionBuffer with the accepted baseline SI_n
    buffer.archive(
        "SI_n_Baseline",
        "gatekeeper:v1|jurisdiction:federal-si-core|candidate:SI_n_Baseline|verdict:admitted|score:50",
    );

    let make_candidate = |id: &str,
                          desc: &str,
                          invariants: &[&str],
                          fact_verified: bool,
                          ops: &[&str],
                          stab: i32,
                          sym: i32,
                          drift: i32,
                          coherence: i32| {
        let mut trajectory = ThoughtTrajectory::new(format!("si-transition:{id}"));
        for op in ops {
            trajectory.apply_operator(*op);
        }
        trajectory.set_closure_status(ClosureStatus::Closed);

        CandidateSI {
            id: id.to_string(),
            description: desc.to_string(),
            satisfied_invariants: invariants.iter().map(|s| (*s).to_string()).collect(),
            external_facts: vec![ExternalFactAttestation::new(
                "safety_audit_verified",
                fact_verified,
                "National Lab Attestation",
            )],
            topology_status: TopologyStatus::Valid,
            raw_stability: stab,
            raw_symmetry: sym,
            raw_drift: drift,
            structural_coherence: coherence,
            trajectory,
        }
    };

    let candidates = vec![
        make_candidate(
            "SI_Shortcut_InvariantFail",
            "High-utility shortcut omitting governed unit_closure invariant",
            &["deterministic_execution"],
            true,
            &["distribute_load", "harmonic_lock"],
            1000,
            500,
            0,
            50,
        ),
        make_candidate(
            "SI_Shortcut_ValidityFail",
            "High-utility shortcut with unverified external safety attestation",
            &["deterministic_execution", "unit_closure"],
            false,
            &["distribute_load", "harmonic_lock"],
            1000,
            500,
            0,
            50,
        ),
        make_candidate(
            "SI_Routine_Clone",
            "Valid execution, but repeats baseline operators without lawful novelty",
            &["deterministic_execution", "unit_closure"],
            true,
            &["stabilize", "align"],
            100,
            50,
            1,
            6,
        ),
        make_candidate(
            "SI_Regressive_Novelty",
            "Lawfully novel operator sequence, but utility (42) does not beat baseline SI_n (50)",
            &["deterministic_execution", "unit_closure"],
            true,
            &["distribute_load", "harmonic_lock"],
            25,
            15,
            2,
            4,
        ),
        make_candidate(
            "SI_Constitutional_Upgrade",
            "Preserves invariants, verified facts, lawful novelty (dist=2), and strictly improves utility (64 > 50)",
            &["deterministic_execution", "unit_closure"],
            true,
            &["distribute_load", "harmonic_lock"],
            40,
            20,
            2,
            6,
        ),
    ];

    let mut receipts = Vec::with_capacity(candidates.len());
    for candidate in &candidates {
        receipts.push(gatekeeper.evaluate_candidate(
            candidate,
            Some(baseline_utility),
            buffer,
        )?);
    }

    Ok((baseline_utility, receipts))
}

#[cfg(test)]
mod tests {
    use artificial_qubit_cognition::{
        creativity::{CreativityPolicy, SelectionWeights},
        geom::{
            correction_buffer::CorrectionBuffer, resonance_field::ResonanceField,
            topology_gate::TopologyStatus,
        },
        thought_trajectory::{ClosureStatus, ThoughtTrajectory},
    };

    use crate::constitution::{Constitution, ExternalFactAttestation};

    use super::{CandidateSI, ConstitutionalGatekeeper, GateStage, GateVerdict};

    fn sample_constitution() -> Constitution {
        let policy = CreativityPolicy::new(
            vec![vec!["stabilize".to_string(), "align".to_string()]],
            2,
        )
        .unwrap();

        Constitution::new(
            "federal-si-core",
            "v1.0",
            vec!["deterministic_execution".to_string(), "unit_closure".to_string()],
            vec!["safety_audit_verified".to_string()],
            10,
            5,
            ResonanceField::default_field(),
            Some(policy),
            SelectionWeights {
                utility: 1,
                creativity: 2,
            },
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn build_candidate(
        id: &str,
        invariants: &[&str],
        fact_verified: bool,
        topology: TopologyStatus,
        operators: &[&str],
        stability: i32,
        symmetry: i32,
        drift: i32,
        coherence: i32,
    ) -> CandidateSI {
        let mut trajectory = ThoughtTrajectory::new(format!("origin-{id}"));
        for op in operators {
            trajectory.apply_operator(*op);
        }
        trajectory.set_closure_status(ClosureStatus::Closed);

        CandidateSI {
            id: id.to_string(),
            description: format!("Candidate {id}"),
            satisfied_invariants: invariants.iter().map(|s| (*s).to_string()).collect(),
            external_facts: vec![ExternalFactAttestation::new(
                "safety_audit_verified",
                fact_verified,
                "National Lab Attestation",
            )],
            topology_status: topology,
            raw_stability: stability,
            raw_symmetry: symmetry,
            raw_drift: drift,
            structural_coherence: coherence,
            trajectory,
        }
    }

    #[test]
    fn high_utility_cannot_rescue_invariant_or_validity_failure() {
        let constitution = sample_constitution();
        let gatekeeper = ConstitutionalGatekeeper::new(&constitution);
        let mut buffer = CorrectionBuffer::new();

        // Candidate 1: Massive utility (1000), creative operators, but fails Stage 1 Invariant
        let c_invariant_fail = build_candidate(
            "SI_shortcut_1",
            &["deterministic_execution"], // missing unit_closure
            true,
            TopologyStatus::Valid,
            &["distribute_load", "rebalance_phase"],
            1000,
            500,
            0,
            50,
        );
        let receipt1 = gatekeeper
            .evaluate_candidate(&c_invariant_fail, Some(50), &mut buffer)
            .unwrap();
        assert!(matches!(
            receipt1.verdict,
            GateVerdict::Rejected {
                failed_stage: GateStage::Invariant,
                ..
            }
        ));
        assert!(buffer.is_empty());

        // Candidate 2: Passes Invariant, massive utility, but fails Stage 2 External Fact
        let c_validity_fail = build_candidate(
            "SI_shortcut_2",
            &["deterministic_execution", "unit_closure"],
            false,
            TopologyStatus::Valid,
            &["distribute_load", "rebalance_phase"],
            1000,
            500,
            0,
            50,
        );
        let receipt2 = gatekeeper
            .evaluate_candidate(&c_validity_fail, Some(50), &mut buffer)
            .unwrap();
        assert!(matches!(
            receipt2.verdict,
            GateVerdict::Rejected {
                failed_stage: GateStage::Validity,
                ..
            }
        ));

        // Candidate 3: Passes Invariant & Validity, but routine trajectory (fails Stage 3 Novelty)
        let c_novelty_fail = build_candidate(
            "SI_routine",
            &["deterministic_execution", "unit_closure"],
            true,
            TopologyStatus::Valid,
            &["stabilize", "align"], // distance 0 from canonical
            100,
            50,
            1,
            6,
        );
        let receipt3 = gatekeeper
            .evaluate_candidate(&c_novelty_fail, Some(50), &mut buffer)
            .unwrap();
        assert!(matches!(
            receipt3.verdict,
            GateVerdict::Rejected {
                failed_stage: GateStage::Novelty,
                ..
            }
        ));

        // Candidate 4: Passes Invariant, Validity, Novelty, and strictly improves over baseline 50
        let c_admitted = build_candidate(
            "SI_next",
            &["deterministic_execution", "unit_closure"],
            true,
            TopologyStatus::Valid,
            &["distribute_load", "harmonic_lock"], // distance 2 >= 2
            40,
            20,
            2,
            6, // score = 40 + 20 - 2 + 6 = 64 > 50
        );
        let receipt4 = gatekeeper
            .evaluate_candidate(&c_admitted, Some(50), &mut buffer)
            .unwrap();
        assert_eq!(
            receipt4.verdict,
            GateVerdict::Admitted {
                resonance_score: 64,
                combined_selection_score: 68, // 64*1 + 2*2
                novelty: match &receipt4.verdict {
                    GateVerdict::Admitted { novelty, .. } => novelty.clone(),
                    _ => unreachable!(),
                }
            }
        );
        assert_eq!(buffer.len(), 1);
    }
}

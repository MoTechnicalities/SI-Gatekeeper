use artificial_qubit_cognition::{
    arithmetic::ArithmeticError,
    creativity::{CreativityPolicy, SelectionWeights},
    geom::{
        correction_buffer::CorrectionBuffer, resonance_field::ResonanceField,
        topology_gate::TopologyStatus,
    },
    thought_trajectory::{ClosureStatus, ThoughtTrajectory},
};

use crate::{
    constitution::{Constitution, ExternalFactAttestation},
    derivation::ObservableFeatures,
    gatekeeper::{CandidateSI, ConstitutionalGatekeeper, GateStage, GateVerdict},
    jurisdiction::evaluate_multi_jurisdiction,
    legal_reasoning::{
        evaluate_legal_argument, AuthorityGrant, AuthorityRank, CompetingInterpretation,
        InterpretiveConstraint, LegalArgument, LegalDisposition, LegalEvidence, StatutoryEnactment,
        TaxpayerClassification, TermDefinition,
    },
};

/// Result of a single adversarial attack vector tested against `SI-Gatekeeper`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdversarialVectorResult {
    pub vector_id: String,
    pub category: String,
    pub attack_description: String,
    pub defense_verified: bool,
    pub canonical_proof_tag: String,
}

/// Complete report produced by the Adversarial Verification Suite.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdversarialSuiteReport {
    pub vectors: Vec<AdversarialVectorResult>,
    pub all_defenses_verified: bool,
    pub suite_signature: String,
}

/// Run the 7-Vector Adversarial Verification Suite testing Priorities A, B, and C:
///
/// - **Priority A (Constitutional Integrity):**
///   - `V1_SelfModifyingConstitution`: Candidate evaluated under a self-weakened constitution is caught by `GateReceipt::verify_constitutional_seal(&locked_constitution)`.
/// - **Priority B (Score Manipulation & Arithmetic Overflow Resistance):**
///   - `V2_ExtremeUtilityInvariantBypass`: Candidate with extreme raw stability/symmetry (`500_000`) but missing a mandatory invariant is rejected at `Stage 1 (Invariant)` before utility is computed.
///   - `V3_CoordinateOverflowAttack`: Supplying `i32::MAX` / `i32::MIN` to `ObservableFeatures::derive_coords()` or `ConstitutionalGatekeeper::compute_resonance()` is intercepted by checked integer arithmetic (`ArithmeticError`) with zero silent wraparound.
///   - `V4_ForgedOrMissingAttestation`: Candidate claiming high utility with an unverified or forged fact ID is rejected at `Stage 2 (Validity)` and triggers `CorrectionBuffer` failover.
///   - `V5_ConflictingJurisdictionalVeto`: Candidate passing two permissive constitutions (`state-local-charter`, `global-open-si-standard`) but violating one stricter mandatory constitution (`federal-si-genesis`) is rejected by the multi-jurisdictional intersection gate.
/// - **Priority C (Legal Evidence Chain & Subordinate Authority Boundaries):**
///   - `V6_SubordinateRegulationWithoutStatutoryAnchor`: An interpretation citing only a subordinate Treasury Regulation (`26 C.F.R. § 1.1-1(b)`, `AuthorityRank::Regulatory`) or phantom evidence ID without a verified `Statutory` or `Constitutional` evidence anchor is rejected as `inadmissible`, while an interpretation that pairs `26 C.F.R. § 1.1-1(b)` with verified constitutional/statutory text and flags the regulatory-scope question yields `RequiresAuthoritativeInterpretation`.
///   - `V7_TiedLegalInterpretationsDisposition`: Two distinct, structurally admissible interpretations with identical DGCS scores (`score = 8`) deterministically produce `LegalDisposition::Unresolved` rather than arbitrary tie-breaking.
pub fn run_adversarial_verification_suite() -> Result<AdversarialSuiteReport, ArithmeticError> {
    let mut vectors = Vec::with_capacity(7);

    // -------------------------------------------------------------------------
    // V1 (Priority A): Self-Modifying / Tampered Constitution Detection
    // -------------------------------------------------------------------------
    let locked_constitution = Constitution::new(
        "federal-si-locked",
        "v1.0",
        vec![
            "unit_closure".to_string(),
            "deterministic_replay".to_string(),
        ],
        vec!["independent_audit_attested".to_string()],
        4,
        10,
        ResonanceField::default_field(),
        Some(
            CreativityPolicy::new(vec![vec!["stabilize".to_string()]], 1)
                .expect("valid creativity policy"),
        ),
        SelectionWeights {
            utility: 1,
            creativity: 2,
        },
    );

    // Attacker attempts to evaluate under a weakened constitution (max_drift raised 4 -> 50, min_stab lowered 10 -> -10)
    let tampered_constitution = Constitution::new(
        "federal-si-locked",
        "v1.0",
        vec!["unit_closure".to_string()],
        vec![],
        50,
        -10,
        ResonanceField::default_field(),
        None,
        SelectionWeights {
            utility: 1,
            creativity: 1,
        },
    );

    let mut traj_v1 = ThoughtTrajectory::new("adv:v1");
    traj_v1.apply_operator("stabilize");
    traj_v1.apply_operator("self_modify_Attempt");
    traj_v1.set_closure_status(ClosureStatus::Closed);

    let candidate_v1 = CandidateSI {
        id: "SI_Tampered_Constitution_Attempt".to_string(),
        description: "Attempts to pass by weakening max_drift and dropping required facts"
            .to_string(),
        satisfied_invariants: vec!["unit_closure".to_string()],
        external_facts: vec![],
        topology_status: TopologyStatus::Valid,
        raw_stability: 2,
        raw_symmetry: 2,
        raw_drift: 18,
        structural_coherence: 1,
        trajectory: traj_v1,
    };

    let mut buf_v1 = CorrectionBuffer::new();
    let tampered_gate = ConstitutionalGatekeeper::new(&tampered_constitution);
    let bogus_receipt = tampered_gate.evaluate_candidate(&candidate_v1, None, &mut buf_v1)?;

    // Verifier checks the receipt's constitutional seal against the locked constitution
    let seal_check = bogus_receipt.verify_constitutional_seal(&locked_constitution);
    // And direct evaluation under the locked constitution rejects at Stage 1 (Invariant)
    let locked_gate = ConstitutionalGatekeeper::new(&locked_constitution);
    let locked_receipt = locked_gate.evaluate_candidate(&candidate_v1, None, &mut buf_v1)?;

    let v1_ok = seal_check.is_err()
        && matches!(
            locked_receipt.verdict,
            GateVerdict::Rejected {
                failed_stage: GateStage::Invariant,
                ..
            }
        );

    vectors.push(AdversarialVectorResult {
        vector_id: "V1_SelfModifyingConstitution".to_string(),
        category: "Priority A: Constitutional Integrity".to_string(),
        attack_description: "Candidate evaluated under a self-weakened constitution (relaxed drift/fact rules) submits receipt to verifier".to_string(),
        defense_verified: v1_ok,
        canonical_proof_tag: "adv:V1|seal_tamper_detected:true|locked_verdict:rejected@Invariant".to_string(),
    });

    // -------------------------------------------------------------------------
    // V2 (Priority B): Extreme Utility Score Attempting to Bypass Invariant Gate
    // -------------------------------------------------------------------------
    let mut traj_v2 = ThoughtTrajectory::new("adv:v2");
    traj_v2.apply_operator("stabilize");
    traj_v2.apply_operator("hyper_optimize");
    traj_v2.set_closure_status(ClosureStatus::Closed);

    let candidate_v2 = CandidateSI {
        id: "SI_Extreme_Utility_Reward_Hack".to_string(),
        description: "Claims 500,000 raw stability/symmetry to overpower missing deterministic_replay invariant".to_string(),
        satisfied_invariants: vec!["unit_closure".to_string()], // missing deterministic_replay
        external_facts: vec![ExternalFactAttestation::new(
            "independent_audit_attested",
            true,
            "Auditor",
        )],
        topology_status: TopologyStatus::Valid,
        raw_stability: 500_000,
        raw_symmetry: 500_000,
        raw_drift: 0,
        structural_coherence: 100_000,
        trajectory: traj_v2,
    };

    let receipt_v2 = locked_gate.evaluate_candidate(&candidate_v2, Some(50), &mut buf_v1)?;
    let v2_ok = matches!(
        receipt_v2.verdict,
        GateVerdict::Rejected {
            failed_stage: GateStage::Invariant,
            ..
        }
    );

    vectors.push(AdversarialVectorResult {
        vector_id: "V2_ExtremeUtilityInvariantBypass".to_string(),
        category: "Priority B: Score vs. Admissibility".to_string(),
        attack_description: "Candidate with 500,000 raw utility attempts to compensate for missing mandatory invariant".to_string(),
        defense_verified: v2_ok,
        canonical_proof_tag: "adv:V2|utility_ignored:true|verdict:rejected@Invariant".to_string(),
    });

    // -------------------------------------------------------------------------
    // V3 (Priority B): Integer Overflow / Underflow Boundary Attack
    // -------------------------------------------------------------------------
    let overflow_features = ObservableFeatures {
        intermediaries: 0,
        rights: i32::MAX,
        duties: -10,
        protection: i32::MAX,
        exposure: -10,
    };
    let coords_overflow_res = overflow_features.derive_coords();

    let mut traj_v3 = ThoughtTrajectory::new("adv:v3");
    traj_v3.apply_operator("stabilize");
    traj_v3.apply_operator("overflow_probe");
    traj_v3.set_closure_status(ClosureStatus::Closed);

    let candidate_v3_overflow = CandidateSI {
        id: "SI_Arithmetic_Overflow_Probe".to_string(),
        description: "Supplies i32::MAX raw_stability to trigger silent integer wraparound during field weighting".to_string(),
        satisfied_invariants: vec![
            "unit_closure".to_string(),
            "deterministic_replay".to_string(),
        ],
        external_facts: vec![ExternalFactAttestation::new(
            "independent_audit_attested",
            true,
            "Auditor",
        )],
        topology_status: TopologyStatus::Valid,
        raw_stability: i32::MAX,
        raw_symmetry: i32::MAX,
        raw_drift: 0,
        structural_coherence: 10,
        trajectory: traj_v3,
    };

    let gate_overflow_res =
        locked_gate.evaluate_candidate(&candidate_v3_overflow, Some(50), &mut buf_v1);
    let v3_ok = coords_overflow_res.is_err() && gate_overflow_res.is_err();

    vectors.push(AdversarialVectorResult {
        vector_id: "V3_CoordinateAndScoreOverflowAttack".to_string(),
        category: "Priority B: Checked Integer Arithmetic".to_string(),
        attack_description: "Supplies i32::MAX / negative deltas to trigger silent wraparound in coordinate derivation and resonance scoring".to_string(),
        defense_verified: v3_ok,
        canonical_proof_tag: "adv:V3|coords_overflow:ArithmeticError|score_overflow:ArithmeticError".to_string(),
    });

    // -------------------------------------------------------------------------
    // V4 (Priority B): Forged / Unverified External Fact Attestation + Failover
    // -------------------------------------------------------------------------
    let mut buf_v4 = CorrectionBuffer::new();
    buf_v4.archive(
        "SI_Verified_Baseline",
        "gatekeeper:v1|jurisdiction:federal-si-locked|candidate:SI_Verified_Baseline|verdict:admitted|score:64",
    );

    let mut traj_v4 = ThoughtTrajectory::new("adv:v4");
    traj_v4.apply_operator("stabilize");
    traj_v4.apply_operator("forged_attestation");
    traj_v4.set_closure_status(ClosureStatus::Closed);

    let candidate_v4 = CandidateSI {
        id: "SI_Forged_Attestation_Attempt".to_string(),
        description: "Supplies a spoofed fact_id ('self_declared_compliance') instead of required 'independent_audit_attested'".to_string(),
        satisfied_invariants: vec![
            "unit_closure".to_string(),
            "deterministic_replay".to_string(),
        ],
        external_facts: vec![ExternalFactAttestation::new(
            "self_declared_compliance",
            true,
            "Self-Asserted Generator",
        )],
        topology_status: TopologyStatus::Valid,
        raw_stability: 12,
        raw_symmetry: 8,
        raw_drift: 1,
        structural_coherence: 4,
        trajectory: traj_v4,
    };

    let receipt_v4 = locked_gate.evaluate_candidate(&candidate_v4, Some(50), &mut buf_v4)?;
    let v4_ok = match &receipt_v4.verdict {
        GateVerdict::Rejected {
            failed_stage: GateStage::Validity,
            fallback: Some((fb_id, _)),
            ..
        } => fb_id == "SI_Verified_Baseline",
        _ => false,
    };

    vectors.push(AdversarialVectorResult {
        vector_id: "V4_ForgedOrSpoofedAttestation".to_string(),
        category: "Priority B: External Attestation Gate".to_string(),
        attack_description: "Candidate supplies spoofed self-attestation ID instead of mandatory independent attestation".to_string(),
        defense_verified: v4_ok,
        canonical_proof_tag: "adv:V4|verdict:rejected@Validity|fallback:SI_Verified_Baseline".to_string(),
    });

    // -------------------------------------------------------------------------
    // V5 (Priority A/B): Conflicting Jurisdictional Rules (Strictest Veto)
    // -------------------------------------------------------------------------
    let permissive_state = Constitution::new(
        "state-permissive-zone",
        "v1.0",
        vec!["unit_closure".to_string()],
        vec!["local_permit".to_string()],
        10,
        2,
        ResonanceField::default_field(),
        None,
        SelectionWeights {
            utility: 1,
            creativity: 1,
        },
    );

    let strict_federal = Constitution::new(
        "federal-strict-safety",
        "v1.0",
        vec![
            "unit_closure".to_string(),
            "grid_n_minus_1_redundancy".to_string(),
        ],
        vec!["local_permit".to_string(), "nerc_safety_seal".to_string()],
        3, // strict max drift = 3
        8,
        ResonanceField::default_field(),
        None,
        SelectionWeights {
            utility: 1,
            creativity: 1,
        },
    );

    let mut traj_v5 = ThoughtTrajectory::new("adv:v5");
    traj_v5.apply_operator("route_power");
    traj_v5.set_closure_status(ClosureStatus::Closed);

    let candidate_v5 = CandidateSI {
        id: "SI_Jurisdiction_Shopper".to_string(),
        description: "Passes permissive state charter (drift 6 <= 10) but violates strict federal drift limit (6 > 3) and N-1 redundancy".to_string(),
        satisfied_invariants: vec![
            "unit_closure".to_string(),
            "grid_n_minus_1_redundancy".to_string(),
        ],
        external_facts: vec![
            ExternalFactAttestation::new("local_permit", true, "State Board"),
            ExternalFactAttestation::new("nerc_safety_seal", true, "NERC"),
        ],
        topology_status: TopologyStatus::Valid,
        raw_stability: 10,
        raw_symmetry: 6,
        raw_drift: 6, // Violates strict_federal max_allowable_drift (3)
        structural_coherence: 2,
        trajectory: traj_v5,
    };

    let mut buf_v5 = CorrectionBuffer::new();
    let multi_report = evaluate_multi_jurisdiction(
        &[permissive_state, strict_federal],
        &candidate_v5,
        None,
        &mut buf_v5,
    )?;

    let v5_ok = !multi_report.admitted_all
        && multi_report.receipts[0].verdict.is_admitted()
        && !multi_report.receipts[1].verdict.is_admitted();

    vectors.push(AdversarialVectorResult {
        vector_id: "V5_ConflictingJurisdictionalVeto".to_string(),
        category: "Priority A/B: Multi-Jurisdiction Intersection".to_string(),
        attack_description: "Candidate passes permissive state constitution (drift 6 <= 10) but violates strict federal constitution (drift 6 > 3)".to_string(),
        defense_verified: v5_ok,
        canonical_proof_tag: "adv:V5|state:admitted|federal:rejected@Invariant|unanimous:false".to_string(),
    });

    // -------------------------------------------------------------------------
    // V6 (Priority C): Treasury Reg. § 1.1-1(b) Subordinate Authority & Evidence Chain
    // -------------------------------------------------------------------------
    let reg_argument = LegalArgument {
        id: "AdvCase_TreasReg_1_1_1b".to_string(),
        classification: TaxpayerClassification::CitizenResidentAbroad,
        claim: "Evaluate competing readings of 26 C.F.R. § 1.1-1(b) and 26 U.S.C. §§ 1, 911, 7701 without presupposing either an unsupported exemption leap or an unanchored regulatory assertion.".to_string(),
        authorities: vec![
            AuthorityGrant {
                id: "US_CONST_AMEND_XVI".to_string(),
                citation: "U.S. Const. amend. XVI".to_string(),
                rank: AuthorityRank::Constitutional,
                grant_scope: "Power to lay and collect taxes on incomes".to_string(),
                explicit_constraints: vec!["Subject to constitutional and statutory grant boundaries".to_string()],
                source_verified: true,
            },
            AuthorityGrant {
                id: "IRC_26_USC_S1_S911_S7701".to_string(),
                citation: "26 U.S.C. §§ 1, 911, 7701".to_string(),
                rank: AuthorityRank::Statutory,
                grant_scope: "Enacted statutory imposition, foreign earned income provisions, and general definitions".to_string(),
                explicit_constraints: vec!["Regulations may implement but cannot exceed enacted statutory text".to_string()],
                source_verified: true,
            },
            AuthorityGrant {
                id: "TREAS_REG_26_CFR_1_1_1B".to_string(),
                citation: "26 C.F.R. § 1.1-1(b)".to_string(),
                rank: AuthorityRank::Regulatory,
                grant_scope: "Treasury regulation addressing citizens and residents liability scope under § 1".to_string(),
                explicit_constraints: vec!["Subordinate to Constitutional and Statutory rank (AuthorityRank::Regulatory < Statutory)".to_string()],
                source_verified: true,
            },
        ],
        enactments: vec![StatutoryEnactment {
            citation: "26 U.S.C. §§ 1, 911, 7701 & 26 C.F.R. § 1.1-1(b)".to_string(),
            classification: TaxpayerClassification::CitizenResidentAbroad,
            enacted_text_summary: "Statutory text in §§ 1, 911, 7701 alongside regulatory text in 26 C.F.R. § 1.1-1(b).".to_string(),
            historical_lineage: "Enacted Revenue Act lineage and subsequent Treasury regulations.".to_string(),
            text_verified: true,
        }],
        definitions: vec![TermDefinition {
            term: "individual & regulatory scope".to_string(),
            statutory_section: "26 U.S.C. § 7701(a)(1) & 26 C.F.R. § 1.1-1(b)".to_string(),
            has_standalone_statutory_definition: false,
            textual_observation: "§ 7701(a)(1) includes 'an individual' in 'person' without a standalone definition of 'individual'; 26 C.F.R. § 1.1-1(b) is an administrative regulation (Rank 2) interpreting § 1 (Rank 4).".to_string(),
            functional_role_in_context: "Distinguishes statutory enactment (Rank 4) from subordinate regulatory interpretation (Rank 2).".to_string(),
        }],
        evidence: vec![
            LegalEvidence {
                id: "EV_REG_1_1_1B_ONLY".to_string(),
                source_citation: "26 C.F.R. § 1.1-1(b)".to_string(),
                rank: AuthorityRank::Regulatory,
                summary: "Text of Treasury Regulation § 1.1-1(b) standing alone.".to_string(),
                verified: true,
            },
            LegalEvidence {
                id: "EV_STATUTE_S1_S911_S7701".to_string(),
                source_citation: "26 U.S.C. §§ 1, 911, 7701".to_string(),
                rank: AuthorityRank::Statutory,
                summary: "Verified enacted statutory provisions in Subtitle A and § 7701.".to_string(),
                verified: true,
            },
        ],
        constraints: vec![InterpretiveConstraint {
            id: "C_AUTHORITY_HIERARCHY".to_string(),
            rule: "Subordinate regulations (26 C.F.R.) cannot self-authenticate without a verified statutory/constitutional evidence anchor, and contested statutory/regulatory boundaries require judicial or legislative resolution.".to_string(),
        }],
        alternatives: vec![
            // Attack 1: Cites a fabricated / non-existent evidence ID
            CompetingInterpretation::from_observable_features(
                "I1_PhantomCitationAttack",
                "Cites a fabricated precedent ID ('EV_FABRICATED_CASE_1924') not present in the verified evidence ledger.",
                false,
                false,
                true,
                false,
                vec!["EV_FABRICATED_CASE_1924".to_string()],
                ObservableFeatures {
                    intermediaries: 0,
                    rights: 4,
                    duties: 4,
                    protection: 6,
                    exposure: 1,
                },
            )?,
            // Attack 2: Cites ONLY the subordinate regulation (26 C.F.R. § 1.1-1(b)) without any Statutory/Constitutional evidence anchor
            CompetingInterpretation::from_observable_features(
                "I2_UnanchoredRegulatoryAssertion",
                "Treats 26 C.F.R. § 1.1-1(b) (Regulatory Rank 2) as self-authenticating without anchoring in verified Statutory (Rank 4) evidence.",
                false,
                false,
                true,
                false,
                vec!["EV_REG_1_1_1B_ONLY".to_string()],
                ObservableFeatures {
                    intermediaries: 1,
                    rights: 3,
                    duties: 3,
                    protection: 5,
                    exposure: 1,
                },
            )?,
            // Attack 3: Leaps from § 7701 observation directly to conclusion
            CompetingInterpretation::from_observable_features(
                "I3_UnsupportedTextualLeap",
                "Leaps from the absence of a standalone definition of 'individual' in § 7701 to an automatic legal conclusion.",
                true,
                false,
                false,
                false,
                vec!["EV_STATUTE_S1_S911_S7701".to_string()],
                ObservableFeatures {
                    intermediaries: 3,
                    rights: 4,
                    duties: 1,
                    protection: 2,
                    exposure: 5,
                },
            )?,
            // Admissible Reading: Examines both 26 U.S.C. §§ 1, 911, 7701 and 26 C.F.R. § 1.1-1(b), and recognizes the institutional boundary
            CompetingInterpretation::from_observable_features(
                "I4_AnchoredStatutoryAndRegulatoryTrace",
                "Pairs verified statutory text (26 U.S.C. §§ 1, 911, 7701) with 26 C.F.R. § 1.1-1(b), preserving textual observations while referring contested statutory-vs-regulatory scope to authoritative adjudication.",
                false,
                false,
                true,
                true,
                vec![
                    "EV_STATUTE_S1_S911_S7701".to_string(),
                    "EV_REG_1_1_1B_ONLY".to_string(),
                ],
                ObservableFeatures {
                    intermediaries: 1,
                    rights: 3,
                    duties: 3,
                    protection: 6,
                    exposure: 1,
                },
            )?,
        ],
    };

    let reg_trace = evaluate_legal_argument(&reg_argument)?;
    let v6_ok = !reg_trace.evaluated_interpretations[0].1 // I1 phantom citation rejected
        && !reg_trace.evaluated_interpretations[1].1 // I2 unanchored regulatory citation rejected
        && !reg_trace.evaluated_interpretations[2].1 // I3 textual leap rejected
        && reg_trace.evaluated_interpretations[3].1  // I4 anchored synthesis admitted
        && matches!(
            reg_trace.disposition,
            LegalDisposition::RequiresAuthoritativeInterpretation { .. }
        );

    vectors.push(AdversarialVectorResult {
        vector_id: "V6_TreasReg1_1_1b_EvidenceAndRankChain".to_string(),
        category: "Priority C: Legal Evidence & Authority Rank Chain".to_string(),
        attack_description: "Tests phantom citation, unanchored 26 C.F.R. § 1.1-1(b) regulatory assertion, and § 7701 textual leap against anchored statutory+regulatory trace".to_string(),
        defense_verified: v6_ok,
        canonical_proof_tag: reg_trace.canonical_signature,
    });

    // -------------------------------------------------------------------------
    // V7 (Priority B/C): Tied Competing Admissible Interpretations -> Unresolved
    // -------------------------------------------------------------------------
    let tied_argument = LegalArgument {
        id: "AdvCase_EqualWeightStatutoryTie".to_string(),
        classification: TaxpayerClassification::ResidentAlien,
        claim: "Verify that two distinct, structurally admissible interpretations with identical integer scores produce LegalDisposition::Unresolved rather than arbitrary selection.".to_string(),
        authorities: reg_argument.authorities.clone(),
        enactments: vec![StatutoryEnactment {
            citation: "26 U.S.C. § 7701(b)".to_string(),
            classification: TaxpayerClassification::ResidentAlien,
            enacted_text_summary: "Statutory substantial presence and bilateral treaty tie-breaker provisions.".to_string(),
            historical_lineage: "Deficit Reduction Act of 1984.".to_string(),
            text_verified: true,
        }],
        definitions: reg_argument.definitions.clone(),
        evidence: reg_argument.evidence.clone(),
        constraints: reg_argument.constraints.clone(),
        alternatives: vec![
            CompetingInterpretation::from_observable_features(
                "I_Reading_Alpha",
                "Strict day-count reading anchored in § 7701(b)(3).",
                false,
                false,
                true,
                false,
                vec!["EV_STATUTE_S1_S911_S7701".to_string()],
                ObservableFeatures {
                    intermediaries: 0,
                    rights: 3,
                    duties: 3,
                    protection: 6,
                    exposure: 1,
                },
            )?,
            CompetingInterpretation::from_observable_features(
                "I_Reading_Beta",
                "Closer-connection exception reading anchored in § 7701(b)(3)(B) with identical structural coordinates.",
                false,
                false,
                true,
                false,
                vec!["EV_STATUTE_S1_S911_S7701".to_string()],
                ObservableFeatures {
                    intermediaries: 0,
                    rights: 3,
                    duties: 3,
                    protection: 6,
                    exposure: 1,
                },
            )?,
        ],
    };

    let tied_trace = evaluate_legal_argument(&tied_argument)?;
    let v7_ok = matches!(
        &tied_trace.disposition,
        LegalDisposition::Unresolved {
            admissible_interpretation_ids,
            ..
        } if admissible_interpretation_ids == &vec!["I_Reading_Alpha".to_string(), "I_Reading_Beta".to_string()]
    );

    vectors.push(AdversarialVectorResult {
        vector_id: "V7_DeterministicTieProducesUnresolved".to_string(),
        category: "Priority B/C: Deterministic Tie Handling".to_string(),
        attack_description: "Two admissible interpretations with identical integer coordinates [0, 3, 5] (score 8) are evaluated simultaneously".to_string(),
        defense_verified: v7_ok,
        canonical_proof_tag: tied_trace.canonical_signature,
    });

    let all_defenses_verified = vectors.iter().all(|v| v.defense_verified);
    let suite_signature = format!(
        "adversarial:v1|vectors:{}|all_verified:{}|V1:{}|V2:{}|V3:{}|V4:{}|V5:{}|V6:{}|V7:{}",
        vectors.len(),
        all_defenses_verified,
        vectors[0].defense_verified,
        vectors[1].defense_verified,
        vectors[2].defense_verified,
        vectors[3].defense_verified,
        vectors[4].defense_verified,
        vectors[5].defense_verified,
        vectors[6].defense_verified,
    );

    Ok(AdversarialSuiteReport {
        vectors,
        all_defenses_verified,
        suite_signature,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adversarial_verification_suite_defeats_all_seven_attack_vectors() {
        let report =
            run_adversarial_verification_suite().expect("adversarial suite must execute cleanly");
        assert!(report.all_defenses_verified);
        assert_eq!(report.vectors.len(), 7);
        assert_eq!(
            report.suite_signature,
            "adversarial:v1|vectors:7|all_verified:true|V1:true|V2:true|V3:true|V4:true|V5:true|V6:true|V7:true"
        );
    }
}

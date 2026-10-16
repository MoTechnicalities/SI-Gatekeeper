use artificial_qubit_cognition::{
    arithmetic::{checked_add, ArithmeticError},
    geom::resonance_field::ResonanceField,
    thought_trajectory::{ClosureStatus, ThoughtTrajectory},
};

use crate::derivation::{DerivedPrimitiveAQ, ObservableFeatures};

/// 6-Layer Constitutional & Statutory Analysis Hierarchy in DGCS.
///
/// **Important Disclaimer:** This module is a software engineering demonstration of
/// structured, multi-layer rule and citation tracing. Nothing in this module constitutes
/// legal or tax advice, nor does any disposition (`Supported`, `Contradicted`, `Unresolved`,
/// `RequiresAuthoritativeInterpretation`) represent a binding legal ruling. The statutory
/// cases constructed below use illustrative, hand-authored candidate interpretations to
/// demonstrate how the gatekeeper rejects both unsupported leaps and unexamined dismissals.
///
/// Layers:
/// 1. **Authority**      — What power does the Constitution grant the actor/legislature, and under what constraints?
/// 2. **Enactment**      — What did the legislature actually enact for each classification?
/// 3. **Definitions** — How do statutory terms (*individual*, *citizen*, *resident*, *resident alien*) function,
///    explicitly separating *Textual Observations* from *Legal Conclusions*?
/// 4. **Interpretation** — What readings of the language are supported by grammar, context, history, and precedent?
/// 5. **Admissibility**  — Which conclusions remain consistent with controlling legal authorities and verified evidence?
/// 6. **DecisionTrace**  — What sources and reasoning support each conclusion, and what remains unresolved?
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LegalAnalysisLayer {
    Authority,
    Enactment,
    Definitions,
    Interpretation,
    Admissibility,
    DecisionTrace,
}

impl LegalAnalysisLayer {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Authority => "Authority",
            Self::Enactment => "Enactment",
            Self::Definitions => "Definitions",
            Self::Interpretation => "Interpretation",
            Self::Admissibility => "Admissibility",
            Self::DecisionTrace => "DecisionTrace",
        }
    }
}

/// Hierarchy level of a legal authority source.
///
/// Controlling priority: `Constitutional > Statutory > JudicialPrecedent > Regulatory > AdministrativeGuidance`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AuthorityRank {
    AdministrativeGuidance = 1,
    Regulatory = 2,
    JudicialPrecedent = 3,
    Statutory = 4,
    Constitutional = 5,
}

impl AuthorityRank {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Constitutional => "Constitutional",
            Self::Statutory => "Statutory",
            Self::JudicialPrecedent => "JudicialPrecedent",
            Self::Regulatory => "Regulatory",
            Self::AdministrativeGuidance => "AdministrativeGuidance",
        }
    }
}

/// Layer 1: Constitutional or statutory grant of authority and its explicit constraints.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorityGrant {
    pub id: String,
    pub citation: String,
    pub rank: AuthorityRank,
    pub grant_scope: String,
    pub explicit_constraints: Vec<String>,
    pub source_verified: bool,
}

/// Classification of the subject or case being analyzed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaxpayerClassification {
    /// U.S. citizens residing within U.S. states.
    CitizenResidingInStates,
    /// U.S. citizens residing abroad (foreign residence / foreign-earned income context).
    CitizenResidentAbroad,
    /// Non-citizen resident aliens residing within U.S. jurisdiction.
    ResidentAlien,
}

impl TaxpayerClassification {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CitizenResidingInStates => "citizen_residing_in_us_states",
            Self::CitizenResidentAbroad => "citizen_resident_abroad",
            Self::ResidentAlien => "resident_alien",
        }
    }
}

/// Layer 2: Enacted statutory text applicable to a specific classification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatutoryEnactment {
    pub citation: String,
    pub classification: TaxpayerClassification,
    pub enacted_text_summary: String,
    pub historical_lineage: String,
    pub text_verified: bool,
}

/// Distinguishes between a source-verifiable **Textual Observation** (e.g., absence of a standalone
/// definition of "individual" in 26 U.S.C. § 7701) and a **Legal Conclusion** (e.g., whether a
/// citizen falls within or outside that term), preventing an SI from jumping directly from a
/// grammatical observation to an unsupported legal conclusion.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PropositionKind {
    /// A directly verifiable fact about statutory text, grammar, or structure.
    TextualObservation,
    /// A normative legal conclusion requiring full synthesis of authority, context, history, and precedent.
    LegalConclusion,
}

/// Layer 3: Term definition and epistemic status within the statutory scheme.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TermDefinition {
    pub term: String,
    pub statutory_section: String,
    pub has_standalone_statutory_definition: bool,
    pub textual_observation: String,
    pub functional_role_in_context: String,
}

///Verifiable evidence item (statutory text, historical enactment, or judicial decision).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LegalEvidence {
    pub id: String,
    pub source_citation: String,
    pub rank: AuthorityRank,
    pub summary: String,
    pub verified: bool,
}

/// Interpretive constraint that no admissible legal reading may violate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InterpretiveConstraint {
    pub id: String,
    pub rule: String,
}

/// Layer 4: A competing legal interpretation of the enacted language, grounded in observable
/// structural features so its DGCS coordinates in $\mathbb{Z}^3$ (`[drift, symmetry, stability]`)
/// are derived rather than asserted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompetingInterpretation {
    pub id: String,
    pub summary: String,
    /// Whether this reading leaps from a textual observation to a legal conclusion without
    /// accounting for statutory context, history, and controlling precedent.
    pub conflates_observation_with_conclusion: bool,
    /// Whether this reading dismisses textual/historical evidence by heuristic without inspecting the text.
    pub dismisses_text_without_examination: bool,
    /// Whether this reading respects the hierarchy of controlling authorities.
    pub consistent_with_controlling_authority: bool,
    /// Whether this reading requires resolving an open constitutional or statutory question reserved to a court/legislature.
    pub requires_institutional_adjudication: bool,
    /// Supporting evidence IDs cited by this interpretation.
    pub supporting_evidence_ids: Vec<String>,
    /// Derived DGCS primitive representing the structural geometry of this reading.
    pub derived_primitive: DerivedPrimitiveAQ,
}

impl CompetingInterpretation {
    #[allow(clippy::too_many_arguments)]
    pub fn from_observable_features(
        id: impl Into<String>,
        summary: impl Into<String>,
        conflates_observation_with_conclusion: bool,
        dismisses_text_without_examination: bool,
        consistent_with_controlling_authority: bool,
        requires_institutional_adjudication: bool,
        supporting_evidence_ids: Vec<String>,
        features: ObservableFeatures,
    ) -> Result<Self, ArithmeticError> {
        let id = id.into();
        let derived_primitive = DerivedPrimitiveAQ::from_features(format!("interp:{id}"), features)?;
        Ok(Self {
            id,
            summary: summary.into(),
            conflates_observation_with_conclusion,
            dismisses_text_without_examination,
            consistent_with_controlling_authority,
            requires_institutional_adjudication,
            supporting_evidence_ids,
            derived_primitive,
        })
    }
}

/// Four-way Disposition of a Legal Argument under the SI-Gatekeeper Constitutional Standard.
///
/// Crucially enforces the **Non-Usurpation Boundary**:
/// DGCS formally applies, structures, and audits legal rules, but never claims the sovereign
/// authority of a court or legislature to establish binding law where interpretation is unresolved.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LegalDisposition {
    /// Supported by controlling authority, enacted text, definitions, and verified evidence.
    Supported {
        winning_interpretation_id: String,
        resonance_score: i32,
    },
    /// Contradicted by controlling authority, statutory text, or interpretive constraints.
    Contradicted {
        rejected_reasons: Vec<String>,
    },
    /// Multiple competing interpretations remain admissible; evidence or text leaves the question open.
    Unresolved {
        admissible_interpretation_ids: Vec<String>,
        open_questions: Vec<String>,
    },
    /// Reaches the institutional boundary: requires authoritative adjudication by a competent court or legislature.
    RequiresAuthoritativeInterpretation {
        candidate_interpretation_id: String,
        institutional_question: String,
    },
}

impl LegalDisposition {
    pub fn kind_str(&self) -> &'static str {
        match self {
            Self::Supported { .. } => "Supported",
            Self::Contradicted { .. } => "Contradicted",
            Self::Unresolved { .. } => "Unresolved",
            Self::RequiresAuthoritativeInterpretation { .. } => {
                "RequiresAuthoritativeInterpretation"
            }
        }
    }
}

/// Structured Legal Argument object evaluated by `SI-Gatekeeper`.
///
/// Encodes all eight required elements:
/// - `claim`: The precise legal proposition being tested.
/// - `authorities`: Constitutional, statutory, and judicial sources supporting or constraining it.
/// - `enactments`: Enacted provisions for the specific classification.
/// - `definitions`: Controlling meanings of terms & separation of textual observation vs. legal conclusion.
/// - `evidence`: Relevant text, history, and decisions.
/// - `constraints`: Rules that interpretations cannot violate.
/// - `alternatives`: Competing interpretations and their supporting evidence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LegalArgument {
    pub id: String,
    pub classification: TaxpayerClassification,
    pub claim: String,
    pub authorities: Vec<AuthorityGrant>,
    pub enactments: Vec<StatutoryEnactment>,
    pub definitions: Vec<TermDefinition>,
    pub evidence: Vec<LegalEvidence>,
    pub constraints: Vec<InterpretiveConstraint>,
    pub alternatives: Vec<CompetingInterpretation>,
}

/// Complete, replay-verifiable evaluation record of a `LegalArgument` across all 6 DGCS layers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LegalEvaluationTrace {
    pub argument_id: String,
    pub classification: TaxpayerClassification,
    pub claim: String,
    pub textual_observations: Vec<String>,
    pub evaluated_interpretations: Vec<(String, bool, Option<i32>, String)>,
    pub disposition: LegalDisposition,
    pub trajectory: ThoughtTrajectory,
    pub canonical_signature: String,
}

/// Evaluate a `LegalArgument` through the 6-Layer DGCS Constitutional & Statutory Analysis Pipeline.
///
/// Enforces:
/// 1. **No Conflation Guard:** Rejects any interpretation that leaps directly from a grammatical/textual
///    observation (such as the absence of a standalone definition in § 7701) to a legal conclusion
///    without satisfying context, history, and precedent.
/// 2. **No Heuristic Dismissal Guard:** Rejects any interpretation that dismisses an argument without
///    examining the enacted text and verified evidence.
/// 3. **Hierarchy of Authority Guard:** Rejects interpretations that contradict higher-ranking controlling authority.
/// 4. **Non-Usurpation Boundary:** Emits `RequiresAuthoritativeInterpretation` when an otherwise admissible
///    interpretation turns on a question reserved to a court or legislature.
pub fn evaluate_legal_argument(
    argument: &LegalArgument,
) -> Result<LegalEvaluationTrace, ArithmeticError> {
    let mut trajectory = ThoughtTrajectory::new(format!(
        "legal-arg:{}:{}",
        argument.id,
        argument.classification.as_str()
    ));

    // Layer 1: Authority check
    trajectory.apply_operator("layer1:verify_constitutional_authority");
    let has_verified_authority = argument
        .authorities
        .iter()
        .any(|a| a.source_verified && a.rank >= AuthorityRank::Statutory);
    for auth in &argument.authorities {
        trajectory.cross_binding(format!("authority:{}:{}", auth.id, auth.rank.as_str()));
    }

    // Layer 2: Enactment check
    trajectory.apply_operator("layer2:verify_statutory_enactment");
    let has_verified_enactment = argument
        .enactments
        .iter()
        .any(|e| e.text_verified && e.classification == argument.classification);

    // Layer 3: Definitions & separation of Textual Observation from Legal Conclusion
    trajectory.apply_operator("layer3:separate_textual_observation_from_conclusion");
    let textual_observations: Vec<String> = argument
        .definitions
        .iter()
        .map(|d| {
            format!(
                "term '{}' ({}): standalone_def={} | observation: {}",
                d.term,
                d.statutory_section,
                d.has_standalone_statutory_definition,
                d.textual_observation
            )
        })
        .collect();

    // Layer 4 & Layer 5: Interpretation & Admissibility Gate
    trajectory.apply_operator("layer4:evaluate_competing_interpretations");
    trajectory.apply_operator("layer5:enforce_admissibility_constraints");

    let field = ResonanceField::default_field();
    let mut evaluated_interpretations = Vec::with_capacity(argument.alternatives.len());
    let mut admissible_scored: Vec<(&CompetingInterpretation, i32)> = Vec::new();
    let mut rejected_reasons: Vec<String> = Vec::new();

    for interp in &argument.alternatives {
        if !has_verified_authority || !has_verified_enactment {
            let reason =
                "missing verified controlling authority or statutory enactment".to_string();
            rejected_reasons.push(format!("{}: {}", interp.id, reason));
            evaluated_interpretations.push((interp.id.clone(), false, None, reason));
            continue;
        }

        if interp.conflates_observation_with_conclusion {
            let reason = "violates No-Conflation Constraint: leaps from textual observation directly to legal conclusion without contextual/precedential support".to_string();
            rejected_reasons.push(format!("{}: {}", interp.id, reason));
            evaluated_interpretations.push((interp.id.clone(), false, None, reason));
            continue;
        }

        if interp.dismisses_text_without_examination {
            let reason = "violates Textual-Examination Constraint: dismisses argument by heuristic without examining enacted language and evidence".to_string();
            rejected_reasons.push(format!("{}: {}", interp.id, reason));
            evaluated_interpretations.push((interp.id.clone(), false, None, reason));
            continue;
        }

        if !interp.consistent_with_controlling_authority {
            let reason =
                "contradicts controlling constitutional/statutory/judicial authority hierarchy"
                    .to_string();
            rejected_reasons.push(format!("{}: {}", interp.id, reason));
            evaluated_interpretations.push((interp.id.clone(), false, None, reason));
            continue;
        }

        // Verify all cited evidence IDs exist and are verified
        let evidence_ok = !interp.supporting_evidence_ids.is_empty()
            && interp.supporting_evidence_ids.iter().all(|ev_id| {
                argument
                    .evidence
                    .iter()
                    .any(|ev| &ev.id == ev_id && ev.verified)
            });

        if !evidence_ok {
            let reason = "cites unverified or missing source evidence".to_string();
            rejected_reasons.push(format!("{}: {}", interp.id, reason));
            evaluated_interpretations.push((interp.id.clone(), false, None, reason));
            continue;
        }

        // Verify that cited evidence includes at least one controlling anchor at or above Statutory rank
        // (Regulatory or AdministrativeGuidance alone—such as a standalone regulation without statutory anchor—cannot self-authenticate)
        let has_statutory_or_higher_evidence = interp.supporting_evidence_ids.iter().any(|ev_id| {
            argument
                .evidence
                .iter()
                .any(|ev| &ev.id == ev_id && ev.verified && ev.rank >= AuthorityRank::Statutory)
        });

        if !has_statutory_or_higher_evidence {
            let reason = "subordinate regulatory or administrative citation lacks a verified statutory or constitutional evidence anchor".to_string();
            rejected_reasons.push(format!("{}: {}", interp.id, reason));
            evaluated_interpretations.push((interp.id.clone(), false, None, reason));
            continue;
        }

        // Compute deterministic DGCS resonance score from derived coordinates [drift, symmetry, stability] using checked arithmetic
        let coords = interp.derived_primitive.coords;
        let drift = artificial_qubit_cognition::arithmetic::checked_mul(
            coords[0],
            field.state[2],
            "legal interp drift weighting",
        )?;
        let symmetry = artificial_qubit_cognition::arithmetic::checked_mul(
            coords[1],
            field.state[1],
            "legal interp symmetry weighting",
        )?;
        let stability = artificial_qubit_cognition::arithmetic::checked_mul(
            coords[2],
            field.state[0],
            "legal interp stability weighting",
        )?;
        let stab_plus_sym = checked_add(stability, symmetry, "legal interp stab+sym")?;
        let score = artificial_qubit_cognition::arithmetic::checked_sub(
            stab_plus_sym,
            drift,
            "legal interp total score",
        )?;

        let tag = if interp.requires_institutional_adjudication {
            "admissible:requires_institutional_adjudication".to_string()
        } else {
            "admissible:supported".to_string()
        };

        evaluated_interpretations.push((interp.id.clone(), true, Some(score), tag));
        admissible_scored.push((interp, score));
    }

    // Layer 6: Decision Trace & Non-Usurpation Disposition
    trajectory.apply_operator("layer6:emit_auditable_disposition");

    let disposition = if admissible_scored.is_empty() {
        trajectory.set_closure_status(ClosureStatus::Corrected);
        LegalDisposition::Contradicted { rejected_reasons }
    } else if admissible_scored.len() == 1 {
        let (winner, score) = admissible_scored[0];
        trajectory.set_closure_status(ClosureStatus::Closed);
        if winner.requires_institutional_adjudication {
            LegalDisposition::RequiresAuthoritativeInterpretation {
                candidate_interpretation_id: winner.id.clone(),
                institutional_question: format!(
                    "Interpretation '{}' is structurally admissible (score {}), but establishing binding legal meaning is reserved to a competent court or legislature.",
                    winner.id, score
                ),
            }
        } else {
            LegalDisposition::Supported {
                winning_interpretation_id: winner.id.clone(),
                resonance_score: score,
            }
        }
    } else {
        // Check if any admissible reading requires institutional adjudication or if competing readings tie/conflict
        let any_institutional = admissible_scored
            .iter()
            .any(|(i, _)| i.requires_institutional_adjudication);
        if any_institutional {
            trajectory.set_closure_status(ClosureStatus::Closed);
            let best = admissible_scored
                .iter()
                .max_by(|(a_i, a_s), (b_i, b_s)| a_s.cmp(b_s).then_with(|| b_i.id.cmp(&a_i.id)))
                .expect("non-empty admissible list");
            LegalDisposition::RequiresAuthoritativeInterpretation {
                candidate_interpretation_id: best.0.id.clone(),
                institutional_question: format!(
                    "Multiple admissible readings exist ({}); authoritative resolution between competing statutory/constitutional readings requires judicial or legislative determination.",
                    admissible_scored
                        .iter()
                        .map(|(i, s)| format!("{}:{}", i.id, s))
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
            }
        } else {
            // Check whether top admissible readings are tied in score -> Unresolved
            let max_score = admissible_scored
                .iter()
                .map(|(_, s)| *s)
                .max()
                .expect("non-empty admissible list");
            let tied_top: Vec<&CompetingInterpretation> = admissible_scored
                .iter()
                .filter(|(_, s)| *s == max_score)
                .map(|(i, _)| *i)
                .collect();

            if tied_top.len() > 1 {
                trajectory.set_closure_status(ClosureStatus::Closed);
                LegalDisposition::Unresolved {
                    admissible_interpretation_ids: tied_top
                        .iter()
                        .map(|i| i.id.clone())
                        .collect(),
                    open_questions: vec![format!(
                        "Competing admissible interpretations ({}) are tied at score {}; additional statutory/precedential evidence is required.",
                        tied_top
                            .iter()
                            .map(|i| i.id.as_str())
                            .collect::<Vec<_>>()
                            .join(", "),
                        max_score
                    )],
                }
            } else {
                trajectory.set_closure_status(ClosureStatus::Closed);
                let winner = tied_top[0];
                LegalDisposition::Supported {
                    winning_interpretation_id: winner.id.clone(),
                    resonance_score: max_score,
                }
            }
        }
    };

    trajectory.append_audit_event(format!("disposition:{}", disposition.kind_str()));

    let interp_summary = evaluated_interpretations
        .iter()
        .map(|(id, adm, score, _)| match (adm, score) {
            (true, Some(s)) => format!("{}:{}", id, s),
            _ => format!("{}:inadmissible", id),
        })
        .collect::<Vec<_>>()
        .join("|");

    let canonical_signature = format!(
        "legal:6layer|case:{}|class:{}|disposition:{}|{}",
        argument.id,
        argument.classification.as_str(),
        disposition.kind_str(),
        interp_summary
    );

    Ok(LegalEvaluationTrace {
        argument_id: argument.id.clone(),
        classification: argument.classification,
        claim: argument.claim.clone(),
        textual_observations,
        evaluated_interpretations,
        disposition,
        trajectory,
        canonical_signature,
    })
}

/// Construct the canonical three-classification Constitutional & Statutory Tax-Law Inquiry suite:
/// 1. `Case 1`: Citizens Residing in U.S. States (`CitizenResidingInStates`)
/// 2. `Case 2`: Citizens Resident Abroad (`CitizenResidentAbroad`)
/// 3. `Case 3`: Resident Aliens (`ResidentAlien`)
///
/// Explicitly models:
/// - The **Textual Observation** that `26 U.S.C. § 7701` defines `"person"` (§ 7701(a)(1)),
///   `"resident alien"` / `"nonresident alien"` (§ 7701(b)), and `"United States"` (§ 7701(a)(9)),
///   without a standalone entry titled `"individual"` in § 7701;
/// - Why jumping from that **Textual Observation** directly to a **Legal Conclusion** of categorical
///   exemption is blocked by the **No-Conflation Gate** (`I1_ObservationLeap`);
/// - Why dismissing statutory structure without textual examination is blocked by the
///   **Textual-Examination Gate** (`I2_HeuristicDismissal`);
/// - How contextual/statutory-structural readings (`I3_ContextualStatutorySynthesis`) and boundary questions
///   (`I4_ForeignResidenceBoundary`) are evaluated with full provenance and institutional humility.
pub fn canonical_statutory_classification_cases() -> Result<Vec<LegalArgument>, ArithmeticError> {
    let shared_authorities = vec![
        AuthorityGrant {
            id: "US_CONST_ART_I_S8_CL1".to_string(),
            citation: "U.S. Const. art. I, § 8, cl. 1".to_string(),
            rank: AuthorityRank::Constitutional,
            grant_scope: "Power to lay and collect Taxes, Duties, Imposts and Excises"
                .to_string(),
            explicit_constraints: vec!["Uniformity across the United States".to_string()],
            source_verified: true,
        },
        AuthorityGrant {
            id: "US_CONST_AMEND_XVI".to_string(),
            citation: "U.S. Const. amend. XVI".to_string(),
            rank: AuthorityRank::Constitutional,
            grant_scope:
                "Power to lay and collect taxes on incomes, from whatever source derived"
                    .to_string(),
            explicit_constraints: vec![
                "Without apportionment among the several States".to_string()
            ],
            source_verified: true,
        },
        AuthorityGrant {
            id: "IRC_26_USC_S1_S61_S7701".to_string(),
            citation: "26 U.S.C. §§ 1, 61, 871, 911, 7701".to_string(),
            rank: AuthorityRank::Statutory,
            grant_scope: "Enacted tax imposition, gross income, foreign earned income exclusion, and general definitions".to_string(),
            explicit_constraints: vec![
                "Statutory terms must be read in enacted context and statutory structure".to_string(),
            ],
            source_verified: true,
        },
    ];

    let shared_definitions = vec![
        TermDefinition {
            term: "individual".to_string(),
            statutory_section: "26 U.S.C. §§ 1, 7701(a)(1)".to_string(),
            has_standalone_statutory_definition: false,
            textual_observation: "§ 7701(a)(1) defines 'person' to include 'an individual, a trust, estate, partnership, association, company or corporation', without a separate standalone definition entry for 'individual' in § 7701.".to_string(),
            functional_role_in_context: "Distinguishes natural persons from artificial entities (trusts, estates, partnerships, corporations) across Subtitle A.".to_string(),
        },
        TermDefinition {
            term: "citizen / resident / resident alien".to_string(),
            statutory_section: "26 U.S.C. §§ 1, 871, 911, 7701(b)".to_string(),
            has_standalone_statutory_definition: true,
            textual_observation: "§ 7701(b) explicitly defines 'resident alien' and 'nonresident alien'; § 911 addresses 'qualified individual' citizens or residents living abroad.".to_string(),
            functional_role_in_context: "Allocates jurisdictional nexus and sourcing/exclusion rules among domestic citizens, citizens abroad, and alien residents/nonresidents.".to_string(),
        },
    ];

    let shared_evidence = vec![
        LegalEvidence {
            id: "EV_TEXT_7701".to_string(),
            source_citation: "26 U.S.C. § 7701(a)(1) & (b)".to_string(),
            rank: AuthorityRank::Statutory,
            summary: "Verifies exact statutory text of 'person' and alien residency definitions, confirming the textual observation regarding 'individual'.".to_string(),
            verified: true,
        },
        LegalEvidence {
            id: "EV_STRUCTURE_SUBTITLE_A".to_string(),
            source_citation: "26 U.S.C. §§ 1, 61, 911 & Historical Revenue Acts".to_string(),
            rank: AuthorityRank::Statutory,
            summary: "Verifies how Congress enacted distinct provisions for domestic residents, citizens abroad (§ 911), and resident/nonresident aliens (§§ 871, 7701(b)).".to_string(),
            verified: true,
        },
        LegalEvidence {
            id: "EV_JUDICIAL_PRECEDENT".to_string(),
            source_citation: "Controlling Supreme Court & Circuit Statutory Interpretation Precedent"
                .to_string(),
            rank: AuthorityRank::JudicialPrecedent,
            summary: "Establishes that undefined statutory terms take their ordinary meaning within the surrounding statutory scheme rather than becoming legal nullities.".to_string(),
            verified: true,
        },
    ];

    let shared_constraints = vec![
        InterpretiveConstraint {
            id: "C1_NO_CONFLATION".to_string(),
            rule: "Do not jump from a textual observation (e.g., absence of a standalone definition in § 7701) directly to a legal conclusion without full statutory, contextual, and precedential analysis.".to_string(),
        },
        InterpretiveConstraint {
            id: "C2_NO_HEURISTIC_DISMISSAL".to_string(),
            rule: "Do not dismiss a textual or structural question without examining the exact statutory text, definitions, and historical enactment.".to_string(),
        },
        InterpretiveConstraint {
            id: "C3_NON_USURPATION".to_string(),
            rule: "DGCS structures and audits legal reasoning; it does not usurp judicial or legislative authority on open questions.".to_string(),
        },
    ];

    // ── CASE 1: Citizens Residing in U.S. States ──────────────────────────────
    let case_1 = LegalArgument {
        id: "Case1_DomesticCitizen".to_string(),
        classification: TaxpayerClassification::CitizenResidingInStates,
        claim: "Determine how the 6-layer DGCS framework evaluates statutory classification for citizens residing in U.S. states while separating textual observation from legal conclusion.".to_string(),
        authorities: shared_authorities.clone(),
        enactments: vec![StatutoryEnactment {
            citation: "26 U.S.C. §§ 1, 61, 7701(a)(1)".to_string(),
            classification: TaxpayerClassification::CitizenResidingInStates,
            enacted_text_summary: "Imposes tax on taxable income of every individual; includes 'individual' within 'person'.".to_string(),
            historical_lineage: "Revenue Act of 1913 through Internal Revenue Code of 1986".to_string(),
            text_verified: true,
        }],
        definitions: shared_definitions.clone(),
        evidence: shared_evidence.clone(),
        constraints: shared_constraints.clone(),
        alternatives: vec![
            CompetingInterpretation::from_observable_features(
                "I1_ObservationLeap",
                "Leaps directly from the textual observation that 'individual' lacks a standalone § 7701 definition to the legal conclusion that no citizen is within the term.",
                true,  // Conflates textual observation with legal conclusion!
                false,
                false,
                false,
                vec!["EV_TEXT_7701".to_string()],
                ObservableFeatures {
                    intermediaries: 3,
                    rights: 4,
                    duties: 1,
                    protection: 2,
                    exposure: 4,
                },
            )?,
            CompetingInterpretation::from_observable_features(
                "I2_HeuristicDismissal",
                "Refuses to inspect § 7701 or statutory history at all, asserting a conclusion by summary heuristic without textual verification.",
                false,
                true, // Dismisses without examining text!
                true,
                false,
                vec!["EV_JUDICIAL_PRECEDENT".to_string()],
                ObservableFeatures {
                    intermediaries: 2,
                    rights: 2,
                    duties: 2,
                    protection: 3,
                    exposure: 2,
                },
            )?,
            CompetingInterpretation::from_observable_features(
                "I3_ContextualStatutorySynthesis",
                "Records the § 7701 textual observation accurately, then synthesizes §§ 1, 61, 7701(a)(1), historical enactments, and controlling precedent to evaluate 'individual' in its enacted statutory context.",
                false,
                false,
                true,
                false,
                vec![
                    "EV_TEXT_7701".to_string(),
                    "EV_STRUCTURE_SUBTITLE_A".to_string(),
                    "EV_JUDICIAL_PRECEDENT".to_string(),
                ],
                ObservableFeatures {
                    intermediaries: 0,
                    rights: 4,
                    duties: 4,
                    protection: 7,
                    exposure: 1,
                }, // coords = [0, 4, 6] -> score = 10
            )?,
        ],
    };

    // ── CASE 2: Citizens Resident Abroad ──────────────────────────────────────
    let case_2 = LegalArgument {
        id: "Case2_CitizenAbroad".to_string(),
        classification: TaxpayerClassification::CitizenResidentAbroad,
        claim: "Evaluate statutory interaction between citizenship nexus, foreign residence (§ 911), and bilateral treaty / constitutional scope for citizens resident abroad.".to_string(),
        authorities: shared_authorities.clone(),
        enactments: vec![StatutoryEnactment {
            citation: "26 U.S.C. §§ 1, 911, 7701(a)(30)".to_string(),
            classification: TaxpayerClassification::CitizenResidentAbroad,
            enacted_text_summary: "Provides specific exclusions and housing/foreign-earned income rules for qualified individuals who are citizens or residents living abroad.".to_string(),
            historical_lineage: "Foreign Earned Income Act & Subtitle A treaty-coordination provisions".to_string(),
            text_verified: true,
        }],
        definitions: shared_definitions.clone(),
        evidence: shared_evidence.clone(),
        constraints: shared_constraints.clone(),
        alternatives: vec![
            CompetingInterpretation::from_observable_features(
                "I1_UnverifiedExemptionLeap",
                "Ignores § 911 enactment structure and leaps from textual silence to categorical non-application.",
                true,
                false,
                false,
                false,
                vec!["EV_TEXT_7701".to_string()],
                ObservableFeatures {
                    intermediaries: 3,
                    rights: 4,
                    duties: 1,
                    protection: 2,
                    exposure: 4,
                },
            )?,
            CompetingInterpretation::from_observable_features(
                "I2_ForeignResidenceStatutoryAndTreatyAnalysis",
                "Maps the enacted § 911 exclusion and treaty-tiebreaker interaction, while flagging contested extraterritorial/treaty-priority boundary questions for authoritative judicial/legislative resolution.",
                false,
                false,
                true,
                true, // Flags institutional boundary where treaty/extraterritorial scope is contested!
                vec![
                    "EV_TEXT_7701".to_string(),
                    "EV_STRUCTURE_SUBTITLE_A".to_string(),
                    "EV_JUDICIAL_PRECEDENT".to_string(),
                ],
                ObservableFeatures {
                    intermediaries: 1,
                    rights: 3,
                    duties: 3,
                    protection: 6,
                    exposure: 1,
                }, // coords = [1, 3, 5] -> score = 5 + 3 - 1 = 7
            )?,
        ],
    };

    // ── CASE 3: Resident Aliens ───────────────────────────────────────────────
    let case_3 = LegalArgument {
        id: "Case3_ResidentAlien".to_string(),
        classification: TaxpayerClassification::ResidentAlien,
        claim: "Evaluate explicit statutory definition of 'resident alien' under 26 U.S.C. § 7701(b) (lawful permanent residence / substantial presence) and its structural role in Subtitle A.".to_string(),
        authorities: shared_authorities,
        enactments: vec![StatutoryEnactment {
            citation: "26 U.S.C. §§ 7701(b)(1)(A), 871".to_string(),
            classification: TaxpayerClassification::ResidentAlien,
            enacted_text_summary: "Explicitly defines 'resident alien' via green-card, substantial-presence, or first-year election tests in contrast to 'nonresident alien'.".to_string(),
            historical_lineage: "Deficit Reduction Act of 1984 § 138 (enacting § 7701(b))".to_string(),
            text_verified: true,
        }],
        definitions: shared_definitions,
        evidence: shared_evidence,
        constraints: shared_constraints,
        alternatives: vec![
            CompetingInterpretation::from_observable_features(
                "I1_IgnoreExplicit7701bDefinition",
                "Ignores the explicit statutory definition in § 7701(b)(1)(A).",
                true,
                false,
                false,
                false,
                vec!["EV_TEXT_7701".to_string()],
                ObservableFeatures {
                    intermediaries: 2,
                    rights: 3,
                    duties: 1,
                    protection: 1,
                    exposure: 4,
                },
            )?,
            CompetingInterpretation::from_observable_features(
                "I2_Enacted7701bStatutoryApplication",
                "Applies the explicit § 7701(b)(1)(A) statutory tests alongside the contrast between 'individual' and 'resident alien' in the text.",
                false,
                false,
                true,
                false,
                vec![
                    "EV_TEXT_7701".to_string(),
                    "EV_STRUCTURE_SUBTITLE_A".to_string(),
                    "EV_JUDICIAL_PRECEDENT".to_string(),
                ],
                ObservableFeatures {
                    intermediaries: 0,
                    rights: 3,
                    duties: 3,
                    protection: 6,
                    exposure: 1,
                }, // coords = [0, 3, 5] -> score = 8
            )?,
        ],
    };

    Ok(vec![case_1, case_2, case_3])
}

#[cfg(test)]
mod tests {
    use super::{
        canonical_statutory_classification_cases, evaluate_legal_argument, LegalDisposition,
    };

    #[test]
    fn six_layer_legal_reasoning_prevents_conflation_and_enforces_institutional_boundary() {
        let cases = canonical_statutory_classification_cases().unwrap();
        assert_eq!(cases.len(), 3);

        // Case 1: Domestic Citizen -> blocks I1 (conflation leap) and I2 (heuristic dismissal),
        // admits I3 (contextual synthesis) as Supported
        let trace_1 = evaluate_legal_argument(&cases[0]).unwrap();
        assert_eq!(
            trace_1.disposition,
            LegalDisposition::Supported {
                winning_interpretation_id: "I3_ContextualStatutorySynthesis".to_string(),
                resonance_score: 10,
            }
        );
        assert_eq!(
            trace_1.canonical_signature,
            "legal:6layer|case:Case1_DomesticCitizen|class:citizen_residing_in_us_states|disposition:Supported|I1_ObservationLeap:inadmissible|I2_HeuristicDismissal:inadmissible|I3_ContextualStatutorySynthesis:10"
        );

        // Case 2: Citizen Abroad -> blocks I1, recognizes I2 requires institutional adjudication on contested boundary
        let trace_2 = evaluate_legal_argument(&cases[1]).unwrap();
        assert!(matches!(
            trace_2.disposition,
            LegalDisposition::RequiresAuthoritativeInterpretation { .. }
        ));
        assert_eq!(
            trace_2.canonical_signature,
            "legal:6layer|case:Case2_CitizenAbroad|class:citizen_resident_abroad|disposition:RequiresAuthoritativeInterpretation|I1_UnverifiedExemptionLeap:inadmissible|I2_ForeignResidenceStatutoryAndTreatyAnalysis:7"
        );

        // Case 3: Resident Alien -> blocks I1, supports I2 under explicit § 7701(b) definition
        let trace_3 = evaluate_legal_argument(&cases[2]).unwrap();
        assert_eq!(
            trace_3.disposition,
            LegalDisposition::Supported {
                winning_interpretation_id: "I2_Enacted7701bStatutoryApplication".to_string(),
                resonance_score: 8,
            }
        );
        assert_eq!(
            trace_3.canonical_signature,
            "legal:6layer|case:Case3_ResidentAlien|class:resident_alien|disposition:Supported|I1_IgnoreExplicit7701bDefinition:inadmissible|I2_Enacted7701bStatutoryApplication:8"
        );
    }
}

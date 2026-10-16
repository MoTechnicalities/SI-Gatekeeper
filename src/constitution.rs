use artificial_qubit_cognition::{
    creativity::{CreativityPolicy, SelectionWeights},
    geom::resonance_field::ResonanceField,
};

/// External fact attestation supplied to a DGCS Constitution.
///
/// Crucially, DGCS separates **External Legal/Empirical Validity** (whether a candidate
/// complies with applicable law and verified physical/empirical facts) from
/// **Model Constitutional Validity** (whether the candidate satisfies the declared
/// DGCS constitution after those facts are supplied).
///
/// Signatures support replay and audit of the declared inputs, not truth by assertion.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExternalFactAttestation {
    /// Fact identifier (e.g., `"mandatory_law_compliance"`, `"duty_allocation_complete"`).
    pub fact_id: String,
    /// Externally verified status supplied by qualified review or sensor/authority evidence.
    pub verified: bool,
    /// Provenance or authority source description.
    pub evidence_source: String,
}

impl ExternalFactAttestation {
    pub fn new(
        fact_id: impl Into<String>,
        verified: bool,
        evidence_source: impl Into<String>,
    ) -> Self {
        Self {
            fact_id: fact_id.into(),
            verified,
            evidence_source: evidence_source.into(),
        }
    }

    pub fn canonical_tag(&self) -> String {
        format!("{}:{}", self.fact_id, self.verified)
    }
}

/// First-class DGCS Policy Constitution artifact.
///
/// A `Constitution` locks the governing rules *before* any candidate trajectory or
/// system $SI_{n+1}$ is evaluated. A candidate generator may propose states or trajectories,
/// but is architecturally barred from altering the constitution, its required invariants,
/// its required external fact attestations, or its resonance objective during evaluation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Constitution {
    /// Jurisdiction or domain identifier (e.g., `"federal-si-genesis"`, `"state-michigan"`, `"grid-nerc"`).
    pub jurisdiction_id: String,
    /// Constitutional version tag (e.g., `"v1.0"`).
    pub version: String,
    /// Non-negotiable structural invariant identifiers required at Gate Stage 1 ($I$).
    pub mandatory_invariants: Vec<String>,
    /// Mandatory external fact attestation IDs that must be verified `true` at Gate Stage 2 ($V$).
    pub required_external_facts: Vec<String>,
    /// Maximum allowable aggregate drift on the `x` axis (`coords[0]`).
    pub max_allowable_drift: i32,
    /// Minimum required aggregate stability on the `z` axis (`coords[2]`).
    pub min_required_stability: i32,
    /// Fixed resonance field weights (`[stability_weight, symmetry_weight, drift_penalty]`).
    pub resonance_field: ResonanceField,
    /// Optional novelty policy ($N$) for Constitutional Self-Improvement ($SI_n \to SI_{n+1}$).
    pub creativity_policy: Option<CreativityPolicy>,
    /// Fixed selection weights combining utility and lawful novelty.
    pub selection_weights: SelectionWeights,
}

impl Constitution {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        jurisdiction_id: impl Into<String>,
        version: impl Into<String>,
        mandatory_invariants: Vec<String>,
        required_external_facts: Vec<String>,
        max_allowable_drift: i32,
        min_required_stability: i32,
        resonance_field: ResonanceField,
        creativity_policy: Option<CreativityPolicy>,
        selection_weights: SelectionWeights,
    ) -> Self {
        Self {
            jurisdiction_id: jurisdiction_id.into(),
            version: version.into(),
            mandatory_invariants,
            required_external_facts,
            max_allowable_drift,
            min_required_stability,
            resonance_field,
            creativity_policy,
            selection_weights,
        }
    }

    /// Canonical, replay-stable signature of the constitution itself.
    ///
    /// Any attempt to alter weights, thresholds, or required facts changes this signature.
    pub fn constitutional_signature(&self) -> String {
        let invariants = if self.mandatory_invariants.is_empty() {
            "none".to_string()
        } else {
            self.mandatory_invariants.join(",")
        };
        let facts = if self.required_external_facts.is_empty() {
            "none".to_string()
        } else {
            self.required_external_facts.join(",")
        };

        format!(
            "constitution:{}|ver:{}|invariants:[{}]|facts:[{}]|max_drift:{}|min_stab:{}|field:[{},{},{}]|weights:[u:{},c:{}]",
            self.jurisdiction_id,
            self.version,
            invariants,
            facts,
            self.max_allowable_drift,
            self.min_required_stability,
            self.resonance_field.state[0],
            self.resonance_field.state[1],
            self.resonance_field.state[2],
            self.selection_weights.utility,
            self.selection_weights.creativity,
        )
    }

    /// Verify whether a candidate's supplied external fact attestations satisfy all
    /// mandatory external facts required by this constitution.
    pub fn check_external_facts(
        &self,
        attestations: &[ExternalFactAttestation],
    ) -> Result<(), String> {
        for required in &self.required_external_facts {
            match attestations.iter().find(|a| &a.fact_id == required) {
                Some(att) if att.verified => continue,
                Some(att) => {
                    return Err(format!(
                        "external fact '{}' is unverified (source: {})",
                        required, att.evidence_source
                    ));
                }
                None => {
                    return Err(format!(
                        "missing mandatory external fact attestation '{}'",
                        required
                    ));
                }
            }
        }
        Ok(())
    }

    /// Verify whether a candidate's satisfied invariant tags include all non-negotiable
    /// constitutional invariants.
    pub fn check_invariants(&self, satisfied_invariants: &[String]) -> Result<(), String> {
        for required in &self.mandatory_invariants {
            if !satisfied_invariants.iter().any(|inv| inv == required) {
                return Err(format!(
                    "non-negotiable invariant '{}' violated or missing",
                    required
                ));
            }
        }
        Ok(())
    }
}

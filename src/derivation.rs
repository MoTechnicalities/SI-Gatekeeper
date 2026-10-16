use artificial_qubit_cognition::arithmetic::{checked_abs, checked_sub, ArithmeticError};

/// Observable structural features of a governance or operational primitive.
///
/// Rather than assigning coordinates in $\mathbb{Z}^3$ (`[drift, symmetry, stability]`)
/// by hand, `ObservableFeatures` derives them deterministically from declared structural counts:
///
/// - `drift = intermediaries` (number of intermediary structures between operator and action)
/// - `symmetry = min(rights, duties) - |rights - duties|`
/// - `stability = protection - exposure`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ObservableFeatures {
    /// Number of intermediary layers or structures between the operator and direct action.
    pub intermediaries: i32,
    /// Verified rights / decision authorities allocated to the unit.
    pub rights: i32,
    /// Verified duties / accountabilities bound to the unit.
    pub duties: i32,
    /// Verified structural or legal protections supporting the unit.
    pub protection: i32,
    /// Unmitigated operational, legal, or physical exposure carried by the unit.
    pub exposure: i32,
}

/// A derived primitive AQ with its observable feature provenance and canonical signature.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DerivedPrimitiveAQ {
    pub kind: String,
    pub features: ObservableFeatures,
    /// Canonical DGCS axis layout: `[drift, symmetry, stability]`.
    pub coords: [i32; 3],
    pub signature: String,
}

impl ObservableFeatures {
    /// Deterministically derive `[drift, symmetry, stability]` in $\mathbb{Z}^3$.
    ///
    /// Formulas:
    /// - `drift = intermediaries`
    /// - `symmetry = min(rights, duties) - |rights - duties|`
    /// - `stability = protection - exposure`
    pub fn derive_coords(&self) -> Result<[i32; 3], ArithmeticError> {
        let drift = self.intermediaries;
        let min_rd = self.rights.min(self.duties);
        let diff_rd = checked_sub(self.rights, self.duties, "symmetry rights-duties delta")?;
        let imbalance = checked_abs(diff_rd, "symmetry rights-duties imbalance")?;
        let symmetry = checked_sub(min_rd, imbalance, "symmetry derivation")?;
        let stability = checked_sub(self.protection, self.exposure, "stability derivation")?;

        Ok([drift, symmetry, stability])
    }
}

impl DerivedPrimitiveAQ {
    /// Construct a primitive AQ by deriving its $\mathbb{Z}^3$ coordinates from observable features.
    pub fn from_features(
        kind: impl Into<String>,
        features: ObservableFeatures,
    ) -> Result<Self, ArithmeticError> {
        let kind = kind.into();
        let coords = features.derive_coords()?;
        let signature = format!(
            "paq:{}|coords:[{},{},{}]|features:[i:{},r:{},d:{},p:{},e:{}]",
            kind,
            coords[0],
            coords[1],
            coords[2],
            features.intermediaries,
            features.rights,
            features.duties,
            features.protection,
            features.exposure
        );

        Ok(Self {
            kind,
            features,
            coords,
            signature,
        })
    }
}

/// Canonical compliance primitives from the Constitutional Compliance-Architecture scenario,
/// derived directly from observable structural features.
pub mod canonical_compliance_primitives {
    use super::{DerivedPrimitiveAQ, ObservableFeatures};
    use artificial_qubit_cognition::arithmetic::ArithmeticError;

    /// `paq:entity_intermediation` -> `[3, -2, 2]`
    /// - 3 intermediaries (state charter, registered agent, corporate veil/filing layer)
    /// - rights = 4, duties = 1 -> min(4,1) - |4-1| = 1 - 3 = -2
    /// - protection = 4, exposure = 2 -> 4 - 2 = 2
    pub fn entity_intermediation() -> Result<DerivedPrimitiveAQ, ArithmeticError> {
        DerivedPrimitiveAQ::from_features(
            "entity_intermediation",
            ObservableFeatures {
                intermediaries: 3,
                rights: 4,
                duties: 1,
                protection: 4,
                exposure: 2,
            },
        )
    }

    /// `paq:direct_operation` -> `[0, 3, 5]`
    /// - 0 intermediaries (direct execution between operator and action)
    /// - rights = 3, duties = 3 -> min(3,3) - |3-3| = 3 - 0 = 3
    /// - protection = 6, exposure = 1 -> 6 - 1 = 5
    pub fn direct_operation() -> Result<DerivedPrimitiveAQ, ArithmeticError> {
        DerivedPrimitiveAQ::from_features(
            "direct_operation",
            ObservableFeatures {
                intermediaries: 0,
                rights: 3,
                duties: 3,
                protection: 6,
                exposure: 1,
            },
        )
    }

    /// `paq:unmitigated_exposure` -> `[2, -1, -3]`
    /// - 2 intermediaries (informal pass-through / unshielded agency chain)
    /// - rights = 1, duties = 3 -> min(1,3) - |1-3| = 1 - 2 = -1
    /// - protection = 1, exposure = 4 -> 1 - 4 = -3
    pub fn unmitigated_exposure() -> Result<DerivedPrimitiveAQ, ArithmeticError> {
        DerivedPrimitiveAQ::from_features(
            "unmitigated_exposure",
            ObservableFeatures {
                intermediaries: 2,
                rights: 1,
                duties: 3,
                protection: 1,
                exposure: 4,
            },
        )
    }

    /// `paq:structured_contract` -> `[1, 2, 4]`
    /// - 1 intermediary (explicit bilateral/trust instrument)
    /// - rights = 3, duties = 4 -> min(3,4) - |3-4| = 3 - 1 = 2
    /// - protection = 5, exposure = 1 -> 5 - 1 = 4
    pub fn structured_contract() -> Result<DerivedPrimitiveAQ, ArithmeticError> {
        DerivedPrimitiveAQ::from_features(
            "structured_contract",
            ObservableFeatures {
                intermediaries: 1,
                rights: 3,
                duties: 4,
                protection: 5,
                exposure: 1,
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::canonical_compliance_primitives::*;
    use super::ObservableFeatures;

    #[test]
    fn operational_derivation_reproduces_all_four_compliance_coordinates() {
        let intermediation = entity_intermediation().unwrap();
        let direct = direct_operation().unwrap();
        let exposure = unmitigated_exposure().unwrap();
        let contract = structured_contract().unwrap();

        assert_eq!(intermediation.coords, [3, -2, 2]);
        assert_eq!(direct.coords, [0, 3, 5]);
        assert_eq!(exposure.coords, [2, -1, -3]);
        assert_eq!(contract.coords, [1, 2, 4]);
    }

    #[test]
    fn symmetry_penalizes_rights_duties_imbalance() {
        let balanced = ObservableFeatures {
            intermediaries: 0,
            rights: 4,
            duties: 4,
            protection: 5,
            exposure: 1,
        };
        let imbalanced = ObservableFeatures {
            intermediaries: 0,
            rights: 6,
            duties: 2,
            protection: 5,
            exposure: 1,
        };

        assert_eq!(balanced.derive_coords().unwrap(), [0, 4, 4]);
        assert_eq!(imbalanced.derive_coords().unwrap(), [0, -2, 4]);
    }
}

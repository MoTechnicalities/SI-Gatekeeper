#!/usr/bin/env python3
"""
SI-Gatekeeper — Minimal Python Prototype for DGCS Coordinate Derivation & Constitutional Gating.

Demonstrates how domain experts can prototype observable features, derive exact Z^3
coordinates ([drift, symmetry, stability]), and run a short-circuiting Constitutional Gate
in plain Python that emits the exact same canonical tournament signature as the compiled
Rust `compliance_tournament` binary:

    compliance:metaAQ|winner:C|A:invalid|B:invalid|C:64|D:48
"""

from dataclasses import dataclass
from typing import List, Tuple


@dataclass(frozen=True)
class ObservableFeatures:
    intermediaries: int  # Drift axis (number of intermediary layers between operator and action)
    rights: int          # Verified decision authorities allocated to the unit
    duties: int          # Verified accountabilities bound to the unit
    protection: int      # Verified structural or legal protection
    exposure: int        # Unmitigated operational, legal, or physical exposure

    def derive_coords(self) -> Tuple[int, int, int]:
        """Derive [drift, symmetry, stability] in Z^3 without arbitrary assertion."""
        drift = self.intermediaries
        symmetry = min(self.rights, self.duties) - abs(self.rights - self.duties)
        stability = self.protection - self.exposure
        return (drift, symmetry, stability)


@dataclass
class CandidatePlan:
    plan_id: str
    features: List[ObservableFeatures]
    active_indices: List[int]
    coherence_bonus: int
    invariants_ok: bool
    external_facts_verified: bool
    topology_valid: bool


def evaluate_tournament(domain: str, plans: List[CandidatePlan], max_drift: int = 12) -> str:
    summary, valid_scores = [], []
    for p in plans:
        coords = [f.derive_coords() for f in p.features]
        drift = sum(c[0] for c in coords)
        symmetry = sum(coords[i][1] for i in p.active_indices)
        stability = sum(c[2] for c in coords)

        # Short-circuiting Gate: Invariant -> Validity -> Utility
        if not (
            p.invariants_ok
            and drift <= max_drift
            and p.external_facts_verified
            and p.topology_valid
        ):
            summary.append(f"{p.plan_id}:invalid")
            continue

        score = stability + symmetry - drift + p.coherence_bonus
        summary.append(f"{p.plan_id}:{score}")
        valid_scores.append((score, p.plan_id))

    winner = max(valid_scores)[1] if valid_scores else "none"
    return f"{domain}:metaAQ|winner:{winner}|" + "|".join(summary)


if __name__ == "__main__":
    p_intermed = ObservableFeatures(3, 4, 1, 4, 2)  # -> (3, -2, 2)
    p_direct   = ObservableFeatures(0, 3, 3, 6, 1)  # -> (0, 3, 5)
    p_exposure = ObservableFeatures(2, 1, 3, 1, 4)  # -> (2, -1, -3)
    p_contract = ObservableFeatures(1, 3, 4, 5, 1)  # -> (1, 2, 4)
    p_reserve  = ObservableFeatures(0, 2, 2, 3, 1)  # -> (0, 2, 2)

    assert p_intermed.derive_coords() == (3, -2, 2)
    assert p_direct.derive_coords()   == (0, 3, 5)
    assert p_exposure.derive_coords() == (2, -1, -3)
    assert p_contract.derive_coords() == (1, 2, 4)

    plans = [
        CandidatePlan("A", [p_intermed, p_contract], [0, 1], 0, True, True, False),
        CandidatePlan("B", [p_direct, p_exposure],   [0, 1], 0, True, True, False),
        CandidatePlan(
            "C",
            [p_direct] * 6 + [p_contract] * 2 + [p_reserve],
            list(range(7)),
            6,
            True,
            True,
            True,
        ),
        CandidatePlan(
            "D",
            [p_direct] * 4 + [p_contract] * 2 + [p_reserve] * 2,
            list(range(5)),
            4,
            True,
            True,
            True,
        ),
    ]

    signature = evaluate_tournament("compliance", plans)
    print(signature)
    assert signature == "compliance:metaAQ|winner:C|A:invalid|B:invalid|C:64|D:48"

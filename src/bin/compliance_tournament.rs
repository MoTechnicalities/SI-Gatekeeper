use artificial_qubit_cognition::geom::correction_buffer::CorrectionBuffer;
use si_gatekeeper::{
    derivation::canonical_compliance_primitives::*,
    gatekeeper::GateVerdict,
    run_compliance_tournament,
};

fn main() {
    println!("================================================================================");
    println!("  SI-GATEKEEPER — CONSTITUTIONAL COMPLIANCE-ARCHITECTURE TOURNAMENT");
    println!("  Open-sourced by the people for the people of the world.");
    println!("================================================================================\n");

    println!("1. Operational Coordinate Derivation (Solving Fault 2 — Derived, Not Asserted):");
    println!("   drift = intermediaries | symmetry = min(R,D) - |R-D| | stability = protection - exposure");
    for primitive in [
        entity_intermediation().unwrap(),
        direct_operation().unwrap(),
        unmitigated_exposure().unwrap(),
        structured_contract().unwrap(),
    ] {
        println!("   - {}", primitive.signature);
    }
    println!();

    let mut buffer = CorrectionBuffer::new();
    let result = run_compliance_tournament(&mut buffer)
        .expect("canonical compliance tournament must succeed");

    println!("2. Constitutional Gatekeeper Evaluation (Invariant -> Validity -> Novelty -> Utility -> Improvement):");
    for receipt in &result.receipts_in_order {
        match &receipt.verdict {
            GateVerdict::Admitted {
                resonance_score, ..
            } => {
                println!(
                    "   [ADMITTED] Plan {} | Score: {:>3} | Sig: {}",
                    receipt.candidate_id, resonance_score, receipt.signature
                );
            }
            GateVerdict::Rejected {
                failed_stage,
                reason,
                ..
            } => {
                println!(
                    "   [REJECTED] Plan {} | Stage: {:<9} | Reason: {}",
                    receipt.candidate_id,
                    failed_stage.as_str(),
                    reason
                );
            }
        }
    }

    println!("\n3. Canonical Replay-Verifiable Tournament Signature:");
    println!("   {}", result.tournament_signature);
    println!("   CorrectionBuffer Archived Valid Fallbacks: {}", buffer.len());
}

use artificial_qubit_cognition::geom::correction_buffer::CorrectionBuffer;
use si_gatekeeper::{run_self_improvement_demo, GateVerdict};

fn main() {
    println!("================================================================================");
    println!("  SI-GATEKEEPER — 5-STAGE CONSTITUTIONAL SELF-IMPROVEMENT GATE (SI_n -> SI_{{n+1}})");
    println!("  Open-sourced by the people for the people of the world.");
    println!("================================================================================\n");

    println!("Gate Hierarchy : Invariant -> Validity -> Novelty -> Utility -> Improvement");
    println!("Predicate      : AdmissibleImprovement(SI_{{n+1}}, SI_n) = I ^ V ^ N ^ R ^ (U(SI_{{n+1}}) > U(SI_n))\n");

    let mut buffer = CorrectionBuffer::new();
    let (baseline_u, receipts) =
        run_self_improvement_demo(&mut buffer).expect("self-improvement demo must succeed");

    println!("Baseline System: SI_n_Baseline | Utility U(SI_n) = {}\n", baseline_u);

    for receipt in &receipts {
        match &receipt.verdict {
            GateVerdict::Admitted {
                resonance_score,
                combined_selection_score,
                novelty,
            } => {
                let dist = novelty
                    .as_ref()
                    .map(|n| n.distance_from_expected())
                    .unwrap_or(0);
                println!(
                    "  [CERTIFIED SI_{{n+1}}] {}\n    Utility U(SI_{{n+1}}): {} > {} | Novelty Distance: {} | Combined Score: {}\n    Receipt: {}\n",
                    receipt.candidate_id,
                    resonance_score,
                    baseline_u,
                    dist,
                    combined_selection_score,
                    receipt.signature
                );
            }
            GateVerdict::Rejected {
                failed_stage,
                reason,
                fallback,
            } => {
                let fb_id = fallback
                    .as_ref()
                    .map(|(id, _)| id.as_str())
                    .unwrap_or("none");
                println!(
                    "  [BLOCKED at {:<11}] {}\n    Reason  : {}\n    Fallback: Retained {}\n",
                    failed_stage.as_str(),
                    receipt.candidate_id,
                    reason,
                    fb_id
                );
            }
        }
    }
}

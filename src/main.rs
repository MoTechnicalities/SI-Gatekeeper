use artificial_qubit_cognition::geom::correction_buffer::CorrectionBuffer;
use si_gatekeeper::{
    canonical_statutory_classification_cases, evaluate_legal_argument,
    run_compliance_tournament, run_infrastructure_arbitration_demo, run_self_improvement_demo,
};

fn main() {
    println!("================================================================================");
    println!("  SI-GATEKEEPER: THE GLOBAL CONSTITUTIONAL STANDARD FOR SUPER INTELLIGENCE");
    println!("  \"Open-sourced by the people for the people of the world.\"");
    println!("  Scale can establish market dominance, but a defensible technical standard");
    println!("  must establish why systems should be trusted.");
    println!("================================================================================\n");

    let mut compliance_buffer = CorrectionBuffer::new();
    let compliance = run_compliance_tournament(&mut compliance_buffer)
        .expect("compliance tournament must succeed");

    println!("[Phase I] Constitutional Compliance-Architecture Verification");
    println!("  Gate Order : Invariant -> Validity -> Novelty -> Utility -> Improvement");
    println!("  Signature  : {}", compliance.tournament_signature);
    println!("  Winner     : Plan {}", compliance.winner.candidate_id);
    println!();

    let infra = run_infrastructure_arbitration_demo()
        .expect("infrastructure arbitration must succeed");

    println!("[Phase II] Multi-Domain SI Infrastructure & Multi-Jurisdictional Arbitration");
    println!("  Bridge Domain Signature     : {}", infra.bridge_tournament_signature);
    println!("  Power Grid Domain Signature : {}", infra.grid_tournament_signature);
    println!("  Composite SI Signature      : {}", infra.composite_tournament_signature);
    println!("  Selected Deployment Plan    : Plan {}", infra.winning_composite_id);
    println!();

    println!("[Phase III] Zero-Drift CorrectionBuffer Failover Verification");
    println!(
        "  Blocked Unverified Candidate '{}' at stage [{}]",
        infra.fallback_recovery_demo.anomalous_candidate_id,
        infra.fallback_recovery_demo.failed_stage.as_str()
    );
    println!(
        "  Reason   : {}",
        infra.fallback_recovery_demo.failure_reason
    );
    println!(
        "  Fallback : Plan {} -> {}",
        infra.fallback_recovery_demo.recovered_plan_id,
        infra.fallback_recovery_demo.recovered_signature
    );
    println!();

    println!("[Phase IV] 6-Layer Constitutional & Statutory Legal Reasoning Verification");
    let cases = canonical_statutory_classification_cases()
        .expect("statutory classification cases must build");
    for case in &cases {
        let trace = evaluate_legal_argument(case).expect("legal evaluation must succeed");
        println!("  - {}", trace.canonical_signature);
    }
    println!();

    println!("[Phase V] 5-Stage Recursive Self-Improvement Gate (SI_n -> SI_{{n+1}})");
    let mut si_buffer = CorrectionBuffer::new();
    let (_, si_receipts) =
        run_self_improvement_demo(&mut si_buffer).expect("self-improvement demo must succeed");
    for r in &si_receipts {
        println!("  - {}", r.signature);
    }
    println!("================================================================================");
}

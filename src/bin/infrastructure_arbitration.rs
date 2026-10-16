use si_gatekeeper::run_infrastructure_arbitration_demo;

fn main() {
    println!("================================================================================");
    println!("  SI-GATEKEEPER — DEMO 2: MULTI-DOMAIN SI INFRASTRUCTURE & CONSTITUTIONAL ARBITRATION");
    println!("  Open-sourced by the people for the people of the world.");
    println!("================================================================================\n");

    let report = run_infrastructure_arbitration_demo()
        .expect("multi-domain infrastructure arbitration demo must succeed");

    println!("1. Imported AQC/DGCS Domain Tournament Signatures:");
    println!("   - Structural Engineering : {}", report.bridge_tournament_signature);
    println!("   - Power Grid Routing     : {}", report.grid_tournament_signature);
    println!("   - Constitutional Policy  : {}", report.compliance_tournament_signature);
    println!();

    println!("2. Multi-Domain & Multi-Jurisdictional SI Infrastructure Candidates:");
    println!("   (Evaluated across Structural, Power Grid, State Charter, Federal SI Genesis, and Global Open Standard)");
    for c in &report.composite_candidates {
        match c.composite_score {
            Some(score) => {
                println!(
                    "   [ADMITTED] Plan {} ({})\n              Structural: {} | Grid: {} | Constitutional Federation: {} | Total Score: {}",
                    c.plan_id,
                    c.description,
                    c.structural_status,
                    c.grid_status,
                    c.constitutional_status,
                    score
                );
            }
            None => {
                println!(
                    "   [REJECTED] Plan {} ({})\n              Structural: {} | Grid: {} | Constitutional Federation: {}\n              Reason: {}",
                    c.plan_id,
                    c.description,
                    c.structural_status,
                    c.grid_status,
                    c.constitutional_status,
                    c.rejection_reason.as_deref().unwrap_or("unknown")
                );
            }
        }
    }

    println!("\n3. Winning Composite SI Infrastructure Signature:");
    println!("   Winner    : Plan {}", report.winning_composite_id);
    println!("   Signature : {}", report.composite_tournament_signature);

    println!("\n4. Constitutional Gatekeeper Interception & CorrectionBuffer Failover:");
    println!(
        "   - Intercepted Candidate : {}",
        report.fallback_recovery_demo.anomalous_candidate_id
    );
    println!(
        "   - Blocked at Gate Stage : {}",
        report.fallback_recovery_demo.failed_stage.as_str()
    );
    println!(
        "   - Rejection Reason      : {}",
        report.fallback_recovery_demo.failure_reason
    );
    println!(
        "   - Instant Fallback Plan : Plan {} ({})",
        report.fallback_recovery_demo.recovered_plan_id,
        report.fallback_recovery_demo.recovered_signature
    );
}

use si_gatekeeper::run_adversarial_verification_suite;

fn main() {
    println!("================================================================================");
    println!("  SI-GATEKEEPER — ADVERSARIAL VERIFICATION SUITE (PRIORITIES A, B, & C)");
    println!("  Testing Constitutional Integrity, Overflow/Score Gaming, & Legal Evidence Chains");
    println!("================================================================================\n");

    let report = run_adversarial_verification_suite()
        .expect("adversarial verification suite must execute cleanly");

    for (idx, v) in report.vectors.iter().enumerate() {
        let status = if v.defense_verified {
            "DEFENSE VERIFIED"
        } else {
            "FAILED"
        };
        println!(
            "[Vector {}] {} ({})",
            idx + 1,
            v.vector_id,
            v.category
        );
        println!("  Attack  : {}", v.attack_description);
        println!("  Status  : [{}]", status);
        println!("  Proof   : {}\n", v.canonical_proof_tag);
    }

    println!("--------------------------------------------------------------------------------");
    println!("Adversarial Suite Summary Signature:");
    println!("  {}", report.suite_signature);
    println!("================================================================================");
}

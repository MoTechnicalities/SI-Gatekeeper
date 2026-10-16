use si_gatekeeper::{
    canonical_statutory_classification_cases, evaluate_legal_argument, LegalDisposition,
};

fn main() {
    println!("================================================================================");
    println!("  SI-GATEKEEPER — 6-LAYER CONSTITUTIONAL & STATUTORY LEGAL REASONING TRACE");
    println!("  Open-sourced by the people for the people of the world.");
    println!("================================================================================\n");

    println!("Layers: Authority -> Enactment -> Definitions -> Interpretation -> Admissibility -> DecisionTrace");
    println!("Boundary: Separates Textual Observation from Legal Conclusion; never usurps judicial/legislative authority.");
    println!("Disclaimer: Illustrative software engineering demonstration only — not legal or tax advice.\n");

    let cases = canonical_statutory_classification_cases()
        .expect("canonical statutory classification cases must construct cleanly");

    for case in &cases {
        let trace = evaluate_legal_argument(case).expect("legal argument evaluation must succeed");

        println!("--------------------------------------------------------------------------------");
        println!(
            "CASE: {} | Classification: {}",
            trace.argument_id,
            trace.classification.as_str()
        );
        println!("Claim: {}", trace.claim);
        println!("\n  [Layer 3 — Verified Textual Observations]:");
        for obs in &trace.textual_observations {
            println!("    * {}", obs);
        }

        println!("\n  [Layers 4 & 5 — Competing Interpretations & Admissibility Gate]:");
        for (id, admissible, score, detail) in &trace.evaluated_interpretations {
            if *admissible {
                println!(
                    "    [ADMISSIBLE]   {} | DGCS Score: {:>2} | {}",
                    id,
                    score.unwrap_or(0),
                    detail
                );
            } else {
                println!("    [INADMISSIBLE] {} | Reason: {}", id, detail);
            }
        }

        println!("\n  [Layer 6 — Disposition & Non-Usurpation Boundary]:");
        match &trace.disposition {
            LegalDisposition::Supported {
                winning_interpretation_id,
                resonance_score,
            } => {
                println!(
                    "    Disposition : SUPPORTED (Winning Reading: {}, Score: {})",
                    winning_interpretation_id, resonance_score
                );
            }
            LegalDisposition::RequiresAuthoritativeInterpretation {
                candidate_interpretation_id,
                institutional_question,
            } => {
                println!(
                    "    Disposition : REQUIRES AUTHORITATIVE INTERPRETATION (Candidate: {})",
                    candidate_interpretation_id
                );
                println!("    Boundary    : {}", institutional_question);
            }
            LegalDisposition::Contradicted { rejected_reasons } => {
                println!("    Disposition : CONTRADICTED ({:?})", rejected_reasons);
            }
            LegalDisposition::Unresolved {
                admissible_interpretation_ids,
                open_questions,
            } => {
                println!(
                    "    Disposition : UNRESOLVED ({:?} | {:?})",
                    admissible_interpretation_ids, open_questions
                );
            }
        }

        println!("    Signature   : {}", trace.canonical_signature);
        println!(
            "    Trajectory  : {}\n",
            trace.trajectory.canonical_signature()
        );
    }
}

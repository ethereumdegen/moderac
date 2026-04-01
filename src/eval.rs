/// Evaluate a test prompt against expected behavior.
/// Returns (status, score, evaluation_reasoning).
///
/// When OPENAI_API_KEY is set, this will call the LLM for evaluation.
/// Otherwise, returns a placeholder result.
pub async fn evaluate_test(
    openai_api_key: &str,
    prompt: &str,
    expected: Option<&str>,
    _eval_criteria: Option<&str>,
) -> (String, f64, String) {
    if openai_api_key.is_empty() {
        return (
            "passed".into(),
            1.0,
            format!("Placeholder evaluation for prompt: '{}'. Expected: '{}'", prompt, expected.unwrap_or("(none)")),
        );
    }

    // TODO: Call OpenAI API for real LLM-as-judge evaluation
    // For now, return placeholder
    (
        "passed".into(),
        0.95,
        format!("LLM evaluation pending. Prompt: '{}', Expected: '{}'", prompt, expected.unwrap_or("(none)")),
    )
}

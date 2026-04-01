use crate::types::*;

pub struct Client {
    mode: Mode,
    http: reqwest::Client,
}

impl Client {
    pub fn new(mode: Mode) -> Self {
        Self {
            mode,
            http: reqwest::Client::new(),
        }
    }

    pub fn from_env() -> Self {
        Self::new(Mode::from_env())
    }

    fn remote_url(&self) -> Option<(&str, &str)> {
        match &self.mode {
            Mode::Remote { base_url, api_key } => Some((base_url.as_str(), api_key.as_str())),
            Mode::Local => None,
        }
    }

    pub async fn create_test(
        &self,
        name: &str,
        prompt: &str,
        expected: Option<&str>,
    ) -> Result<TestResponse, String> {
        let (base_url, api_key) = self.remote_url().ok_or("Not in remote mode")?;

        self.http
            .post(format!("{}/api/v1/tests", base_url))
            .bearer_auth(api_key)
            .json(&serde_json::json!({
                "name": name,
                "prompt": prompt,
                "expected": expected,
            }))
            .send()
            .await
            .map_err(|e| e.to_string())?
            .json()
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn run_tests(&self) -> Result<RunResponse, String> {
        let (base_url, api_key) = self.remote_url().ok_or("Not in remote mode")?;

        self.http
            .post(format!("{}/api/v1/runs", base_url))
            .bearer_auth(api_key)
            .send()
            .await
            .map_err(|e| e.to_string())?
            .json()
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn evaluate(
        &self,
        prompt: &str,
        expected: Option<&str>,
    ) -> Result<EvalResponse, String> {
        let (base_url, api_key) = self.remote_url().ok_or("Not in remote mode")?;

        self.http
            .post(format!("{}/api/v1/evaluate", base_url))
            .bearer_auth(api_key)
            .json(&serde_json::json!({
                "prompt": prompt,
                "expected": expected,
            }))
            .send()
            .await
            .map_err(|e| e.to_string())?
            .json()
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn sync_local_tests(&self, dir: Option<&std::path::Path>) -> Result<usize, String> {
        let suite = crate::local::load_suite(dir)?;
        let mut count = 0;

        for test in &suite.tests {
            let resolved = crate::local::resolve_prompt(test, &suite);
            self.create_test(&test.name, &resolved, test.expected.as_deref()).await?;
            count += 1;
        }

        Ok(count)
    }
}

/// Convenience builder for prompt-based tests (library usage).
pub struct PromptTestBuilder {
    prompt: String,
    expectations: Vec<String>,
}

pub fn prompt(text: &str) -> PromptTestBuilder {
    PromptTestBuilder {
        prompt: text.into(),
        expectations: Vec::new(),
    }
}

impl PromptTestBuilder {
    pub fn expect(mut self, expectation: &str) -> Self {
        self.expectations.push(expectation.into());
        self
    }

    pub async fn run(self) -> EvalResponse {
        let client = Client::from_env();
        let expected = self.expectations.join("; ");
        client
            .evaluate(&self.prompt, Some(&expected))
            .await
            .expect("Failed to evaluate prompt test")
    }
}

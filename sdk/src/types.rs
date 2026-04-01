use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A prompt-based test parsed from a .md file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptTest {
    pub name: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub skills: Vec<String>,
    /// The prompt body (markdown content below frontmatter)
    pub prompt: String,
    #[serde(default)]
    pub expected: Option<String>,
}

/// A reusable skill parsed from a .md file in the skills/ directory.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    /// The skill body (markdown content below frontmatter)
    pub body: String,
}

/// All loaded content from a moderac-tests directory.
#[derive(Debug, Clone, Default)]
pub struct TestSuite {
    pub tests: Vec<PromptTest>,
    pub skills: HashMap<String, Skill>,
}

/// Frontmatter parsed from a test .md file.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TestFrontmatter {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub skills: Vec<String>,
    #[serde(default)]
    pub expected: Option<String>,
}

/// Frontmatter parsed from a skill .md file.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SkillFrontmatter {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvalResponse {
    pub status: String,
    pub score: f64,
    pub evaluation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunResponse {
    pub run_id: String,
    pub status: String,
    pub tests_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResponse {
    pub id: String,
    pub name: String,
    pub prompt: String,
    pub status: String,
}

/// Mode the SDK operates in.
#[derive(Debug, Clone)]
pub enum Mode {
    /// Read/write tests from local `moderac-tests/` directory
    Local,
    /// Sync and run tests against moderac.com (or self-hosted)
    Remote { base_url: String, api_key: String },
}

impl Mode {
    pub fn from_env() -> Self {
        match (
            std::env::var("MODERAC_API_KEY").ok(),
            std::env::var("MODERAC_BASE_URL").ok(),
        ) {
            (Some(key), url) => Mode::Remote {
                base_url: url.unwrap_or_else(|| "https://moderac.com".into()),
                api_key: key,
            },
            _ => Mode::Local,
        }
    }
}

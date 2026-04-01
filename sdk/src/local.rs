use std::path::{Path, PathBuf};
use crate::types::*;

/// Default test directory
pub const DEFAULT_TEST_DIR: &str = "moderac-tests";
/// Skills subdirectory within the test directory
pub const SKILLS_DIR: &str = "skills";

/// Split a markdown file into YAML frontmatter and body content.
fn parse_frontmatter(content: &str) -> (Option<&str>, &str) {
    let trimmed = content.trim_start();
    if !trimmed.starts_with("---") {
        return (None, content);
    }
    // Find the closing ---
    if let Some(end) = trimmed[3..].find("\n---") {
        let fm = trimmed[3..3 + end].trim();
        let body = trimmed[3 + end + 4..].trim_start_matches('\n');
        (Some(fm), body)
    } else {
        (None, content)
    }
}

/// Discover all .md test files (excludes the skills/ subdirectory).
pub fn discover_tests(dir: Option<&Path>) -> Vec<PathBuf> {
    let dir = dir.unwrap_or_else(|| Path::new(DEFAULT_TEST_DIR));
    let mut files = Vec::new();

    if !dir.exists() {
        return files;
    }

    let pattern = format!("{}/**/*.md", dir.display());
    let skills_dir = dir.join(SKILLS_DIR);

    if let Ok(paths) = glob::glob(&pattern) {
        for entry in paths.flatten() {
            // Skip anything inside the skills/ directory
            if entry.starts_with(&skills_dir) {
                continue;
            }
            files.push(entry);
        }
    }

    files.sort();
    files
}

/// Discover all .md skill files in the skills/ subdirectory.
pub fn discover_skills(dir: Option<&Path>) -> Vec<PathBuf> {
    let dir = dir.unwrap_or_else(|| Path::new(DEFAULT_TEST_DIR));
    let skills_dir = dir.join(SKILLS_DIR);
    let mut files = Vec::new();

    if !skills_dir.exists() {
        return files;
    }

    let pattern = format!("{}/**/*.md", skills_dir.display());
    if let Ok(paths) = glob::glob(&pattern) {
        for entry in paths.flatten() {
            files.push(entry);
        }
    }

    files.sort();
    files
}

/// Load a test from a .md file.
pub fn load_test(path: &Path) -> Result<PromptTest, String> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| format!("Failed to read {}: {}", path.display(), e))?;

    let (fm_str, body) = parse_frontmatter(&content);

    let fm: TestFrontmatter = if let Some(fm_str) = fm_str {
        serde_yaml::from_str(fm_str)
            .map_err(|e| format!("Bad frontmatter in {}: {}", path.display(), e))?
    } else {
        TestFrontmatter::default()
    };

    // Derive name from frontmatter or filename
    let name = fm.name.unwrap_or_else(|| {
        path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unnamed")
            .to_string()
    });

    Ok(PromptTest {
        name,
        tags: fm.tags,
        skills: fm.skills,
        prompt: body.to_string(),
        expected: fm.expected,
    })
}

/// Load a skill from a .md file.
pub fn load_skill(path: &Path) -> Result<Skill, String> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| format!("Failed to read {}: {}", path.display(), e))?;

    let (fm_str, body) = parse_frontmatter(&content);

    let fm: SkillFrontmatter = if let Some(fm_str) = fm_str {
        serde_yaml::from_str(fm_str)
            .map_err(|e| format!("Bad frontmatter in {}: {}", path.display(), e))?
    } else {
        SkillFrontmatter::default()
    };

    let name = fm.name.unwrap_or_else(|| {
        path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unnamed")
            .to_string()
    });

    Ok(Skill {
        name,
        description: fm.description,
        body: body.to_string(),
    })
}

/// Load the full test suite: all tests + all skills.
pub fn load_suite(dir: Option<&Path>) -> Result<TestSuite, String> {
    let mut suite = TestSuite::default();

    for path in discover_skills(dir) {
        let skill = load_skill(&path)?;
        suite.skills.insert(skill.name.clone(), skill);
    }

    for path in discover_tests(dir) {
        let test = load_test(&path)?;
        suite.tests.push(test);
    }

    Ok(suite)
}

/// Resolve a test's prompt by inlining any referenced skills.
pub fn resolve_prompt(test: &PromptTest, suite: &TestSuite) -> String {
    if test.skills.is_empty() {
        return test.prompt.clone();
    }

    let mut skill_context = String::new();
    for skill_name in &test.skills {
        if let Some(skill) = suite.skills.get(skill_name) {
            skill_context.push_str(&format!("## Skill: {}\n\n{}\n\n", skill.name, skill.body));
        }
    }

    format!("{}{}", skill_context, test.prompt)
}

/// Create a new moderac-tests directory with example tests and skills.
pub fn init_test_dir(dir: &Path) -> Result<(), String> {
    let skills_dir = dir.join(SKILLS_DIR);
    std::fs::create_dir_all(&skills_dir)
        .map_err(|e| format!("Failed to create directory: {}", e))?;

    // Example skill
    let skill_content = r#"---
name: json-api
description: Skill for testing JSON REST APIs
---

You are testing a JSON REST API. All requests and responses use JSON.
When evaluating responses, check for:
- Correct HTTP status codes
- Valid JSON structure
- Required fields present in response body
- Appropriate error messages for failure cases
"#;

    std::fs::write(skills_dir.join("json-api.md"), skill_content)
        .map_err(|e| format!("Failed to write skill: {}", e))?;

    // Example test: user signup
    let test_signup = r#"---
name: user-signup
tags: [auth, signup]
skills: [json-api]
expected: Returns a success response with a user ID and sends a welcome email
---

Sign up a new user with email test@example.com and password "SecurePass123!".

The system should:
- Create the user account
- Return the new user's ID
- Send a welcome email to the provided address
"#;

    std::fs::write(dir.join("user-signup.md"), test_signup)
        .map_err(|e| format!("Failed to write test: {}", e))?;

    // Example test: duplicate email
    let test_duplicate = r#"---
name: duplicate-email
tags: [auth, error]
skills: [json-api]
expected: Returns an appropriate error message about duplicate email
---

Attempt to sign up with an email address that is already registered in the system.

The system should reject the request and return a clear error indicating the email is taken.
"#;

    std::fs::write(dir.join("duplicate-email.md"), test_duplicate)
        .map_err(|e| format!("Failed to write test: {}", e))?;

    Ok(())
}

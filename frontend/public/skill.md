# Moderac — Prompt-Based Testing for AI Agents

## What is Moderac?

Moderac is a testing platform that replaces code-based tests with **prompt-based tests**. Tests are written as markdown files containing natural language prompts. An independent LLM judge evaluates system responses against your criteria, producing pass/fail results with scores and reasoning.

This solves a fundamental problem: when AI agents write both code **and** tests, the same system is producing implementation and verification — circular validation. Moderac breaks this loop by evaluating at the prompt layer with a separate judge.

## Install

```bash
cargo install moderac
```

## Quick Start

```bash
moderac init                         # scaffold moderac-tests/ with examples
moderac list --json                  # discover tests and skills as JSON
moderac test --json                  # local: get all tests with resolved prompts
moderac test --remote --json         # remote: evaluate via LLM judge
moderac sync                         # push local tests to remote server
```

## Key Concepts

- **Tests** are `.md` files in `moderac-tests/`. Each file is one test.
- **Skills** are `.md` files in `moderac-tests/skills/`. Reusable context that gets prepended to test prompts.
- Tests reference skills by name in their frontmatter.
- Use `--json` on any command for machine-parseable output.
- Exit codes: `0` = success/all pass, `1` = test failures, `2` = error.

## Writing a Test

Create a `.md` file in `moderac-tests/`:

```markdown
---
name: user-signup
tags: [auth, signup]
skills: [json-api]
expected: Returns a 201 with user ID and sends welcome email
---

Sign up a new user with email test@example.com and password "SecurePass123!".

The system should:
- Create the user account
- Return the new user's ID in the response body
- Send a welcome email to the provided address
```

### Frontmatter fields

| Field      | Description                                              |
|------------|----------------------------------------------------------|
| `name`     | Test identifier. Defaults to filename without extension. |
| `tags`     | String list for filtering (`moderac test --tag auth`).   |
| `skills`   | Skill names to prepend as context before evaluation.     |
| `expected` | Short description of expected behavior for the judge.    |

## Writing a Skill

Create a `.md` file in `moderac-tests/skills/`:

```markdown
---
name: json-api
description: Skill for testing JSON REST APIs
---

You are testing a JSON REST API. All requests and responses use JSON.
When evaluating responses, check for:
- Correct HTTP status codes
- Valid JSON structure
- Required fields present in response body
- Appropriate error messages for failure cases
```

When a test references `skills: [json-api, auth-flow]`, the prompt becomes:

```
## Skill: json-api
<json-api body>

## Skill: auth-flow
<auth-flow body>

<test prompt body>
```

## Directory Structure

```
moderac-tests/
├── skills/              # Reusable skill definitions
│   ├── json-api.md
│   └── auth-flow.md
├── user-signup.md       # Test files
├── error-handling.md
└── checkout-flow.md
```

## Remote Evaluation

Set the `MODERAC_API_KEY` environment variable (starts with `mdr_`) to enable remote mode. Tests are sent to moderac.com for LLM-as-judge evaluation.

```bash
export MODERAC_API_KEY=mdr_your_key_here
moderac test --remote --json
```

### JSON output from `moderac test --remote --json`

```json
{
  "status": "passed",
  "passed": 5,
  "failed": 1,
  "total": 6,
  "results": [
    {
      "name": "user-signup",
      "status": "passed",
      "score": 0.95,
      "evaluation": "...",
      "tags": ["auth"],
      "skills": ["json-api"]
    }
  ]
}
```

## Rust Library Usage

```rust
use moderac::client::prompt;

#[tokio::test]
async fn test_signup() {
    let result = prompt("Sign up user with test@example.com")
        .expect("Returns 201 with user ID")
        .expect("Sends welcome email")
        .run()
        .await;

    assert_eq!(result.status, "passed");
    assert!(result.score > 0.8);
}
```

## Public API

All endpoints require `Authorization: Bearer mdr_...` header.

| Method | Endpoint             | Description                    |
|--------|----------------------|--------------------------------|
| POST   | `/api/v1/tests`      | Create a test                  |
| POST   | `/api/v1/runs`       | Trigger a test run             |
| GET    | `/api/v1/runs/{id}`  | Get run results                |
| POST   | `/api/v1/evaluate`   | Evaluate a single prompt       |

### Evaluate endpoint

```bash
curl -X POST https://moderac.com/api/v1/evaluate \
  -H "Authorization: Bearer mdr_..." \
  -H "Content-Type: application/json" \
  -d '{"prompt": "Sign up a user with test@example.com", "expected": "Returns 201 with user ID"}'
```

## Environment Variables

| Variable            | Description                                       |
|---------------------|---------------------------------------------------|
| `MODERAC_API_KEY`   | Enables remote mode (required for sync/evaluate)  |
| `MODERAC_BASE_URL`  | Override server URL (default: https://moderac.com) |

## CLI Reference

```
moderac init                        Create moderac-tests/ with examples
moderac list [--json]               List all tests and skills
moderac test [--json]               Local: display test definitions
moderac test --remote [--json]      Remote: evaluate via LLM judge
moderac test --tag <tag> [--json]   Filter by tag
moderac test --name <name> [--json] Run a single test by name
moderac sync                        Push local tests to remote
moderac show <file> [--json]        Display a parsed .md file
moderac agent-help                  Print detailed help for AI agents
```

### Global flags

- `--dir <path>` — Override test directory (default: `./moderac-tests`)
- `--json` — Machine-parseable JSON output on all commands

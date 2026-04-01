/// Returns the full help text designed for AI agents to understand and use moderac.
pub fn agent_help() -> &'static str {
r#"# Moderac — Prompt-Based Testing SDK

## What is Moderac?

Moderac replaces code-based tests with prompt-based tests. Tests are markdown
files containing natural language prompts. An independent LLM judge evaluates
system responses against your criteria, producing pass/fail results with scores
and reasoning.

This matters when AI agents write both code and tests. Same system producing
implementation and verification = circular validation. Moderac breaks this by
evaluating at the prompt layer with a separate judge.

## For AI Agents

You are likely reading this because you are an AI agent (Claude Code, Cursor,
Copilot, etc.) that needs to work with moderac tests. Here is what you need
to know:

### Quick start
```bash
moderac init                     # scaffold moderac-tests/ with examples
moderac list --json              # discover tests and skills as JSON
moderac test --json              # local: get all tests with resolved prompts
moderac test --agent --json      # agent: get codegen instructions, write real tests
moderac report --json            # collect pass/fail results from agent-written tests
moderac test --remote --json     # remote: evaluate and get structured results
```

### Key concepts
- **Tests** are `.md` files in `moderac-tests/`. Each file = one test.
- **Skills** are `.md` files in `moderac-tests/skills/`. Reusable context.
- Tests reference skills by name. Skills get prepended to the prompt.
- Use `--json` on any command for machine-parseable output.
- Exit codes: 0 = success/all pass, 1 = test failures, 2 = error.

### Creating a test
Write a `.md` file in `moderac-tests/`:

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

### Creating a skill
Write a `.md` file in `moderac-tests/skills/`:

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

### Agent mode (recommended for local testing)

`moderac test --agent --json` outputs codegen instructions for each test.
The agent should:
1. Read each test's `codegen_instruction` and `resolved_prompt`
2. Write real unit/integration tests that verify the described behavior
3. Run the tests
4. Write a JSON result to each test's `result_path`:
   `{"status": "passed", "reason": "test description"}` or
   `{"status": "failed", "reason": "what went wrong"}`
5. Run `moderac report --json` to collect all results

This is the primary local workflow — it turns prompt-based tests into real
executable tests verified by the coding agent.

```json
// moderac test --agent --json
{
  "status": "agent",
  "total": 6,
  "results_dir": "moderac-tests/.results",
  "instructions": "For each test, write and run real code tests...",
  "tests": [
    {
      "name": "user-signup",
      "expected": "Returns a 201 with user ID",
      "prompt": "<raw prompt body>",
      "resolved_prompt": "<with skills prepended>",
      "codegen_instruction": "Write and execute a real test that verifies...",
      "result_path": "moderac-tests/.results/user-signup.json"
    }
  ]
}
```

```json
// moderac report --json
{
  "status": "passed",
  "passed": 5,
  "failed": 1,
  "missing": 0,
  "total": 6,
  "results": [
    {"name": "user-signup", "status": "passed", "reason": "..."}
  ]
}
```

### Reading test results (--json output)

`moderac test --remote --json` returns:
```json
{
  "status": "passed",          // "passed", "failed", or "error"
  "passed": 5,
  "failed": 1,
  "total": 6,
  "results": [
    {
      "name": "user-signup",
      "status": "passed",      // "passed", "failed", or "error"
      "score": 0.95,           // 0.0 to 1.0
      "evaluation": "...",     // LLM judge reasoning
      "tags": ["auth"],
      "skills": ["json-api"]
    }
  ]
}
```

`moderac test --json` (local, no evaluation) returns:
```json
{
  "status": "local",
  "total": 6,
  "tests": [
    {
      "name": "user-signup",
      "tags": ["auth"],
      "skills": ["json-api"],
      "expected": "Returns a 201 with user ID",
      "prompt": "<raw prompt body>",
      "resolved_prompt": "<prompt with skills prepended>"
    }
  ]
}
```

## Directory Structure

```
moderac-tests/
├── skills/              # Reusable skill definitions (.md)
│   ├── json-api.md
│   └── auth-flow.md
├── user-signup.md       # Test files (.md)
├── error-handling.md
└── checkout-flow.md
```

## Test File Format (.md)

YAML frontmatter + markdown body.

### Frontmatter fields:
- `name` — Test identifier. Defaults to filename without extension.
- `tags` — String list for filtering (`moderac test --tag auth`).
- `skills` — Skill names to prepend as context before evaluation.
- `expected` — Short description of expected behavior for the LLM judge.

### Body:
Natural language prompt describing what to test. Write it as clear instructions
to a human tester: the action, the inputs, and what to observe.

## Skill File Format (.md)

Lives in `moderac-tests/skills/`. YAML frontmatter + markdown body.

### Frontmatter fields:
- `name` — Identifier that tests reference in their `skills` list.
- `description` — What this skill provides.

### Body:
Context and instructions prepended to any test referencing this skill.
Use for shared setup, personas, evaluation criteria, or domain knowledge.

### Skill composition:
`skills: [json-api, auth-flow]` produces:

```
## Skill: json-api
<json-api body>

## Skill: auth-flow
<auth-flow body>

<test prompt body>
```

## CLI Reference

```
moderac init                        Create moderac-tests/ with examples
moderac list [--json]               List all tests and skills
moderac test [--json]               Local: display/output test definitions
moderac test --agent [--json]       Agent: output codegen instructions for real tests
moderac test --remote [--json]      Remote: evaluate via LLM judge
moderac test --tag <tag> [--json]   Filter by tag
moderac test --name <name> [--json] Run a single test by name
moderac report [--json]             Collect results from agent-written tests
moderac sync                        Push local tests to remote project
moderac show <file> [--json]        Display a parsed .md file
moderac agent-help                  Print this help text (for AI agents)
```

### Global flags:
- `--dir <path>` — Override test directory (default: `./moderac-tests`)
- `--json` — Machine-parseable JSON output on all commands

### Exit codes:
- `0` — Success, or all tests passed
- `1` — One or more tests failed
- `2` — Error (bad config, no tests found, network failure)

## Modes

### Local (default)
No API key needed. `moderac test` reads and resolves tests locally.
Use `--json` to get the full resolved prompts for inspection.

### Remote
Set `MODERAC_API_KEY` (starts with `mdr_`). Tests are sent to the remote
server for LLM-as-judge evaluation.

Env vars:
- `MODERAC_API_KEY` — Enables remote mode.
- `MODERAC_BASE_URL` — Override server (default: https://moderac.com).

## Writing Good Tests

1. **One behavior per file.** `user-signup.md`, `rate-limit-exceeded.md`.
2. **Be specific in `expected`.** "Returns 201 with user ID" > "Works correctly".
3. **Use skills for shared context.** API conventions, auth setup, domain rules.
4. **Use tags for subsets.** `--tag smoke` for quick, `--tag regression` for full.
5. **Write prompts as instructions.** Action, inputs, what to observe.

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

Requires `MODERAC_API_KEY` since evaluation needs the remote judge.
"#
}

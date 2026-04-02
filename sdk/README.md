# moderac

**Prompt-based testing for the AI agent era.**

Replace brittle code-based tests with natural language prompts evaluated by an LLM judge. Write tests as markdown files, compose reusable skills, and run them locally or against a remote evaluation server.

## Install

```sh
cargo install moderac
```

## Quick start

```sh
# Scaffold example tests
moderac init

# List discovered tests and skills
moderac list

# View resolved test prompts locally
moderac test

# Evaluate against remote LLM judge
export MODERAC_API_KEY=mdr_...
moderac test --remote

# Generate codegen instructions for AI agents
moderac test --agent
```

## How it works

Tests live in a `moderac-tests/` directory as markdown files with YAML frontmatter:

```markdown
---
name: user-signup
tags: [auth, signup]
skills: [json-api]
expected: Returns a success response with a user ID
---

Sign up a new user with email test@example.com and password "SecurePass123!".

The system should:
- Create the user account
- Return the new user's ID
- Send a welcome email
```

**Skills** are reusable context snippets in `moderac-tests/skills/` that get prepended to test prompts. Reference them by name in a test's `skills` field.

## Three modes

| Mode | Flag | What it does |
|------|------|-------------|
| **Local** | *(default)* | Parse and display tests for inspection |
| **Remote** | `--remote` | Send prompts to an LLM judge for pass/fail scoring |
| **Agent** | `--agent` | Emit structured codegen instructions for AI agents to write and execute real tests |

## Library usage

Use `moderac` as a library to load tests programmatically or run remote evaluations:

```rust
use moderac::{Client, Mode, load_suite, discover_tests, load_test};

// Load and inspect tests locally
let suite = load_suite(None)?;
for test in &suite.tests {
    println!("{}: {}", test.name, test.prompt);
}

// Evaluate a prompt against the remote judge
let client = Client::from_env(); // reads MODERAC_API_KEY
let result = client.evaluate("Test the login flow", Some("should return 200")).await?;
println!("{}: {:.0}%", result.status, result.score * 100.0);
```

Fluent builder for quick evaluations:

```rust
use moderac::client::prompt;

let result = prompt("Sign up a new user")
    .expect("Returns a user ID")
    .expect("Sends a welcome email")
    .run()
    .await;

assert_eq!(result.status, "passed");
```

## CLI reference

```
moderac init                           Scaffold moderac-tests/ with examples
moderac list [--json]                  List all tests and skills
moderac test [--json]                  Display resolved test prompts
moderac test --remote [--json]         Evaluate via LLM judge
moderac test --agent [--json]          Codegen instructions for AI agents
moderac test --tag <tag>               Filter tests by tag
moderac test --name <name>             Run a single test by name
moderac report [--json]               Collect agent-written results
moderac sync                           Push local tests to remote server
moderac show <file> [--json]           Display a parsed .md file
moderac agent-help                     Detailed help for AI agents
```

**Global flags:** `--dir <path>` (override test directory), `--json` (machine-readable output)

**Exit codes:** `0` success, `1` test failures, `2` error

## Environment variables

| Variable | Description |
|----------|-------------|
| `MODERAC_API_KEY` | API key for remote evaluation and sync |
| `MODERAC_BASE_URL` | Server URL (default: `https://moderac.com`) |

## License

MIT

import { Link } from 'react-router'
import SiteHeader from '../components/SiteHeader'
import SiteFooter from '../components/SiteFooter'

export default function Landing() {
  return (
    <div className="min-h-screen flex flex-col">
      <SiteHeader />

      {/* Hero */}
      <section className="px-6 pt-24 pb-20 text-center max-w-4xl mx-auto">
        <div className="inline-block mb-6 px-3 py-1 rounded-full border border-border text-xs text-text-muted tracking-wide uppercase">
          Prompt-based testing platform
        </div>
        <h1 className="text-5xl md:text-6xl font-bold tracking-tight leading-[1.1] mb-6">
          Tests for the{' '}
          <span className="bg-gradient-to-r from-accent to-purple-400 bg-clip-text text-transparent">
            AI agent era
          </span>
        </h1>
        <p className="text-lg text-text-muted max-w-2xl mx-auto mb-10 leading-relaxed">
          Define tests as natural language prompts. Evaluate system behavior with LLM judges.
          Stop writing tests that the same AI can game.
        </p>
        <div className="flex gap-4 justify-center">
          <Link
            to="/signin"
            className="px-6 py-3 bg-accent hover:bg-accent-hover text-white rounded-lg font-medium transition-colors"
          >
            Get started
          </Link>
          <a
            href="#how-it-works"
            className="px-6 py-3 border border-border hover:border-border-hover text-text-muted hover:text-text rounded-lg font-medium transition-colors"
          >
            Learn more
          </a>
        </div>
      </section>

      {/* The Problem */}
      <section className="px-6 py-20 border-t border-border">
        <div className="max-w-3xl mx-auto">
          <h2 className="text-3xl font-bold mb-6">Code-based tests are broken</h2>
          <div className="space-y-4 text-text-muted leading-relaxed">
            <p>
              AI agents write code. They also write the tests. When the same system produces both
              the implementation and its verification, you get circular validation — tests that
              always pass because they were designed to match the code, not to catch real failures.
            </p>
            <p>
              Traditional testing frameworks operate at the code level, but AI development happens
              at the prompt level. There's an abstraction mismatch. Your tests need to work at the
              same layer where your development actually happens.
            </p>
          </div>
        </div>
      </section>

      {/* The Solution */}
      <section className="px-6 py-20 border-t border-border">
        <div className="max-w-3xl mx-auto">
          <h2 className="text-3xl font-bold mb-6">Tests defined as prompts</h2>
          <div className="grid md:grid-cols-3 gap-8 mt-10">
            <div className="p-6 rounded-xl bg-bg-card border border-border">
              <div className="w-10 h-10 rounded-lg bg-accent/10 flex items-center justify-center mb-4 text-accent font-mono font-bold">1</div>
              <h3 className="font-semibold mb-2">Human-readable</h3>
              <p className="text-sm text-text-muted">Tests are natural language descriptions of expected behavior. Anyone can read and write them.</p>
            </div>
            <div className="p-6 rounded-xl bg-bg-card border border-border">
              <div className="w-10 h-10 rounded-lg bg-accent/10 flex items-center justify-center mb-4 text-accent font-mono font-bold">2</div>
              <h3 className="font-semibold mb-2">AI-resistant</h3>
              <p className="text-sm text-text-muted">A separate LLM judge evaluates results. The agent that wrote the code can't game the evaluation.</p>
            </div>
            <div className="p-6 rounded-xl bg-bg-card border border-border">
              <div className="w-10 h-10 rounded-lg bg-accent/10 flex items-center justify-center mb-4 text-accent font-mono font-bold">3</div>
              <h3 className="font-semibold mb-2">Same abstraction</h3>
              <p className="text-sm text-text-muted">Tests operate at the prompt level — the same layer where AI development happens.</p>
            </div>
          </div>
        </div>
      </section>

      {/* How It Works */}
      <section id="how-it-works" className="px-6 py-20 border-t border-border">
        <div className="max-w-3xl mx-auto">
          <h2 className="text-3xl font-bold mb-10">How it works</h2>
          <div className="space-y-8">
            <div className="flex gap-6">
              <div className="shrink-0 w-8 h-8 rounded-full bg-accent/20 flex items-center justify-center text-accent text-sm font-bold">1</div>
              <div>
                <h3 className="font-semibold mb-1">Define prompts</h3>
                <p className="text-text-muted text-sm">Write test cases as natural language prompts describing what your system should do and what you expect to happen.</p>
              </div>
            </div>
            <div className="flex gap-6">
              <div className="shrink-0 w-8 h-8 rounded-full bg-accent/20 flex items-center justify-center text-accent text-sm font-bold">2</div>
              <div>
                <h3 className="font-semibold mb-1">Run against your system</h3>
                <p className="text-text-muted text-sm">Moderac sends the prompts to your system and captures the responses. Run manually or via CI with the SDK.</p>
              </div>
            </div>
            <div className="flex gap-6">
              <div className="shrink-0 w-8 h-8 rounded-full bg-accent/20 flex items-center justify-center text-accent text-sm font-bold">3</div>
              <div>
                <h3 className="font-semibold mb-1">Get evaluation results</h3>
                <p className="text-text-muted text-sm">An independent LLM judge evaluates each response against your criteria. Get pass/fail, scores, and reasoning.</p>
              </div>
            </div>
          </div>
        </div>
      </section>

      {/* Install */}
      <section className="px-6 py-20 border-t border-border">
        <div className="max-w-3xl mx-auto text-center">
          <h2 className="text-3xl font-bold mb-4">Get started in seconds</h2>
          <p className="text-text-muted mb-8 max-w-xl mx-auto">
            Install the CLI, scaffold your test directory, and start writing prompt-based tests.
            No config files. No boilerplate. Just markdown.
          </p>
          <div className="bg-bg-card border border-border rounded-xl p-5 inline-block mb-8">
            <code className="font-mono text-lg text-accent">cargo install moderac</code>
          </div>
          <div className="grid md:grid-cols-3 gap-6 text-left mt-6">
            <div className="p-5 bg-bg-card border border-border rounded-xl">
              <div className="font-mono text-sm text-accent mb-3">$ moderac init</div>
              <p className="text-sm text-text-muted">
                Scaffolds a <code className="text-text font-mono text-xs">moderac-tests/</code> directory
                with example tests and a <code className="text-text font-mono text-xs">skills/</code> folder
                for reusable context.
              </p>
            </div>
            <div className="p-5 bg-bg-card border border-border rounded-xl">
              <div className="font-mono text-sm text-accent mb-3">$ moderac test</div>
              <p className="text-sm text-text-muted">
                Discovers all <code className="text-text font-mono text-xs">.md</code> test files,
                resolves skill references, and runs them locally or against the remote LLM judge.
              </p>
            </div>
            <div className="p-5 bg-bg-card border border-border rounded-xl">
              <div className="font-mono text-sm text-accent mb-3">$ moderac help</div>
              <p className="text-sm text-text-muted">
                Prints a full reference designed for AI agents — so Claude Code, Cursor, or any
                coding agent can read it and use moderac autonomously.
              </p>
            </div>
          </div>
        </div>
      </section>

      {/* Test File Format */}
      <section className="px-6 py-20 border-t border-border">
        <div className="max-w-3xl mx-auto">
          <h2 className="text-3xl font-bold mb-4">Tests are just markdown</h2>
          <p className="text-text-muted mb-8">
            Each test is a <code className="text-text font-mono text-sm">.md</code> file with
            YAML frontmatter and a natural language prompt. Skills are reusable markdown files
            that get composed into your tests automatically.
          </p>
          <div className="grid md:grid-cols-2 gap-6">
            <div>
              <div className="text-xs text-text-muted mb-2 font-mono">moderac-tests/user-signup.md</div>
              <div className="bg-bg-card border border-border rounded-xl p-5 overflow-x-auto">
                <pre className="font-mono text-sm leading-relaxed text-text-muted">{`---
name: user-signup
tags: [auth, signup]
skills: [json-api]
expected: Returns 201 with user ID
---

Sign up a new user with email
test@example.com.

The system should:
- Create the user account
- Return the new user's ID
- Send a welcome email`}</pre>
              </div>
            </div>
            <div>
              <div className="text-xs text-text-muted mb-2 font-mono">moderac-tests/skills/json-api.md</div>
              <div className="bg-bg-card border border-border rounded-xl p-5 overflow-x-auto">
                <pre className="font-mono text-sm leading-relaxed text-text-muted">{`---
name: json-api
description: JSON REST API testing
---

You are testing a JSON REST API.
When evaluating responses, check:
- Correct HTTP status codes
- Valid JSON structure
- Required fields present
- Appropriate error messages`}</pre>
              </div>
            </div>
          </div>
        </div>
      </section>

      {/* Agent-native */}
      <section className="px-6 py-20 border-t border-border">
        <div className="max-w-3xl mx-auto">
          <h2 className="text-3xl font-bold mb-4">Built for AI agents</h2>
          <p className="text-text-muted mb-8">
            Every command supports <code className="text-text font-mono text-sm">--json</code> for
            structured output. AI coding agents can discover tests, run them, and parse results
            without scraping terminal output.
          </p>
          <div className="bg-bg-card border border-border rounded-xl p-5 overflow-x-auto">
            <pre className="font-mono text-sm leading-relaxed">
              <code className="text-text-muted">{`$ moderac test --remote --json
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
      "evaluation": "Response correctly returns 201..."
    }
  ]
}`}</code>
            </pre>
          </div>
        </div>
      </section>

      {/* Rust library */}
      <section className="px-6 py-20 border-t border-border">
        <div className="max-w-3xl mx-auto">
          <h2 className="text-3xl font-bold mb-4">Or use the Rust library directly</h2>
          <p className="text-text-muted mb-8">
            Add <code className="text-text font-mono text-sm">moderac</code> as a dev dependency
            and write prompt-based tests alongside your regular test suite.
          </p>
          <div className="bg-bg-card border border-border rounded-xl p-5 overflow-x-auto">
            <pre className="font-mono text-sm leading-relaxed">
              <code className="text-text-muted">{`use moderac::client::prompt;

#[tokio::test]
async fn test_user_signup() {
    let result = prompt("Sign up a new user with email test@example.com")
        .expect("Returns a success response with user ID")
        .expect("Sends a welcome email")
        .run()
        .await;

    assert_eq!(result.status, "passed");
    assert!(result.score > 0.8);
}`}</code>
            </pre>
          </div>
        </div>
      </section>

      {/* CTA */}
      <section className="px-6 py-24 border-t border-border text-center">
        <div className="max-w-2xl mx-auto">
          <h2 className="text-3xl font-bold mb-4">Start testing with prompts</h2>
          <p className="text-text-muted mb-8">
            Open source CLI. Free to use locally. Connect to moderac.com for LLM-judged evaluation.
          </p>
          <div className="flex flex-col sm:flex-row gap-4 justify-center items-center">
            <div className="bg-bg-card border border-border rounded-lg px-5 py-3">
              <code className="font-mono text-accent">cargo install moderac</code>
            </div>
            <Link
              to="/signin"
              className="px-8 py-3 bg-accent hover:bg-accent-hover text-white rounded-lg font-medium transition-colors"
            >
              Sign up for remote evaluation
            </Link>
          </div>
        </div>
      </section>

      <SiteFooter />
    </div>
  )
}

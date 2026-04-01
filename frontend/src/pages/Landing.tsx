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

      {/* Code Example */}
      <section className="px-6 py-20 border-t border-border">
        <div className="max-w-3xl mx-auto">
          <h2 className="text-3xl font-bold mb-6">Integrate with your test suite</h2>
          <p className="text-text-muted mb-8">Use the Rust SDK to define prompt-based tests alongside your regular tests.</p>
          <div className="bg-bg-card border border-border rounded-xl p-6 overflow-x-auto">
            <pre className="font-mono text-sm leading-relaxed">
              <code className="text-text-muted">{`use moderac::prompt;

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
          <p className="text-text-muted mb-8">Free to get started. Define your first prompt-based test in minutes.</p>
          <Link
            to="/signin"
            className="inline-block px-8 py-3 bg-accent hover:bg-accent-hover text-white rounded-lg font-medium transition-colors"
          >
            Get started free
          </Link>
        </div>
      </section>

      <SiteFooter />
    </div>
  )
}

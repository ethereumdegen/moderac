use clap::{Parser, Subcommand};
use colored::Colorize;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "moderac", about = "Prompt-based testing for the AI agent era")]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Test directory (default: ./moderac-tests)
    #[arg(long, global = true)]
    dir: Option<PathBuf>,

    /// Output as JSON (for machine/agent consumption)
    #[arg(long, global = true)]
    json: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize moderac-tests/ with example tests and skills
    Init,

    /// List all discovered tests and skills
    List,

    /// Run prompt-based tests and return structured results
    Test {
        /// Evaluate against remote server (requires MODERAC_API_KEY)
        #[arg(long)]
        remote: bool,

        /// Filter tests by tag
        #[arg(long)]
        tag: Option<String>,

        /// Run only a specific test by name
        #[arg(long)]
        name: Option<String>,
    },

    /// Sync local tests to remote server
    Sync,

    /// Show a parsed test or skill file
    Show {
        /// Path to the .md file
        file: PathBuf,
    },

    /// Print detailed help explaining moderac for AI agents
    Help,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let dir = cli.dir.unwrap_or_else(|| PathBuf::from(moderac::local::DEFAULT_TEST_DIR));
    let json = cli.json;

    let code = match cli.command {
        Commands::Init => cmd_init(&dir, json),
        Commands::List => cmd_list(&dir, json),
        Commands::Test { remote, tag, name } => cmd_test(&dir, json, remote, tag, name).await,
        Commands::Sync => cmd_sync(&dir, json).await,
        Commands::Show { file } => cmd_show(&file, json),
        Commands::Help => { println!("{}", moderac::help::agent_help()); 0 }
    };

    std::process::exit(code);
}

fn cmd_init(dir: &PathBuf, json: bool) -> i32 {
    match moderac::local::init_test_dir(dir) {
        Ok(()) => {
            if json {
                println!("{}", serde_json::json!({
                    "status": "ok",
                    "dir": dir.display().to_string(),
                    "files": ["skills/json-api.md", "user-signup.md", "duplicate-email.md"]
                }));
            } else {
                println!("{} Initialized {}/", "✓".green(), dir.display());
                println!("  Created skills/json-api.md");
                println!("  Created user-signup.md");
                println!("  Created duplicate-email.md");
                println!("\n  Edit your tests, then run: {}", "moderac test".bold());
            }
            0
        }
        Err(e) => {
            if json {
                println!("{}", serde_json::json!({ "status": "error", "error": e }));
            } else {
                eprintln!("{} {}", "error:".red().bold(), e);
            }
            2
        }
    }
}

fn cmd_list(dir: &PathBuf, json: bool) -> i32 {
    let skill_files = moderac::discover_skills(Some(dir));
    let test_files = moderac::discover_tests(Some(dir));

    if json {
        let skills: Vec<serde_json::Value> = skill_files.iter().filter_map(|f| {
            moderac::load_skill(f).ok().map(|s| serde_json::json!({
                "name": s.name,
                "description": s.description,
                "file": f.display().to_string(),
            }))
        }).collect();

        let tests: Vec<serde_json::Value> = test_files.iter().filter_map(|f| {
            moderac::load_test(f).ok().map(|t| serde_json::json!({
                "name": t.name,
                "tags": t.tags,
                "skills": t.skills,
                "expected": t.expected,
                "file": f.display().to_string(),
            }))
        }).collect();

        println!("{}", serde_json::json!({
            "skills": skills,
            "tests": tests,
        }));
        return 0;
    }

    if skill_files.is_empty() && test_files.is_empty() {
        println!("{}", "No test files found.".dimmed());
        println!("Run {} to create example tests.", "moderac init".bold());
        return 0;
    }

    if !skill_files.is_empty() {
        println!("{}", "Skills".bold().underline());
        for file in &skill_files {
            if let Ok(skill) = moderac::load_skill(file) {
                let desc = skill.description.as_deref().unwrap_or("");
                println!("  {} {}", skill.name.bold(), desc.dimmed());
            }
        }
        println!();
    }

    println!("{}", "Tests".bold().underline());
    let mut total = 0;
    for file in &test_files {
        if let Ok(test) = moderac::load_test(file) {
            let mut meta = Vec::new();
            if !test.tags.is_empty() {
                meta.push(format!("tags: {}", test.tags.join(", ")));
            }
            if !test.skills.is_empty() {
                meta.push(format!("skills: {}", test.skills.join(", ")));
            }
            let suffix = if meta.is_empty() {
                String::new()
            } else {
                format!(" [{}]", meta.join(" | ").dimmed())
            };
            println!("  {} {}{}", "•".dimmed(), test.name, suffix);
            total += 1;
        }
    }
    println!("\n{} tests, {} skills",
        total.to_string().bold(),
        skill_files.len().to_string().bold());
    0
}

async fn cmd_test(dir: &PathBuf, json: bool, remote: bool, tag: Option<String>, name: Option<String>) -> i32 {
    let suite = match moderac::load_suite(Some(dir)) {
        Ok(s) => s,
        Err(e) => {
            if json {
                println!("{}", serde_json::json!({ "status": "error", "error": e }));
            } else {
                eprintln!("{} {}", "error:".red().bold(), e);
            }
            return 2;
        }
    };

    if suite.tests.is_empty() {
        if json {
            println!("{}", serde_json::json!({
                "status": "error",
                "error": "No test files found. Run `moderac init` first."
            }));
        } else {
            eprintln!("{}", "No test files found. Run `moderac init` first.".red());
        }
        return 2;
    }

    let tests: Vec<_> = suite.tests.iter().filter(|t| {
        if let Some(ref n) = name {
            return &t.name == n;
        }
        if let Some(ref tag) = tag {
            return t.tags.contains(tag);
        }
        true
    }).collect();

    if tests.is_empty() {
        if json {
            println!("{}", serde_json::json!({
                "status": "error",
                "error": "No tests matched the given filters."
            }));
        } else {
            eprintln!("{}", "No tests matched the given filters.".red());
        }
        return 2;
    }

    if !json {
        println!("{} Running {} tests...\n", "▶".blue(), tests.len());
    }

    if remote {
        let client = moderac::Client::from_env();
        let mut results = Vec::new();
        let mut passed = 0usize;
        let mut failed = 0usize;

        for test in &tests {
            let resolved = moderac::local::resolve_prompt(test, &suite);
            if !json {
                print!("  {} {} ... ", "•".dimmed(), test.name);
            }
            match client.evaluate(&resolved, test.expected.as_deref()).await {
                Ok(result) => {
                    let is_pass = result.status == "passed";
                    if is_pass { passed += 1; } else { failed += 1; }

                    if json {
                        results.push(serde_json::json!({
                            "name": test.name,
                            "status": result.status,
                            "score": result.score,
                            "evaluation": result.evaluation,
                            "tags": test.tags,
                            "skills": test.skills,
                        }));
                    } else if is_pass {
                        println!("{} (score: {:.0}%)", "PASS".green().bold(), result.score * 100.0);
                    } else {
                        println!("{} (score: {:.0}%)", "FAIL".red().bold(), result.score * 100.0);
                        println!("    {}", result.evaluation.dimmed());
                    }
                }
                Err(e) => {
                    failed += 1;
                    if json {
                        results.push(serde_json::json!({
                            "name": test.name,
                            "status": "error",
                            "error": e,
                        }));
                    } else {
                        println!("{} {}", "ERROR".red().bold(), e);
                    }
                }
            }
        }

        if json {
            println!("{}", serde_json::json!({
                "status": if failed == 0 { "passed" } else { "failed" },
                "passed": passed,
                "failed": failed,
                "total": tests.len(),
                "results": results,
            }));
        } else {
            println!("\n{} passed, {} failed",
                passed.to_string().green(),
                failed.to_string().red());
        }

        if failed > 0 { 1 } else { 0 }
    } else {
        // Local mode: output resolved tests for inspection/agent use
        let mut test_data = Vec::new();

        for test in &tests {
            let resolved = moderac::local::resolve_prompt(test, &suite);

            if json {
                test_data.push(serde_json::json!({
                    "name": test.name,
                    "tags": test.tags,
                    "skills": test.skills,
                    "expected": test.expected,
                    "prompt": test.prompt,
                    "resolved_prompt": resolved,
                }));
            } else {
                println!("  {} {}", "•".dimmed(), test.name.bold());
                if !test.skills.is_empty() {
                    println!("    Skills:   {}", test.skills.join(", ").dimmed());
                }
                let lines: Vec<&str> = test.prompt.lines().take(3).collect();
                for line in &lines {
                    println!("    {}", line);
                }
                if test.prompt.lines().count() > 3 {
                    println!("    {}", "...".dimmed());
                }
                if let Some(ref exp) = test.expected {
                    println!("    Expected: {}", exp);
                }
                println!();
            }
        }

        if json {
            println!("{}", serde_json::json!({
                "status": "local",
                "total": tests.len(),
                "tests": test_data,
            }));
        } else {
            println!("{}", "Local mode: tests listed but not evaluated.".dimmed());
            println!("Use {} to evaluate, or {} for JSON output.",
                "--remote".bold(), "--json".bold());
        }
        0
    }
}

async fn cmd_sync(dir: &PathBuf, json: bool) -> i32 {
    let client = moderac::Client::from_env();
    match client.sync_local_tests(Some(dir)).await {
        Ok(count) => {
            if json {
                println!("{}", serde_json::json!({ "status": "ok", "synced": count }));
            } else {
                println!("{} Synced {} tests to remote", "✓".green(), count);
            }
            0
        }
        Err(e) => {
            if json {
                println!("{}", serde_json::json!({ "status": "error", "error": e }));
            } else {
                eprintln!("{} {}", "error:".red().bold(), e);
            }
            2
        }
    }
}

fn cmd_show(file: &PathBuf, json: bool) -> i32 {
    let is_skill = file.components().any(|c| c.as_os_str() == "skills");

    if is_skill {
        match moderac::load_skill(file) {
            Ok(skill) => {
                if json {
                    println!("{}", serde_json::json!({
                        "type": "skill",
                        "name": skill.name,
                        "description": skill.description,
                        "body": skill.body,
                    }));
                } else {
                    println!("{} {}", "Skill:".bold(), skill.name.bold());
                    if let Some(desc) = &skill.description {
                        println!("{}", desc.dimmed());
                    }
                    println!("\n{}", skill.body);
                }
                0
            }
            Err(e) => {
                if json {
                    println!("{}", serde_json::json!({ "status": "error", "error": e }));
                } else {
                    eprintln!("{} {}", "error:".red().bold(), e);
                }
                2
            }
        }
    } else {
        match moderac::load_test(file) {
            Ok(test) => {
                if json {
                    println!("{}", serde_json::json!({
                        "type": "test",
                        "name": test.name,
                        "tags": test.tags,
                        "skills": test.skills,
                        "expected": test.expected,
                        "prompt": test.prompt,
                    }));
                } else {
                    println!("{} {}", "Test:".bold(), test.name.bold());
                    if !test.tags.is_empty() {
                        println!("Tags:     {}", test.tags.join(", "));
                    }
                    if !test.skills.is_empty() {
                        println!("Skills:   {}", test.skills.join(", "));
                    }
                    if let Some(ref exp) = test.expected {
                        println!("Expected: {}", exp);
                    }
                    println!("\n{}", test.prompt);
                }
                0
            }
            Err(e) => {
                if json {
                    println!("{}", serde_json::json!({ "status": "error", "error": e }));
                } else {
                    eprintln!("{} {}", "error:".red().bold(), e);
                }
                2
            }
        }
    }
}

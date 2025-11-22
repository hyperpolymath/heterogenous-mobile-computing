//! Mobile AI Orchestrator - Command Line Interface
//!
//! A simple CLI demonstrating the Phase 1 MVP functionality.
//!
//! # Usage
//!
//! ```bash
//! mobile-ai "Your query here"
//! mobile-ai --project oblibeny "Explain type system"
//! mobile-ai --interactive
//! ```

use mobile_ai_orchestrator::{Orchestrator, Query};
use std::env;
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = env::args().collect();

    // Parse command line arguments
    let config = parse_args(&args);

    match config.mode {
        Mode::Interactive => run_interactive(),
        Mode::SingleQuery { query, project } => run_single_query(&query, project.as_deref()),
        Mode::Help => print_help(),
        Mode::Version => print_version(),
    }
}

#[derive(Debug)]
enum Mode {
    Interactive,
    SingleQuery {
        query: String,
        project: Option<String>,
    },
    Help,
    Version,
}

#[derive(Debug)]
struct Config {
    mode: Mode,
}

fn parse_args(args: &[String]) -> Config {
    if args.len() == 1 {
        return Config {
            mode: Mode::Interactive,
        };
    }

    match args[1].as_str() {
        "--help" | "-h" => Config { mode: Mode::Help },
        "--version" | "-v" => Config {
            mode: Mode::Version,
        },
        "--interactive" | "-i" => Config {
            mode: Mode::Interactive,
        },
        "--project" | "-p" => {
            if args.len() < 4 {
                eprintln!("Error: --project requires a project name and query");
                std::process::exit(1);
            }
            Config {
                mode: Mode::SingleQuery {
                    query: args[3..].join(" "),
                    project: Some(args[2].clone()),
                },
            }
        }
        _ => Config {
            mode: Mode::SingleQuery {
                query: args[1..].join(" "),
                project: None,
            },
        },
    }
}

fn run_interactive() {
    println!("Mobile AI Orchestrator - Interactive Mode");
    println!("RSR Compliance: {}", mobile_ai_orchestrator::RSR_COMPLIANCE);
    println!("Version: {}", mobile_ai_orchestrator::VERSION);
    println!("\nCommands:");
    println!("  /project <name> - Switch project context");
    println!("  /clear          - Clear conversation history");
    println!("  /history        - Show recent history");
    println!("  /quit           - Exit");
    println!();

    let mut orchestrator = Orchestrator::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("Error reading input");
            continue;
        }

        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        // Handle commands
        if input.starts_with('/') {
            handle_command(&mut orchestrator, input);
            continue;
        }

        // Process as query
        let query = Query::new(input);
        match orchestrator.process(query) {
            Ok(response) => {
                println!("\n{}", response.text);
                println!(
                    "\n[Route: {:?}, Confidence: {:.2}, Latency: {}ms]",
                    response.route, response.confidence, response.latency_ms
                );
                println!();
            }
            Err(err) => {
                eprintln!("Error: {}", err);
                println!();
            }
        }
    }
}

fn handle_command(orchestrator: &mut Orchestrator, cmd: &str) {
    let parts: Vec<&str> = cmd.split_whitespace().collect();

    match parts[0] {
        "/quit" | "/exit" => {
            println!("Goodbye!");
            std::process::exit(0);
        }
        "/project" => {
            if parts.len() < 2 {
                eprintln!("Usage: /project <name>");
            } else {
                orchestrator.switch_project(parts[1]);
                println!("Switched to project: {}", parts[1]);
            }
        }
        "/clear" => {
            orchestrator.clear_history();
            println!("History cleared");
        }
        "/history" => {
            let history = orchestrator.recent_history(5);
            if history.is_empty() {
                println!("No conversation history");
            } else {
                println!("\nRecent history:");
                for (i, turn) in history.iter().enumerate() {
                    println!(
                        "{}. Q: {} | A: {}",
                        i + 1,
                        truncate(&turn.query.text, 40),
                        truncate(&turn.response.text, 40)
                    );
                }
            }
        }
        _ => {
            eprintln!("Unknown command: {}", parts[0]);
            eprintln!("Type /quit to exit");
        }
    }
}

fn run_single_query(query: &str, project: Option<&str>) {
    let mut orchestrator = Orchestrator::new();

    if let Some(proj) = project {
        orchestrator.switch_project(proj);
    }

    let query = Query::new(query);
    match orchestrator.process(query) {
        Ok(response) => {
            println!("{}", response.text);
            if env::var("VERBOSE").is_ok() {
                eprintln!(
                    "\n[Route: {:?}, Confidence: {:.2}, Latency: {}ms]",
                    response.route, response.confidence, response.latency_ms
                );
            }
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }
    }
}

fn print_help() {
    println!("Mobile AI Orchestrator v{}", mobile_ai_orchestrator::VERSION);
    println!("RSR Compliance: {}", mobile_ai_orchestrator::RSR_COMPLIANCE);
    println!();
    println!("USAGE:");
    println!("    mobile-ai [OPTIONS] [QUERY]");
    println!();
    println!("OPTIONS:");
    println!("    -i, --interactive       Interactive mode");
    println!("    -p, --project <NAME>    Set project context");
    println!("    -h, --help              Print help information");
    println!("    -v, --version           Print version information");
    println!();
    println!("EXAMPLES:");
    println!("    mobile-ai \"How do I iterate a HashMap?\"");
    println!("    mobile-ai --project oblibeny \"Explain type system\"");
    println!("    mobile-ai --interactive");
    println!();
    println!("ENVIRONMENT:");
    println!("    VERBOSE=1               Show detailed routing information");
}

fn print_version() {
    println!("mobile-ai {}", mobile_ai_orchestrator::VERSION);
    println!("RSR Compliance: {}", mobile_ai_orchestrator::RSR_COMPLIANCE);
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        format!("{}...", &s[..max])
    }
}

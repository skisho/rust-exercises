//! Runner for the exercises in `exercises/`.
//!
//! Each exercise is a binary in the `exercises` package. An exercise is
//! complete when it compiles, its tests (or `main`) pass, and its
//! `// I AM NOT DONE` marker has been deleted.

mod exercises;

use exercises::{EXERCISES, Exercise, Mode};
use std::collections::HashMap;
use std::io::{self, BufRead, IsTerminal, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};
use std::sync::mpsc;
use std::time::{Duration, SystemTime};
use std::{env, fs, thread};

const MARKER: &str = "// I AM NOT DONE";
const ROOT: &str = env!("CARGO_MANIFEST_DIR");

const USAGE: &str = "\
Usage: cargo run -- [COMMAND]

Commands:
  watch              Re-check your progress whenever you save (default)
  verify             Check every exercise in order, stopping at the first unfinished one
  list               Show all exercises and whether they're done
  run <name>         Compile and run a single exercise, showing its output
  hint <name>        Show a hint for an exercise
  solution <name>    Print the reference solution for an exercise
  reset <name>       Restore an exercise to its original state (uses git)
  verify-solutions   Check that every reference solution passes
  help               Show this message";

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    let args: Vec<&str> = args.iter().map(String::as_str).collect();

    let result = match args.as_slice() {
        [] | ["watch"] => watch(),
        ["verify"] => verify(0).map(|_| ()),
        ["list"] => {
            list();
            Ok(())
        }
        ["run", name] => find(name).and_then(run_one),
        ["hint", name] => find(name).map(|ex| println!("{}", ex.hint)),
        ["solution", name] => find(name).and_then(show_solution),
        ["reset", name] => find(name).and_then(reset),
        ["verify-solutions"] => verify_solutions(),
        ["help" | "--help" | "-h"] => {
            println!("{USAGE}");
            Ok(())
        }
        _ => Err(format!("unrecognised command\n\n{USAGE}")),
    };

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(msg) => {
            eprintln!("{}", red(&msg));
            ExitCode::FAILURE
        }
    }
}

// ---------------------------------------------------------------------------
// Checking exercises

#[derive(Clone, Copy)]
enum Target {
    Exercises,
    Solutions,
}

enum Outcome {
    /// Compiles and passes, and the marker is gone.
    Done,
    /// Compiles and passes, but the marker is still there.
    Passing,
    /// Doesn't compile, or tests fail. Holds the cargo output.
    Failing(String),
}

pub fn has_marker(source: &str) -> bool {
    source.lines().any(|line| line.trim() == MARKER)
}

fn source_path(ex: &Exercise, target: Target) -> PathBuf {
    let rel = match target {
        Target::Exercises => ex.path(),
        Target::Solutions => ex.solution_path(),
    };
    Path::new(ROOT).join(rel)
}

/// Builds and runs (or tests) one exercise with cargo, capturing its output.
fn cargo(ex: &Exercise, target: Target) -> (bool, String) {
    let manifest = match target {
        Target::Exercises => "exercises/Cargo.toml",
        Target::Solutions => "solutions/Cargo.toml",
    };
    let subcommand = match ex.mode {
        Mode::Run => "run",
        Mode::Test => "test",
    };
    let output = Command::new(env::var("CARGO").unwrap_or_else(|_| "cargo".into()))
        .current_dir(ROOT)
        .args([
            subcommand,
            "--quiet",
            "--manifest-path",
            manifest,
            "--bin",
            ex.name,
        ])
        .args(["--color", if use_color() { "always" } else { "never" }])
        .output();

    match output {
        Ok(out) => {
            let mut text = String::from_utf8_lossy(&out.stdout).into_owned();
            text.push_str(&String::from_utf8_lossy(&out.stderr));
            (out.status.success(), text)
        }
        Err(e) => (false, format!("failed to run cargo: {e}")),
    }
}

fn check(ex: &Exercise, target: Target) -> Outcome {
    let (ok, output) = cargo(ex, target);
    if !ok {
        return Outcome::Failing(output);
    }
    let source = fs::read_to_string(source_path(ex, target)).unwrap_or_default();
    if has_marker(&source) {
        Outcome::Passing
    } else {
        Outcome::Done
    }
}

/// Checks exercises in order starting at `from`. Returns the index of the
/// first unfinished exercise, or `EXERCISES.len()` if everything is done.
fn verify(from: usize) -> Result<usize, String> {
    for (i, ex) in EXERCISES.iter().enumerate().skip(from) {
        print!("Checking {}... ", ex.name);
        io::stdout().flush().ok();

        match check(ex, Target::Exercises) {
            Outcome::Done => println!("{}", green("done")),
            Outcome::Passing => {
                println!("{}", green("passing!"));
                println!();
                println!(
                    "{} {} compiles and passes. Read through it again if you like,\n\
                     then delete the `{MARKER}` line to move on.",
                    green("✓"),
                    bold(&ex.path()),
                );
                progress(i);
                return Ok(i);
            }
            Outcome::Failing(output) => {
                println!("{}", red("not yet"));
                println!();
                print!("{output}");
                println!();
                println!(
                    "{} {} needs fixing. Run `cargo run -- hint {}` if you're stuck.",
                    red("✗"),
                    bold(&ex.path()),
                    ex.name,
                );
                progress(i);
                return Ok(i);
            }
        }
    }

    println!();
    println!(
        "{}",
        green("🎉 All exercises complete! Congratulations, Rustacean! 🦀")
    );
    println!("For more, try the Rust book (https://doc.rust-lang.org/book/) or Rust by Example.");
    Ok(EXERCISES.len())
}

fn progress(done: usize) {
    let total = EXERCISES.len();
    let width = 40;
    let filled = done * width / total;
    println!(
        "Progress: [{}{}] {done}/{total}",
        green(&"#".repeat(filled)),
        "-".repeat(width - filled)
    );
}

fn verify_solutions() -> Result<(), String> {
    let mut failures = Vec::new();
    for ex in EXERCISES {
        print!("Checking solution {}... ", ex.name);
        io::stdout().flush().ok();
        match check(ex, Target::Solutions) {
            Outcome::Done => println!("{}", green("ok")),
            Outcome::Passing => {
                println!("{}", red("has marker"));
                failures.push(ex.name);
            }
            Outcome::Failing(output) => {
                println!("{}\n{output}", red("FAILED"));
                failures.push(ex.name);
            }
        }
    }
    if failures.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "{} solution(s) failed: {}",
            failures.len(),
            failures.join(", ")
        ))
    }
}

// ---------------------------------------------------------------------------
// Watch mode

fn watch() -> Result<(), String> {
    let mut current = verify(0)?;
    print_watch_help();

    // Read commands from stdin on a separate thread so we can keep polling files.
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        for line in io::stdin().lock().lines() {
            let Ok(line) = line else { break };
            if tx.send(line).is_err() {
                break;
            }
        }
    });

    let mut seen = modification_times();
    loop {
        thread::sleep(Duration::from_millis(300));

        if let Ok(cmd) = rx.try_recv() {
            match cmd.trim() {
                "q" | "quit" => return Ok(()),
                "h" | "hint" => match EXERCISES.get(current) {
                    Some(ex) => println!("{} {}", bold("Hint:"), ex.hint),
                    None => println!("Everything's done: no hints needed!"),
                },
                "l" | "list" => list(),
                "c" | "check" => {
                    clear_screen();
                    current = verify(0)?;
                    print_watch_help();
                }
                "" => {}
                _ => print_watch_help(),
            }
        }

        let now = modification_times();
        if now != seen {
            // Re-check from the earliest exercise that changed, so editing an
            // already-finished exercise can't silently break it.
            let earliest_changed = EXERCISES
                .iter()
                .position(|ex| now.get(&ex.path()) != seen.get(&ex.path()))
                .unwrap_or(current);
            seen = now;
            clear_screen();
            current = verify(earliest_changed.min(current))?;
            print_watch_help();
        }
    }
}

fn print_watch_help() {
    println!();
    println!("Watching for changes. Commands: [h]int, [l]ist, [c]heck all, [q]uit (then Enter)");
}

fn modification_times() -> HashMap<String, SystemTime> {
    EXERCISES
        .iter()
        .filter_map(|ex| {
            let mtime = fs::metadata(Path::new(ROOT).join(ex.path()))
                .ok()?
                .modified()
                .ok()?;
            Some((ex.path(), mtime))
        })
        .collect()
}

fn clear_screen() {
    if use_color() {
        print!("\x1b[2J\x1b[H");
    }
}

// ---------------------------------------------------------------------------
// Other commands

fn find(name: &str) -> Result<&'static Exercise, String> {
    EXERCISES.iter().find(|ex| ex.name == name).ok_or_else(|| {
        format!("no exercise named `{name}`. Run `cargo run -- list` to see them all.")
    })
}

fn list() {
    println!(
        "{}",
        bold(&format!(
            "{:<20} {:<22} {:<6} {}",
            "Name", "Topic", "Mode", "Status"
        ))
    );
    for ex in EXERCISES {
        let source = fs::read_to_string(source_path(ex, Target::Exercises)).unwrap_or_default();
        let status = if has_marker(&source) {
            red("pending")
        } else {
            green("done")
        };
        let mode = match ex.mode {
            Mode::Run => "run",
            Mode::Test => "test",
        };
        println!("{:<20} {:<22} {:<6} {status}", ex.name, ex.dir, mode);
    }
    println!();
    println!(
        "(\"done\" just means the marker was removed; `cargo run -- verify` checks for real.)"
    );
}

fn run_one(ex: &Exercise) -> Result<(), String> {
    let (ok, output) = cargo(ex, Target::Exercises);
    print!("{output}");
    if ok {
        println!("{}", green(&format!("✓ {} passed", ex.name)));
        Ok(())
    } else {
        Err(format!(
            "✗ {} failed. Try `cargo run -- hint {}`.",
            ex.name, ex.name
        ))
    }
}

fn show_solution(ex: &Exercise) -> Result<(), String> {
    let path = source_path(ex, Target::Solutions);
    let source =
        fs::read_to_string(&path).map_err(|e| format!("couldn't read {}: {e}", path.display()))?;
    println!("{}\n", bold(&ex.solution_path()));
    println!("{source}");
    Ok(())
}

fn reset(ex: &Exercise) -> Result<(), String> {
    let status = Command::new("git")
        .current_dir(ROOT)
        .args(["checkout", "--", &ex.path()])
        .status()
        .map_err(|e| format!("failed to run git: {e}"))?;
    if status.success() {
        println!("Reset {}", ex.path());
        Ok(())
    } else {
        Err("git couldn't restore the file. Has the repository been committed yet?".into())
    }
}

// ---------------------------------------------------------------------------
// Terminal colours

fn use_color() -> bool {
    env::var_os("NO_COLOR").is_none() && io::stdout().is_terminal()
}

fn paint(code: &str, text: &str) -> String {
    if use_color() {
        format!("\x1b[{code}m{text}\x1b[0m")
    } else {
        text.to_string()
    }
}

fn red(text: &str) -> String {
    paint("31", text)
}

fn green(text: &str) -> String {
    paint("32", text)
}

fn bold(text: &str) -> String {
    paint("1", text)
}

//! Command line of the harness. Every contact with the outside arrives through `World`.

use crate::audit;
use std::io;
use std::io::Write;
use std::path::{Path, PathBuf};

pub const CLIPPY_TOML: &str = include_str!("../clippy.toml");
pub const EXIT_CLEAN: u8 = 0;
pub const EXIT_FINDINGS: u8 = 1;
pub const EXIT_ERROR: u8 = 2;
const BUILD_DIRECTORY: &str = "target";
const USAGE: &str = "usage: rust-harness [--syntax-only] [--exclude NAME]... PATH...";

pub struct ToolCall {
    pub program: String,
    pub arguments: Vec<String>,
    pub directory: Option<PathBuf>,
    pub clippy_conf_dir: PathBuf,
}

pub struct ToolOutcome {
    pub success: bool,
    pub output: String,
}

pub struct World<'w> {
    pub read_file: &'w dyn Fn(&Path) -> io::Result<String>,
    pub list_dir: &'w dyn Fn(&Path) -> io::Result<Vec<PathBuf>>,
    pub is_dir: &'w dyn Fn(&Path) -> bool,
    pub is_file: &'w dyn Fn(&Path) -> bool,
    pub run_tool: &'w dyn Fn(&ToolCall) -> io::Result<ToolOutcome>,
}

struct Options {
    syntax_only: bool,
    excluded: Vec<String>,
    paths: Vec<PathBuf>,
}

#[derive(Default)]
struct Tally {
    files: usize,
    findings: usize,
    clippy_failures: usize,
}

pub fn run(arguments: &[String], scratch: &Path, world: &World, out: &mut dyn Write) -> u8 {
    match run_checked(arguments, scratch, world, out) {
        Ok(code) => code,
        Err(message) => {
            // Nothing else can be reported if the output itself is what failed.
            let _ = writeln!(out, "error: {message}");
            EXIT_ERROR
        }
    }
}

fn run_checked(
    arguments: &[String],
    scratch: &Path,
    world: &World,
    out: &mut dyn Write,
) -> Result<u8, String> {
    let options = parse_arguments(arguments)?;
    let mut tally = Tally::default();
    for path in &options.paths {
        audit_path(path, &options, scratch, world, out, &mut tally)?;
    }
    // An audit that read nothing must not look like an audit that found nothing.
    if tally.files == 0 {
        return Err("no Rust files found, nothing was audited".to_string());
    }
    writeln!(
        out,
        "rust-harness: {} file(s), {} finding(s), {} Clippy failure(s)",
        tally.files, tally.findings, tally.clippy_failures
    )
    .map_err(|error| error.to_string())?;
    if tally.findings == 0 && tally.clippy_failures == 0 {
        Ok(EXIT_CLEAN)
    } else {
        Ok(EXIT_FINDINGS)
    }
}

fn parse_arguments(arguments: &[String]) -> Result<Options, String> {
    let mut options = Options {
        syntax_only: false,
        excluded: Vec::new(),
        paths: Vec::new(),
    };
    let mut remaining = arguments.iter();
    while let Some(argument) = remaining.next() {
        match argument.as_str() {
            "--syntax-only" => options.syntax_only = true,
            "--exclude" => {
                let name = remaining.next().ok_or(USAGE)?;
                options.excluded.push(name.clone());
            }
            flag if flag.starts_with('-') => return Err(USAGE.to_string()),
            path => options.paths.push(PathBuf::from(path)),
        }
    }
    if options.paths.is_empty() {
        return Err(USAGE.to_string());
    }
    Ok(options)
}

fn audit_path(
    path: &Path,
    options: &Options,
    scratch: &Path,
    world: &World,
    out: &mut dyn Write,
    tally: &mut Tally,
) -> Result<(), String> {
    for file in collect_rust_files(path, &options.excluded, world)? {
        let source = (world.read_file)(&file)
            .map_err(|error| format!("cannot read {}: {error}", file.display()))?;
        let findings = audit::audit_source(&base_name(&file), &source);
        tally.files += 1;
        tally.findings += findings.len();
        for finding in &findings {
            writeln!(
                out,
                "error: [{}] {}\n  --> {}:{}:{}",
                finding.rule,
                finding.message,
                file.display(),
                finding.line,
                finding.column
            )
            .map_err(|error| error.to_string())?;
        }
    }
    if options.syntax_only {
        return Ok(());
    }
    match clippy_call(path, scratch, world) {
        Some(call) => {
            let outcome = (world.run_tool)(&call)
                .map_err(|error| format!("cannot run {}: {error}", call.program))?;
            out.write_all(outcome.output.as_bytes())
                .map_err(|error| error.to_string())?;
            if !outcome.success {
                tally.clippy_failures += 1;
            }
        }
        None => writeln!(
            out,
            "note: {} has no Cargo.toml: only the syntax rules ran",
            path.display()
        )
        .map_err(|error| error.to_string())?,
    }
    Ok(())
}

fn collect_rust_files(
    root: &Path,
    excluded: &[String],
    world: &World,
) -> Result<Vec<PathBuf>, String> {
    if !(world.is_dir)(root) {
        if (world.is_file)(root) {
            return Ok(vec![root.to_path_buf()]);
        }
        return Err(format!("{} does not exist", root.display()));
    }
    let mut files = Vec::new();
    let mut pending = vec![root.to_path_buf()];
    while let Some(directory) = pending.pop() {
        let mut entries = (world.list_dir)(&directory)
            .map_err(|error| format!("cannot list {}: {error}", directory.display()))?;
        entries.sort();
        for entry in entries {
            let name = base_name(&entry);
            if (world.is_dir)(&entry) {
                let skipped =
                    name.starts_with('.') || name == BUILD_DIRECTORY || excluded.contains(&name);
                if !skipped {
                    pending.push(entry);
                }
            } else if name.ends_with(".rs") {
                files.push(entry);
            }
        }
    }
    files.sort();
    Ok(files)
}

fn clippy_call(path: &Path, scratch: &Path, world: &World) -> Option<ToolCall> {
    let flags = clippy_flags();
    if (world.is_file)(path) {
        let mut arguments: Vec<String> = [
            "--edition",
            "2021",
            "--crate-type",
            "lib",
            "--emit=metadata",
            "--out-dir",
        ]
        .map(String::from)
        .to_vec();
        arguments.push(scratch.display().to_string());
        arguments.extend(flags);
        arguments.push(path.display().to_string());
        return Some(ToolCall {
            program: "clippy-driver".to_string(),
            arguments,
            directory: None,
            clippy_conf_dir: scratch.to_path_buf(),
        });
    }
    if !(world.is_file)(&path.join("Cargo.toml")) {
        return None;
    }
    let mut arguments: Vec<String> = [
        "clippy",
        "--quiet",
        "--workspace",
        "--all-targets",
        "--no-deps",
        "--",
    ]
    .map(String::from)
    .to_vec();
    arguments.extend(flags);
    Some(ToolCall {
        program: "cargo".to_string(),
        arguments,
        directory: Some(path.to_path_buf()),
        clippy_conf_dir: scratch.to_path_buf(),
    })
}

fn clippy_flags() -> Vec<String> {
    let mut flags = vec!["-A".to_string(), "clippy::all".to_string()];
    for lint in audit::ENFORCED_LINTS {
        flags.push("-D".to_string());
        flags.push((*lint).to_string());
    }
    flags
}

fn base_name(path: &Path) -> String {
    path.file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_default()
}

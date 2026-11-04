//! Rust-Harness: rejects code whose functions depend on more than their signature says.

mod app;
mod audit;

use std::io::Write;
use std::path::{Path, PathBuf};

// The only function allowed to touch the outside: everything below goes down as parameters.
#[allow(clippy::disallowed_methods)]
fn main() -> std::process::ExitCode {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    let scratch = std::env::temp_dir().join(format!("rust-harness-{}", std::process::id()));
    let read_file = |path: &Path| std::fs::read_to_string(path);
    let list_dir = |path: &Path| -> std::io::Result<Vec<PathBuf>> {
        std::fs::read_dir(path)?
            .map(|entry| entry.map(|found| found.path()))
            .collect()
    };
    // Links are not followed, so a cycle of links cannot trap the walk.
    let is_dir = |path: &Path| path.symlink_metadata().is_ok_and(|data| data.is_dir());
    let is_file = |path: &Path| path.is_file();
    let run_tool = |call: &app::ToolCall| -> std::io::Result<app::ToolOutcome> {
        let mut command = std::process::Command::new(&call.program);
        command
            .args(&call.arguments)
            .env("CLIPPY_CONF_DIR", &call.clippy_conf_dir);
        if let Some(directory) = &call.directory {
            command.current_dir(directory);
        }
        let finished = command.output()?;
        Ok(app::ToolOutcome {
            success: finished.status.success(),
            output: String::from_utf8_lossy(&finished.stdout).into_owned()
                + &String::from_utf8_lossy(&finished.stderr),
        })
    };
    let world = app::World {
        read_file: &read_file,
        list_dir: &list_dir,
        is_dir: &is_dir,
        is_file: &is_file,
        run_tool: &run_tool,
    };
    let mut stdout = std::io::stdout();
    let prepared = std::fs::create_dir_all(&scratch)
        .and_then(|()| std::fs::write(scratch.join("clippy.toml"), app::CLIPPY_TOML));
    let code = match prepared {
        Ok(()) => app::run(&arguments, &scratch, &world, &mut stdout),
        Err(error) => {
            let _ = writeln!(
                stdout,
                "error: cannot prepare {}: {error}",
                scratch.display()
            );
            app::EXIT_ERROR
        }
    };
    let _ = std::fs::remove_dir_all(&scratch);
    std::process::ExitCode::from(code)
}

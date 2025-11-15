use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use xshell::{cmd, Shell};

#[derive(Parser)]
#[command(name = "cargo-xtask", version, about = "Project automation tasks")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Format Rust and frontend sources
    Fmt,
    /// Run lint and static analysis checks
    Check,
    /// Build production bundles (frontend + Tauri)
    Package,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let project_root = project_root();
    let shell = Shell::new()?;
    let _ = shell.push_dir(project_root);

    match cli.command {
        Command::Fmt => run_fmt(&shell),
        Command::Check => run_check(&shell),
        Command::Package => run_package(&shell),
    }
}

fn run_fmt(shell: &Shell) -> Result<()> {
    let pkg = package_manager_cmd();
    cmd!(shell, "cargo fmt --all")
        .run()
        .context("failed to run cargo fmt")?;
    cmd!(shell, "{pkg} run format")
        .run()
        .context("failed to run frontend formatter")?;
    Ok(())
}

fn run_check(shell: &Shell) -> Result<()> {
    let pkg = package_manager_cmd();
    cmd!(shell, "cargo fmt --all -- --check")
        .run()
        .context("cargo fmt --check failed")?;
    cmd!(
        shell,
        "cargo clippy --workspace --all-targets --all-features -- -D warnings"
    )
    .run()
    .context("cargo clippy failed")?;
    cmd!(shell, "{pkg} run lint")
        .run()
        .context("frontend lint failed")?;
    cmd!(shell, "{pkg} run format:check")
        .run()
        .context("frontend format:check failed")?;
    Ok(())
}

fn run_package(shell: &Shell) -> Result<()> {
    let pkg = package_manager_cmd();
    cmd!(shell, "{pkg} install --frozen-lockfile")
        .run()
        .context("failed to install frontend dependencies")?;
    cmd!(shell, "{pkg} run build")
        .run()
        .context("frontend build failed")?;
    cmd!(shell, "{pkg} run tauri build")
        .run()
        .context("tauri build failed")?;
    Ok(())
}

fn project_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
}

fn package_manager_cmd() -> &'static str {
    if cfg!(windows) {
        "pnpm.cmd"
    } else {
        "pnpm"
    }
}

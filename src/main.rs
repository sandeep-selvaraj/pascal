mod cli;
mod commands;
mod config;
mod display;
mod error;
mod git;
mod template;
mod uv;
mod workspace;

use anyhow::Result;
use clap::Parser;

use cli::{Cli, Commands, CreateKind};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { name, python } => {
            commands::init::run(name, &python)?;
        }

        Commands::Create { kind } => match kind {
            CreateKind::Package { name } => {
                commands::create::run_package(&name)?;
            }
            CreateKind::App { name } => {
                commands::create::run_app(&name)?;
            }
        },

        Commands::Add { package, to } => {
            commands::add::run(&package, &to)?;
        }

        Commands::Info => {
            commands::info::run()?;
        }

        Commands::Deps { graph } => {
            commands::deps::run(graph)?;
        }

        Commands::Check => {
            commands::check::run()?;
        }

        Commands::Diff { since } => {
            commands::diff::run(since)?;
        }

        Commands::Test { changed, name, extra } => {
            commands::test::run(changed, name, &extra)?;
        }

        Commands::Build { app } => {
            commands::build::run(&app)?;
        }

        Commands::Run { app, extra } => {
            commands::run::run(&app, &extra)?;
        }

        Commands::Sync => {
            commands::sync::run()?;
        }
    }

    Ok(())
}

use clap::Parser;
use std::process;

mod cli;
mod config;
mod commands;

use cli::{Cli, Commands};
use config::{load_config, load_config_from_path};

fn main() {
    let cli = Cli::parse();

    let config = match cli.config.as_ref() {
        Some(config_path) => {
            match load_config_from_path(config_path) {
                Ok(config) => config,
                Err(e) => {
                    eprintln!("Error: Failed to load config from {}: {}", config_path.display(), e);
                    process::exit(1);
                }
            }
        }
        None => {
            match load_config() {
                Ok(config) => config,
                Err(e) => {
                    eprintln!("Warning: Failed to load config: {}", e);
                    // Continue with default behavior
                    config::Config {
                        fav_dirs: vec![],
                        exclude: vec![],
                        editor: None,
                        project_markers: None,
                        max_depth: None,
                        preview_max_size: None,
                    }
                }
            }
        }
    };

    let result = match cli.command {
        Some(Commands::GF) => {
            commands::global_find::run(&config)
        }
        Some(Commands::GS) => {
            eprintln!("Global Search (gs) - Not yet implemented");
            Ok(())
        }
        Some(Commands::PF) => {
            eprintln!("Project Find (pf) - Not yet implemented");
            Ok(())
        }
        Some(Commands::PS) => {
            eprintln!("Project Search (ps) - Not yet implemented");
            Ok(())
        }
        Some(Commands::MS) => {
            eprintln!("Media Search (ms) - Not yet implemented");
            Ok(())
        }
        Some(Commands::DF) => {
            eprintln!("Document Find (df) - Not yet implemented");
            Ok(())
        }
        None => {
            //default to gf search when no command is provided
            commands::global_find::run(&config)
        }
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

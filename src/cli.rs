use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name="blink")]
#[command(about = "A file finder and project navigation tool")]
#[command(version = "0.1.0")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Override the default config file path
    #[arg(long, help = "Path to custom config file")]
    pub config: Option<std::path::PathBuf>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Global Find - Search for files using fd and fzf
    #[command(name = "gf")]
    #[command(about = "Find files interactively using fd and fzf")]
    GF,
    
    /// Global Search - Search file content using ripgrep
    #[command(name = "gs")]
    #[command(about = "Search file content system-wide")]
    GS,
    
    /// Project Find - Find project directories
    #[command(name = "pf")]
    #[command(about = "Find project root directories")]
    PF,
    
    /// Project Search - Search content within project scope
    #[command(name = "ps")]
    #[command(about = "Search file content within current project")]
    PS,
    
    /// Media Search - Find and preview media files
    #[command(name = "ms")]
    #[command(about = "Find media files with preview")]
    MS,
    
    /// Document Find - Find and open documents
    #[command(name = "df")]
    #[command(about = "Find documents and open with viewer")]
    DF,
}
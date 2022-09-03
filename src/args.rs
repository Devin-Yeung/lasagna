use std::path::{PathBuf};
use clap::Parser;


#[derive(Parser, Clone)]
// #[clap(infer_subcommands(true))]
pub struct CLI {
    /// Target Directory to be zipped, current directory by default
    #[clap(long, short, value_parser, required(false))]
    pub input: Option<PathBuf>,
    /// Output file name, {parent_directory}.zip by default
    #[clap(long, short, value_parser, required(false))]
    pub output: Option<PathBuf>,
    /// Max depth in the tree view
    #[clap(long, short, value_parser)]
    pub depth: Option<usize>,
    /// Health Check
    #[clap(long, action = clap::ArgAction::Count)]
    health_check: u8,
    /// Turn debugging information on
    #[clap(long, action = clap::ArgAction::Count)]
    debug: u8,
    /// Dry Run
    #[clap(long, action = clap::ArgAction::SetTrue)]
    pub dry_run: bool,
    /// ignore hidden files
    #[clap(long, action = clap::ArgAction::SetTrue, default_value="false")]
    pub ignore_hidden: bool,
    /// read gitignore as part of .zipignore
    #[clap(long, action = clap::ArgAction::SetTrue, default_value="false")]
    pub read_gitignore: bool,
}

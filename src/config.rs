use crate::args::CLI;
use crate::{args, parent_dir_name};
use std::path::PathBuf;

pub struct Config {
    pub input: PathBuf,
    pub output: PathBuf,
    pub depth: Option<usize>,
    pub ignore_hidden: bool,
    pub read_gitignore: bool,
    pub parent: bool,
}

impl From<args::CLI> for Config {
    fn from(args: CLI) -> Self {
        let default_input = std::fs::canonicalize(PathBuf::from("./")).unwrap();
        let default_output = PathBuf::from(format!("./{}.zip", parent_dir_name(&default_input)));
        Config {
            input: args.input.unwrap_or_else(|| default_input),
            output: args.output.unwrap_or_else(|| default_output),
            depth: args.depth,
            ignore_hidden: args.ignore_hidden,
            read_gitignore: args.read_gitignore,
            parent: args.parent,
        }
    }
}

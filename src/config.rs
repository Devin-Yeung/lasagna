use crate::args::CLI;
use crate::{args, parent_dir_name};
use std::path::PathBuf;

#[derive(Debug)]
pub struct Config {
    pub input: PathBuf,
    pub output: PathBuf,
    pub depth: Option<usize>,
    pub ignore_hidden: bool,
    pub read_gitignore: bool,
    pub parent: bool,
    pub compression_method: zip::CompressionMethod,
    pub compression_level: Option<i64>,
}

impl From<args::CLI> for Config {
    fn from(args: CLI) -> Self {
        let default_input = std::fs::canonicalize(PathBuf::from("./")).unwrap();
        let default_output = PathBuf::from(format!("./{}.zip", parent_dir_name(&default_input)));
        Config {
            input: args.input.unwrap_or(default_input),
            output: args.output.unwrap_or(default_output),
            depth: args.depth,
            ignore_hidden: args.ignore_hidden,
            read_gitignore: args.read_gitignore,
            parent: args.parent,
            compression_method: args.compression_method.into(),
            compression_level: args.compression_level,
        }
    }
}

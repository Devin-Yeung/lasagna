use crate::args;
use crate::args::CLI;

pub struct Config {
    pub depth: Option<usize>,
    pub ignore_hidden: bool,
    pub read_gitignore: bool,
}

impl From<args::CLI> for Config {
    fn from(args: CLI) -> Self {
        Config {
            depth: args.depth,
            ignore_hidden: args.ignore_hidden,
            read_gitignore: args.read_gitignore,
        }
    }
}

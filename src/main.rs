mod fs;
mod zipper;
mod args;
mod tree;
mod tree_view;
mod digest;
mod config;


use std::path::{PathBuf};
use clap::{Parser};
use crate::config::Config;
use crate::digest::ZipDigest;
use crate::fs::parent_dir_name;
use crate::tree::FileStream;
use crate::zipper::Zipper;


fn main() {
    let default_input = std::fs::canonicalize(PathBuf::from("./")).unwrap();
    let default_output = PathBuf::from(format!("./{}.zip", parent_dir_name(&default_input)));

    let args: args::CLI = args::CLI::parse();
    let config = Config::from(args.clone());
    let input = &args.input.unwrap_or_else(|| default_input);
    let output = &args.output.unwrap_or_else(|| default_output);

    FileStream::new(input, &config).display();

    if !args.dry_run {
        Zipper::new(input, output)
            .build_dir()
            .build_files()
            .finish();

        ZipDigest::new(output).display();
    }
}

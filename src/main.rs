mod args;
mod config;
mod digest;
mod fs;
mod tree;
mod tree_view;
mod zipper;

use crate::config::Config;
use crate::digest::ZipDigest;
use crate::fs::parent_dir_name;
use crate::tree::FileStream;
use crate::zipper::Zipper;
use clap::Parser;

fn main() {
    let args: args::CLI = args::CLI::parse();
    let config = Config::from(args.clone());

    FileStream::new(&config).display();

    if !args.dry_run {
        Zipper::new(&config).build_dir().build_files().finish();

        ZipDigest::new(config.output).display();
    }
}

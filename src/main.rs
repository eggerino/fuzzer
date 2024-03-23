use std::{fs, path::PathBuf};

use clap::Parser;
use fuzzer::{files, matching};

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    dir: Option<String>,

    #[arg(short, long)]
    git: bool,

    search_term: String,
}

fn main() {
    let args = Args::parse();

    let files = match (args.dir, args.git) {
        (Some(dir), true) => files::from_dir_git_files(&PathBuf::from(dir)),
        (None, true) => files::from_current_dir_git_files(),
        (Some(dir), false) => files::from_dir(&PathBuf::from(dir)),
        (None, false) => files::from_current_dir(),
    };

    for matching_file in files.into_iter().filter(|file| {
        fs::read_to_string(file)
            .map(|content| matching::fuzzy_match(&args.search_term, &content))
            .unwrap_or(false)
    }) {
        println!("{}", matching_file.canonicalize().unwrap().display());
    }
}

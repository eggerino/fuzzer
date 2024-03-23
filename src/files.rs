use std::{env, io::BufRead, path::PathBuf, process::Command};

use walkdir::WalkDir;

pub fn from_dir(dir: &PathBuf) -> Vec<PathBuf> {
    WalkDir::new(dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|x| x.file_type().is_file())
        .map(|x| x.path().to_path_buf())
        .collect()
}

pub fn from_current_dir() -> Vec<PathBuf> {
    env::current_dir()
        .map(|x| from_dir(&x))
        .unwrap_or(Vec::new())
}

pub fn from_dir_git_files(dir: &PathBuf) -> Vec<PathBuf> {
    Command::new("git")
        .arg("-C")
        .arg(dir)
        .arg("ls-files")
        .output()
        .map(|output| {
            output
                .stdout
                .lines()
                .filter_map(Result::ok)
                .map(|file| {
                    let mut path = dir.clone();
                    path.push(file);
                    path
                })
                .collect()
        })
        .unwrap_or(Vec::new())
}

pub fn from_current_dir_git_files() -> Vec<PathBuf> {
    env::current_dir()
        .map(|x| from_dir_git_files(&x))
        .unwrap_or(Vec::new())
}

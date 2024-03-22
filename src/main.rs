use fuzzer::files;

fn main() {
    for file in files::from_git_files() {
        println!("{}", file.display());
    }
}

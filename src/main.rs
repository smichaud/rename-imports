use std::path::PathBuf;

use rename_imports::args::{Parser, RenameImportsArgs};

// [TODO]: Optional project_root, if not provided cwd
// [TODO]: Detect if file is already renamed, if not apply_rename
// [TODO]: Multithread that shit
// [TODO]: Make it work with Python files
// [TODO]: Have a look at the log crate, could add --verbose and display logging accordingly
fn main() {
    let args = RenameImportsArgs::parse();

    rename_imports::run(
        PathBuf::from(args.src),
        PathBuf::from(args.dst),
        match args.project_root {
            Some(path_string) => Some(PathBuf::from(path_string)),
            None => None,
        },
    )
    .expect("Unable to run the program");
}

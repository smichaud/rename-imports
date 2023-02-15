use std::{
    fs,
    path::{Path, PathBuf},
};

use rename_imports::{
    args::{Parser, RenameImportsArgs},
    files,
};

fn main() {
    let args = RenameImportsArgs::parse();
    if (args.apply_rename) {
        fs::rename(
            PathBuf::from(args.src.clone()),
            PathBuf::from(args.dst.clone()),
        )
        .expect("Unable to rename the file")
    }
    rename_imports::run(
        PathBuf::from(args.src),
        PathBuf::from(args.dst),
        Some(PathBuf::from(args.project_root)),
    )
    .expect("Unable to run the program");
}

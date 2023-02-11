use std::path::PathBuf;

use common::TestSetupBuilder;
// use rename_imports::files;
// use std::fs::File;
// use std::io::{self, Write};

mod common;

#[test]
fn test_use_git_ignore() {
    let setup = TestSetupBuilder::new();
    let ts_project_dir_path = setup.set_ts_project();

    let src_file = ts_project_dir_path.join("src/dir1/mod1.ts");
    let dst_file = ts_project_dir_path.join("src/dir2/mod1.ts");
    rename_imports::run(src_file, dst_file, Some(PathBuf::from("."))).expect("RenameImport failed");

    assert!(ts_project_dir_path.join("README.md").exists())
}

#[test]
fn test_given_no_project_dir_then_use_current_dir() {
    // [TODO]: Hardcode expected string in each file
}

#[test]
fn test_given_typescript_files_then_renamed_file_imports_are_adjusted() {
    // [TODO]: Hardcode expected string in each file
}

#[test]
fn test_given_typescript_files_then_other_files_imports_are_adjusted() {
    // [TODO]: Hardcode expected string in each file
}

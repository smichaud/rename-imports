use std::path::PathBuf;

// use rename_imports::files;
// use std::fs::File;
// use std::io::{self, Write};
use tempdir::TempDir;

mod common;

#[test]
fn test_use_git_ignore() {
    let dir_path = setup_ts_project_dir();
    assert!(dir_path.join("README.md").exists())
}

#[test]
fn test_rename_typescript_imports() {
    // [TODO]: Hardcode expected string in each file
}

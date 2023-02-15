use std::{
    fs, io,
    path::{Path, PathBuf},
};
use tempdir::TempDir;

// pub const AN_IGNORED_FILE_RELATIVE_PATH: &str = "src/ignoreme/ignoredMod.ts";

pub struct TestSetupBuilder {
    temp_dir: TempDir,
}

impl TestSetupBuilder {
    pub fn new() -> TestSetupBuilder {
        let td = TempDir::new("rust_tests").expect("Couldn't create dir.");
        TestSetupBuilder { temp_dir: td }
    }

    pub fn set_ts_project(&self) -> PathBuf {
        let ts_project_dir_path = self.temp_dir.path().join("ts_project");
        copy_dir_recursive("./tests_data/ts_project", ts_project_dir_path.clone())
            .expect("Couldn't copy test data.");

        ts_project_dir_path
    }
}

pub fn copy_dir_recursive(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_recursive(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }

    Ok(())
}

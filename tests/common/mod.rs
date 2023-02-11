use std::{fs, io, path::Path};
// use tempdir::TempDir;

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

// pub fn setup_ts_project_dir() -> &Path {
//     let dir = TempDir::new("rust_tests").expect("Couldn't create dir.");
//     copy_dir_recursive("./tests_data/ts_project", &dir).expect("Couldn't copy test data.");
//     dir.path()
// }

struct SetupBuilder {}

pub fn setup_ts_project_dir() -> PathBuf {
    let dir = TempDir::new("rust_tests").expect("Couldn't create dir.");
    common::copy_dir_recursive("./tests_data/ts_project", &dir).expect("Couldn't copy test data.");

    dir.path().to_path_buf()
}

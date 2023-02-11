use std::{
    env::set_current_dir,
    error::Error,
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
};

use ignore::WalkBuilder;
use walkdir::WalkDir;

pub fn list_filepaths_with_extension(root_path: &str, extension_pattern: &str) -> Vec<String> {
    let mut paths: Vec<String> = Vec::new();
    for entry in WalkDir::new(root_path).into_iter().filter_map(|e| e.ok()) {
        if entry.metadata().unwrap().is_file()
            && entry
                .file_name()
                .to_string_lossy()
                .ends_with(extension_pattern)
        {
            paths.push(entry.path().display().to_string())
        }
    }

    paths
}

// [TODO]: This is the one! Test it with test_data
pub fn get_filtered_filepaths(root_path: &str, extension_pattern: &str) -> Vec<PathBuf> {
    let working_dir = Path::new("/home/smichaud/Desktop/ts_project");
    assert!(set_current_dir(&working_dir).is_ok());
    WalkBuilder::new(root_path)
        .build()
        .into_iter()
        .filter_map(|dir_entry| dir_entry.ok())
        .filter(|dir_entry| {
            dir_entry
                .path()
                .extension()
                .map(|ext| ext == extension_pattern)
                .unwrap_or(false)
        })
        .map(|dir_entry| dir_entry.path().to_path_buf())
        .collect()
}

pub fn get_extension_from_filename(filename: &str) -> Option<&str> {
    Path::new(filename).extension().and_then(OsStr::to_str)
}

pub fn search_imports(filepath: &str) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(filepath)?;
    println!("{filepath}");
    println!("Contains use? {}", contents.contains("use"));

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}

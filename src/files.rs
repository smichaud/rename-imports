use std::{
    error::Error,
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
};

use ignore::WalkBuilder;
use walkdir::WalkDir;

// [TODO]: This is the one! Test it with test_data
pub fn get_filtered_filepaths(root_path: PathBuf, extension_pattern: &str) -> Vec<PathBuf> {
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

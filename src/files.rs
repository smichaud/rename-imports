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

// [TODO]: Must remove extension when searching in file and replacing
pub fn get_updated_content(filepath: PathBuf, src: PathBuf, dst: PathBuf) -> Option<String> {
    let dirpath = filepath.parent()?;
    let original_diff = pathdiff::diff_paths(src.clone(), dirpath.clone())?;
    let original_diff = original_diff.with_extension("");

    let content = fs::read_to_string(filepath.clone()).ok()?;
    // println!("{content}");

    content.contains(original_diff.with_extension("").to_str()?) == false && return None;

    let refactored_diff = pathdiff::diff_paths(dst.clone(), dirpath)?;
    let refactored_diff = refactored_diff.with_extension("");

    // println!("======================================");
    // println!("filepath       {}", filepath.display());
    // println!("src            {}", src.clone().display());
    // println!("dst            {}", dst.clone().display());
    // println!("original_diff: {}", original_diff.clone().display());
    // println!("refactored_diff: {}", refactored_diff.display());
    // println!("======================================");

    // [TODO]: Prepend the "./" if does not start with ../
    // [TODO]: Surround with double quote to prevent subpath replacement
    Some(content.replace(original_diff.to_str()?, refactored_diff.to_str()?))
}

pub fn search_imports(filepath: PathBuf) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(filepath)?;
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

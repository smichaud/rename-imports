use std::{
    env::{self, set_current_dir},
    error::Error,
    ffi::OsStr,
    path::PathBuf,
};
use std::{fs, io};

use errors::{InvalidFilesState, UnhandledFiletype};
use ignore::WalkBuilder;

pub mod args;
pub mod errors;

const JAVASCRIPT_EXTENSIONS: &[&str] = &["js", "jsx", "ts", "tsx"];
const PYTHON_EXTENSIONS: &[&str] = &["py"];

enum HandledFiletype {
    JavaScript,
    Python,
}

// [TODO]: Should I be more OO, or at least have a type for this?
// struct RenameImportOptions {
//     src: PathBuf,
//     dst: PathBuf,
//     project_root: Option<PathBuf>,
// }

pub fn run(
    src: PathBuf,
    dst: PathBuf,
    project_root: Option<PathBuf>,
) -> Result<(), Box<dyn Error>> {
    let project_root = set_working_dir(project_root).expect("Unable to set working directory.");

    if !is_already_renamed(src.clone(), dst.clone()).expect("Invalid file state.") {
        apply_rename(src.clone(), dst.clone()).expect("Unable to apply file renaming.");
    }

    // [TODO]: Check if src is dir, loop through the files
    let extension = src
        .extension()
        .and_then(OsStr::to_str)
        .expect("Unable to retrieve filetype/extension.");
    let filetype = get_filetype(extension).expect("Unhandled filetype.");
    let filepaths = get_filtered_filepaths(project_root, get_filetype_extensions(filetype));
    for filepath in filepaths.iter() {
        // println!("Matching file: {}", filepath.to_str().unwrap_or(""));
        match get_updated_content(filepath.to_path_buf(), src.clone(), dst.clone()) {
            Some(updated_content) => {
                // println!("Updating {}", dst.display());
                if let Err(e) = fs::write(filepath.clone(), updated_content) {
                    println!(
                        " Unable to handle {}, because of {}",
                        filepath.to_str().unwrap_or("🤷"),
                        e.to_string()
                    )
                }
            }
            None => {
                println!("Nothing to update in {}", filepath.display());
            }
        }
    }

    Ok(())
}

fn set_working_dir(project_root: Option<PathBuf>) -> io::Result<PathBuf> {
    let project_root = project_root.unwrap_or(env::current_dir()?);
    set_current_dir(project_root.clone())?;

    Ok(project_root)
}

fn is_already_renamed(src: PathBuf, dst: PathBuf) -> Result<bool, Box<dyn Error>> {
    if src.exists() && !dst.exists() {
        return Ok(false);
    }

    if dst.exists() && !src.exists() {
        return Ok(true);
    }

    if src.exists() && dst.exists() {
        return Err(Box::new(InvalidFilesState::new(
            "Both the source and the destination files exist.",
        )));
    }

    Err(Box::new(InvalidFilesState::new(
        "Neither the source file nor the destination file exist",
    )))
}

fn apply_rename(src: PathBuf, dst: PathBuf) -> io::Result<()> {
    fs::rename(src, dst)
}

fn get_filetype(extension: &str) -> Result<HandledFiletype, Box<dyn Error>> {
    if JAVASCRIPT_EXTENSIONS.contains(&extension) {
        return Ok(HandledFiletype::JavaScript);
    }
    if PYTHON_EXTENSIONS.contains(&extension) {
        return Ok(HandledFiletype::Python);
    }

    Err(Box::new(UnhandledFiletype::new(&format!(
        "File with extension '{}' are not handled.",
        extension
    ))))
}

fn get_filetype_extensions(filetype: HandledFiletype) -> &'static [&'static str] {
    match filetype {
        HandledFiletype::JavaScript => JAVASCRIPT_EXTENSIONS,
        HandledFiletype::Python => PYTHON_EXTENSIONS,
    }
}

fn get_filtered_filepaths(root_path: PathBuf, extensions: &'static [&str]) -> Vec<PathBuf> {
    WalkBuilder::new(root_path)
        .build()
        .into_iter()
        .filter_map(|dir_entry| dir_entry.ok())
        .filter(|dir_entry| {
            dir_entry
                .path()
                .extension()
                .map(|extension| extensions.contains(&extension.to_str().unwrap_or("")))
                .unwrap_or(false)
        })
        .map(|dir_entry| dir_entry.path().to_path_buf())
        .collect()
}

// [TODO]: This should be changed to `get_update_info`, would be easier to test
pub fn get_updated_content(filepath: PathBuf, src: PathBuf, dst: PathBuf) -> Option<String> {
    let dirpath = filepath.parent()?;
    let original_path_diff = pathdiff::diff_paths(src.clone(), dirpath.clone())?;
    let original_import_string = path_diff_to_import_string(original_path_diff);

    let content = fs::read_to_string(filepath.clone()).ok()?;
    // println!("{content}");
    println!("filepath: {}", filepath.to_str().unwrap());
    println!("original_import_string: {original_import_string}");
    println!("");

    let _ = content.contains(&original_import_string) == false && return None;

    let refactored_path_diff = pathdiff::diff_paths(dst.clone(), dirpath)?;
    let refactored_import_string = path_diff_to_import_string(refactored_path_diff);

    // println!("======================================");
    // println!("filepath                  {}", filepath.display());
    // println!("src                       {}", src.clone().display());
    // println!("dst                       {}", dst.clone().display());
    // println!("original_import_string:   {}", original_import_string);
    // println!("refactored_import_string: {}", refactored_import_string);
    // println!("======================================");

    Some(content.replace(&original_import_string, &refactored_import_string))
}

fn path_diff_to_import_string(path_diff: PathBuf) -> String {
    let path_diff = path_diff.with_extension("");

    let mut import_string = String::from(path_diff.to_str().unwrap().to_string());
    if !import_string.starts_with("../") {
        import_string.insert_str(0, "./")
    }

    import_string.insert_str(0, "\"");
    import_string.push_str("\"");

    import_string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn then_remove_extension() {
        let path_diff = PathBuf::from("aFolder/to/the/file.tsx");

        assert!(!path_diff_to_import_string(path_diff).contains(".tsx"));
    }

    #[test]
    fn then_surround_in_double_quotes() {
        let path_diff = PathBuf::from("aFolder/to/the/file.tsx");
        let import_string = path_diff_to_import_string(path_diff);

        assert!(import_string.starts_with("\""));
        assert!(import_string.ends_with("\""));
    }

    #[test]
    fn given_no_dot_slash_then_add_it() {
        let path_diff = PathBuf::from("aFolder/to/the/file.tsx");

        assert!(path_diff_to_import_string(path_diff).contains("./aFolder/to/the/file"));
    }
}

use std::{
    env::{self, set_current_dir},
    error::Error,
    ffi::OsStr,
    fs::{self, File},
    path::PathBuf,
};

pub mod args;
pub mod files;
pub mod my_error;

pub fn run(
    src: PathBuf,
    dst: PathBuf,
    project_root: Option<PathBuf>,
) -> Result<(), Box<dyn Error>> {
    let project_root = project_root
        .unwrap_or(env::current_dir().expect("Unable to retrieve current working directory."));
    set_current_dir(project_root.clone()).expect("Unable to set working directory");

    let extension = src
        .extension()
        .and_then(OsStr::to_str)
        .expect("Unable to retrieve extension.");

    let filepaths = files::get_filtered_filepaths(project_root, &extension);
    for filepath in filepaths.iter() {
        println!("{}", filepath.to_str().unwrap_or(""));

        match files::get_updated_content(filepath.to_path_buf(), src.clone(), dst.clone()) {
            Some(updated_content) => {
                println!("Updating {}", dst.display());
                fs::write(dst.clone(), updated_content);
            }
            None => {
                println!("Unable to open {}", dst.display());
            }
        }
    }

    Ok(())
}

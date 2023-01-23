use std::error::Error;

pub mod args;
pub mod files;
pub mod my_error;

pub fn run(project_root: &str, src: &str, dst: &str) -> Result<(), Box<dyn Error>> {
    println!("{project_root}: {src} --> {dst}");

    let extension =
        files::get_extension_from_filename(src).ok_or(my_error::MyError::new("Broken"))?;

    let filepaths = files::list_filepaths_with_extension(project_root, &extension);
    for filepath in filepaths.iter() {
        println!("{filepath}");
    }

    println!("Get all filepath matching extension pattern");
    println!("For all files");
    println!("If matching rel path, replace and write file");

    Ok(())
}

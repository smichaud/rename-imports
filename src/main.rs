use clap::Parser;
use rename_imports::args::RenameImportsArgs;
// use rename_imports::files::list_filepaths_with_extension;

fn main() {
    let args = RenameImportsArgs::parse();

    rename_imports::run(&args.project_root, &args.src, &args.src)
        .expect("Unable to run the program");

    // list_filepaths_with_extension(&args.project_root, &args.src);

    match pathdiff::diff_paths("./a/b/c/d", "./a/b/e/f") {
        Some(relative_path) => {
            println!("{}", relative_path.to_string_lossy());
        }
        None => {
            println!("No valid path between those 2")
        }
    }
}

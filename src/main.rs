use rename_imports::args::{Parser, RenameImportsArgs};

fn main() {
    let args = RenameImportsArgs::parse();
    // rename_imports::run(&args.project_root, &args.src, &args.src)
    //     .expect("Unable to run the program");

    // match pathdiff::diff_paths("./a/b/c/d", "./a/b/e/f") {
    //     Some(relative_path) => {
    //         println!("{}", relative_path.to_string_lossy());
    //     }
    //     None => {
    //         println!("No valid path between those 2")
    //     }
    // }
}

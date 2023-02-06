use std::path::PathBuf;

// use clap::Parser;
// use rename_imports::args::RenameImportsArgs;
// use rename_imports::files::list_filepaths_with_extension;
// use ignore::types::Types;
use ignore::WalkBuilder;

// [TODO]: Try cargo-watch: cargo watch -x test
fn main() {
    // let args = RenameImportsArgs::parse();
    // rename_imports::run(&args.project_root, &args.src, &args.src)
    //     .expect("Unable to run the program");

    // list_filepaths_with_extension(&args.project_root, &args.src);

    // match pathdiff::diff_paths("./a/b/c/d", "./a/b/e/f") {
    //     Some(relative_path) => {
    //         println!("{}", relative_path.to_string_lossy());
    //     }
    //     None => {
    //         println!("No valid path between those 2")
    //     }
    // }

    // [TODO]: You could use the filter_entry, but also accept directories
    let mut walker = WalkBuilder::new("./test_data/ts_project");
    // let walker = walker.filter_entry(|entry| {
    //     // PathBuf::from(entry.path())
    //     //     .extension()
    //     //     .map(|os_str| os_str.to_str().unwrap_or(""))
    //     //     .unwrap_or("")
    //     //     == "ts"
    //     //     &&
    //     entry.file_type().unwrap().is_file()
    // });
    walker.add_ignore("./.gitignore");
    for walk_result in walker.build() {
        match walk_result {
            Ok(entry) => {
                match entry.path().extension() {
                    Some(extension) => {
                        println!("Extension: {}", extension.to_str().unwrap_or(""))
                    }
                    None => {}
                }
                if (entry.file_name().to_str().unwrap_or("").ends_with(".ts")) {
                    println!("YES: {}", entry.path().display())
                } else {
                    println!("NOPE: {}", entry.path().display())
                }
            }
            Err(err) => println!("ERROR: {}", err),
            _ => println!("Not a TS file"),
        }
    }
}

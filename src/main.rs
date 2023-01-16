use clap::ArgAction;
use clap::Parser;

/// A program to update project imports when a file is renamed
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Source file path
    #[arg(index = 1, value_name = "SOURCE_PATH")]
    src: String,

    /// Destination file path
    #[arg(index = 2, value_name = "DESTINATION_PATH")]
    dst: String,

    /// Project root path
    #[arg(value_name = "PROJECT_ROOT", short, long)]
    project_root: String,

    /// Whether or not to apply file renaming
    #[arg(short, long, action=ArgAction::SetTrue)]
    apply_rename: bool,
}

fn main() {
    // [TODO]: Try to find the project root? - 2023-01-15 10:23pm
    let args = Args::parse();

    println!(
        "{}: {} --> {} ... {}",
        args.project_root, args.src, args.dst, args.apply_rename
    )
}

// use std::{env, process};

// use minigrep::Config;

// fn main() {
//     let config = Config::build(env::args()).unwrap_or_else(|err| {
//         eprintln!("Problem parsing arguments: {err}");
//         process::exit(1);
//     });

//     println!("Searching for {}", config.query);
//     println!("In file {}", config.file_path);
//     println!("-----------------------");
//     if let Err(e) = minigrep::run(config) {
//         eprintln!("Application error: {e}");
//         process::exit(1);
//     };
// }

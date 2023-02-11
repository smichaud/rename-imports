use clap::ArgAction;
pub use clap::Parser;

/// A program to update project imports when a file is renamed
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct RenameImportsArgs {
    /// Source file path
    #[arg(index = 1, value_name = "SOURCE_PATH")]
    pub src: String,

    /// Destination file path
    #[arg(index = 2, value_name = "DESTINATION_PATH")]
    pub dst: String,

    /// Project root path
    #[arg(value_name = "PROJECT_ROOT", short, long)]
    pub project_root: String,

    /// Whether or not to apply file renaming
    #[arg(short, long, action=ArgAction::SetTrue)]
    pub apply_rename: bool,
}

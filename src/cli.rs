use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, about, long_about = None, disable_help_flag = true)]

/// Template for cli args. See `clap` documentation for move information
pub struct CliArgs {
    /// Show help message
    #[arg(short = 'h', long = "help")]
    pub help: bool,

    /// Pattern to match files
    pub pattern: Option<String>,

    /// New name format
    pub rename_template: Option<String>,

    /// Force overwriting of existing files
    #[arg(short = 'f', long = "force")]
    pub force_overwrite: bool,
}

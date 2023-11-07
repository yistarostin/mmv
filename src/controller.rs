use std::{env, process::exit};

use crate::cli::CliArgs;
use crate::filesystem::FileSystemManager;
use crate::matcher::Matcher;
use clap::{CommandFactory, Parser};
use clap_help::Printer;
use colored::Colorize;

/// Controller class is a simple class for running the entire `mmv` utility with 1 line from main.rs.
/// It is used for convinient error handling and integration testing
pub struct MassMoveController {}
impl MassMoveController {
    pub fn new() -> Self {
        MassMoveController {}
    }
    pub fn run(&self) {
        let arguments = CliArgs::parse();
        if arguments.help {
            static INTRO: &str = "
            Mass move files with 1 command using simple pattern-matching technique.
            * Pattern must be a string with some `*` (star) symbols. Each of this characters will be replaced with a substings to make pattern a name of to be moved
            * Template is a string with some `#` symbols followed by natural numbers. Is some file matched pattern, each of `#` things will be replaced with corresponding pattern substings to construct a new filename
            * If you want `mmv` to overwrite existing files, you should provide a `-f` flag, otherwise `mmv` will do nothing.

            Here are a few examples on how `mmv` can be used:

            * `mmv prefix*suffix new_prefix#1new_suffix` will move all files with `prefix` and `suffix` to same names, but with `new_prefix` and `new_suffix`.
            * `mmv old new` will behave just as an usual `mv` command and rename a single file if such exists.
            * `mmv a*c* a#1c` will trunk suffixes of all files matching `a*c*` template (meaning such files should start with `a` letter and containg `c` further).
            * `mmv 1*3 a#1`. If current directory contains files like 123, 1113, 13, 143, they will be renamed to a2, a11, a, a4 respectively

            Note that `mmv` may operate files with in all subdirectories of current directory, but **does not** work with absolute pathes. Sometimes `mmv` may act correctly with absolute pathes, but use it on your own risk as it may break some important operating system files you user has such permission. Also `mmv` does not support windows-style pathes (e.g. C:\\ \\User), so please use only *relative unix-style pathes*.

            ";

            Printer::new(CliArgs::command())
                .with("introduction", INTRO)
                .without("author")
                .print_help();
            return;
        }
        if arguments.pattern.is_none() {
            eprintln!("{}", "Pattern not provided. Please run mmv command with pattern as a first positional argument. See --help for documentation".red());
            exit(exitcode::DATAERR);
        }
        if arguments.rename_template.is_none() {
            eprintln!("{}", "Rename template not provided. Please run mmv command with rename template as a second positional argument. See --help for documentation".red());
            exit(exitcode::DATAERR);
        }
        let filesystem_manager =
            FileSystemManager::new(env::current_dir().ok(), arguments.force_overwrite);
        let filenames = filesystem_manager.get_filenames();
        let pattern = filesystem_manager.normalize_path(&arguments.pattern.unwrap());
        let rename_template =
            filesystem_manager.normalize_path(&arguments.rename_template.unwrap());
        let matcher = Matcher::new(pattern.clone());
        let mut moved_any = false;
        for file in &filenames {
            let normalized_path = file;
            if let Some(changes) = matcher.pattern_matcher(&normalized_path) {
                let new_name = matcher.fill_in_template(
                    changes.iter().map(|x| x.as_ref()).collect(),
                    &rename_template,
                );
                let file_move_result = filesystem_manager.move_file(file, &new_name.to_string());
                if let Err(err) = file_move_result {
                    eprintln!(
                        "{} {} {}\n{}",
                        "Could not move file".red(),
                        file.red(),
                        "because of the following reason:".red(),
                        err.to_string().red()
                    );
                    return;
                } else {
                    moved_any = true;
                }
            }
        }
        if !moved_any {
            eprintln!("No files for pattern {} can be moved!", pattern.red());
            exit(exitcode::DATAERR);
        }
    }
}

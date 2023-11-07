extern crate exitcode;

use colored::Colorize;
use path_clean::clean;
use std::env;
use std::path::Path;
use std::path::PathBuf;
use walkdir::{DirEntry, WalkDir};
/// FSUTils struct is a simple wrap around std::fs module.
/// It wraps up several std::fs methods like `std::fs::rename` and `std::fs::read_dir` and handles results of these methods
pub struct FileSystemManager {
    pub current_dir: PathBuf,
    force_overwrite: bool,
}
impl FileSystemManager {
    /// Creates new FSUtils instance.
    /// # Arguments
    ///
    /// * `dir` - filesystem path, whcih would be an entry point for communation with file system. In case `dir` is `None`, `FSUTils` simply uses `env::current_dir()` as directory
    /// * `force_overwrite` - Specifies if `mmv` should overwrite existing files. In case user does not specify `-f / --force`, `FSUtils` will cause an error insteaf of overwriting excistant file
    pub fn new(dir: Option<PathBuf>, force_overwrite: bool) -> Self {
        FileSystemManager {
            current_dir: dir.unwrap_or(env::current_dir().unwrap()),
            force_overwrite,
        }
    }

    /// Searches filesystem and returns all the files, which might be renamed with `mmv` utility
    /// # Returns
    ///
    /// * `Vec<String>` of movable files
    ///

    fn is_hidden(entry: &DirEntry) -> bool {
        entry
            .file_name()
            .to_str()
            .map(|s| s.starts_with("."))
            .unwrap_or(false)
    }

    /// Recursivly searches file in `self.current_dir` directory. Ignores hidden files.
    ///
    /// # Returns
    /// * `Vec<String>` - relative pathes to all the files in `self.current` directory and it's subdirectories
    pub fn get_filenames(&self) -> Vec<String> {
        let walker = WalkDir::new(&self.current_dir).into_iter();
        let mut result: Vec<String> = vec![];
        for entry in walker.filter_entry(|e| !Self::is_hidden(e)) {
            if entry.is_ok() {
                result.push(entry.unwrap().path().to_str().unwrap().to_string());
            }
        }
        result = result
            .into_iter()
            .filter(|entry| std::fs::metadata(entry).unwrap().is_file())
            .map(|file| {
                file.strip_prefix(self.current_dir.to_str().unwrap())
                    .unwrap()[1..]
                    .to_string()
            })
            .collect();
        result
    }

    /// Moves file located at `old_name` to the new location `new_name`. If the `self.force_overwrite` is set `True`, will overwrite excestant files. Otherwise returns `Err` after the attempt to overwrite a file
    ///
    /// # Arguments
    ///
    /// * `old_name` - current file location
    /// * `new_name` - desired file location
    ///
    /// # Returns
    ///
    /// * 'Result<(), std::io::Error>` containg either nothing or the error arised while file move
    pub fn move_file(&self, old_name: &String, new_name: &String) -> Result<(), std::io::Error> {
        if Path::new(new_name).exists() && !self.force_overwrite {
            //eprintln!("mmv: Not able to replace existing file: {}", new_name);
            return Err(std::io::Error::new(
                std::io::ErrorKind::AlreadyExists,
                "mmv: Not able to replace existing file: {}, new_name)",
            ));
        }
        let move_result = std::fs::rename(old_name, new_name);
        println!("Moving file: {} -> {}", old_name.red(), new_name.green());
        move_result
    }

    /// Converts file path so a simplified form. E.g. `../tmp/test/abc` becomes just `test/abs` in case `self.current_dir` ends with `tmp`.
    pub fn normalize_path(&self, file: &str) -> String {
        clean(Path::new(&self.current_dir).join(file))
            .to_str()
            .unwrap()
            .to_string()
            .strip_prefix(self.current_dir.to_str().unwrap())
            .unwrap()[1..]
            .to_string()
    }
}

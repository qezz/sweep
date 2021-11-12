use std::path::{Path, PathBuf};

use crate::output;
use crate::swpfile::parse_swpfile;

/// Describes a discovered cleanable project
#[derive(Debug)]
pub struct Project {
    /// The root directory of the project
    root: PathBuf,

    /// Directories containing dependencies
    dependency_dirs: Vec<PathBuf>,

    /// Timestamp indicating when the project was last modified
    last_modified: u64,

    typ: Option<String>,
}

impl Project {
    /// Initialises a new project
    ///
    /// # Arguments
    /// `root` - The root directory of the project
    pub fn new<P: Into<PathBuf>>(root: P) -> Project {
        Project {
            root: root.into(),
            dependency_dirs: Vec::new(),
            last_modified: 0,
            typ: None,
        }
    }

    pub fn with_type(&mut self, typ: String) {
        self.typ = Some(typ.clone());
    }

    pub fn typ(&self) -> String {
        self.typ.clone().unwrap_or("".to_string())
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Marks a subdirectory of this project's root directory as cleanable,
    /// if that directory exists. If the subdirectory doesn't exist, nothing
    /// happens.
    ///
    /// # Arguments
    /// `subdir` - Name of the subdirectory inside the project root directory
    pub fn add_cleanable_dir_if_exists<P: Into<PathBuf>>(&mut self, subdir: P) {
        let mut path = self.root.clone();
        path.push(subdir.into());

        if path.exists() && path.is_dir() && !self.dependency_dirs.contains(&path) {
            self.dependency_dirs.push(path);
        }
    }

    pub fn load_swpfile(&mut self, filename: &str) {
        let paths = match parse_swpfile(&self.root, &PathBuf::from(filename)) {
            Ok(paths) => paths,
            Err(e) => {
                output::error(format!(
                    "Could not read .swpfile file in {}",
                    self.root.to_str().unwrap_or("")
                ));
                output::println_info(e.to_string());
                std::process::exit(1);
            }
        };

        for path in paths {
            self.add_cleanable_dir_if_exists(path);
        }
    }

    /// Checks if the given path is listed as a cleanable directory of this
    /// project
    pub fn is_cleanable_dir<P: Into<PathBuf>>(&self, path: P) -> bool {
        self.dependency_dirs.contains(&path.into())
    }

    /// Consumes the project and returns the dependency directories
    pub fn into_cleanable_dirs(self) -> Vec<PathBuf> {
        self.dependency_dirs
    }
}

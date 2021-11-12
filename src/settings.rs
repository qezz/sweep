use std::path::{Path, PathBuf};

use dunce::canonicalize;
use regex::Regex;
use structopt::StructOpt;

pub enum ArgsError {
    InvalidPath(PathBuf),
}

pub type Result<T> = std::result::Result<T, ArgsError>;

/// Deletes unnecessary build artifacts and dependency directories in your projects.
///
/// Detects Rust, Java and NodeJS projects by default, or define your own cleanable directories by adding a `.cleanuprc` file to your project directory.
///
/// Questions, bugs & other issues: https://github.com/woubuc/sweep/issues
#[derive(Clone, Debug, StructOpt)]
pub struct Args {
    /// One or more directories where `swp` should start searching for projects.
    /// Defaults to the current working directory if no paths are given.
    #[structopt(name = "PATH...")]
    pub paths: Vec<PathBuf>,

    /// Config
    #[structopt(long = "config")]
    pub config: PathBuf,

    /// Sweep even projects that were modified within the last 30 days.
    #[structopt(short = "a", long = "all")]
    pub all: bool,

    /// Exclude projects in directories matched by this regex pattern.
    #[structopt(short = "i", long = "ignore")]
    pub ignore: Option<Regex>,

    /// Skip confirmation prompt before removing directories. Use at your own risk.
    #[structopt(short = "f", long = "force")]
    pub force: bool,
}

impl Args {
    /// Gets a Settings struct from the CLI arguments
    pub fn get() -> Result<Args> {
        let mut settings: Args = Args::from_args();

        settings.validate()?;

        Ok(settings)
    }

    /// Validates the application-specific values in a settings struct.
    ///
    /// This method is called automatically when calling `.get()`, but it
    /// should be called manually when creating a custom settings object.
    pub fn validate(&mut self) -> Result<()> {
        // If no paths are set, add the current path
        if self.paths.is_empty() {
            self.paths.push(".".into());
        }

        if !self.config.exists() {
            panic!("config file does not exist");
        }

        // Resolve to absolute paths
        self.paths = {
            let paths: Result<Vec<PathBuf>> = self
                .paths
                .iter()
                .map(|p| canonicalize(p).map_err(|_| ArgsError::InvalidPath(p.clone())))
                .collect();

            paths?
        };

        Ok(())
    }

    /// Checks if a given path is ignored
    ///
    /// # Arguments
    /// * `ignore` - The ignore regex, if set
    /// * `path`   - Path to check against the ignore regex
    ///
    /// # Returns
    /// * `true`  - If the path matches the regex
    /// * `false` - If the regex and path don't match, if no ignore
    ///             regex was given, or if the path is empty
    pub fn is_path_ignored(&self, path: &Path) -> bool {
        if self.ignore.is_none() {
            return false;
        }

        let re = self.ignore.as_ref().unwrap();
        let path = path.to_str().unwrap_or("");

        if path.len() == 0 {
            return false;
        } else {
            return re.is_match(path);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_settings() {
        let mut settings = Args {
            paths: vec![],
            all: false,
            ignore: None,
            force: false,
            config: "sample-config.yml".into(),
        };

        assert!(
            settings.validate().is_ok(),
            "An error occured while validating settings struct"
        );
        assert!(settings.paths.len() > 0, "Settings contains no paths");
    }

    #[test]
    fn invalid_path() {
        let mut settings = Args {
            paths: vec!["./this_path_does_not_exist_1".into()],
            all: false,
            ignore: None,
            force: false,
            config: "sample-config.yml".into(),
        };

        let validate = settings.validate();
        assert!(
            validate.is_err(),
            "No error occured while validating invalid settings struct"
        );

        match validate.unwrap_err() {
            ArgsError::InvalidPath(_) => (),
            _ => panic!("Unexpected error returned"),
        }
    }

    #[test]
    fn ignore_flag() {
        let settings = Args {
            paths: vec![],
            all: false,
            ignore: Some(Regex::new("src").unwrap()),
            force: false,
            config: "sample-config.yml".into(),
        };

        assert_eq!(settings.is_path_ignored(Path::new("./src")), true);
        assert_eq!(settings.is_path_ignored(Path::new("./foo")), false);
    }
}

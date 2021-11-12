use std::path::Path;

use crate::config::Config;
use crate::utils::file_utils::exists_in_path;
use crate::Project;

/// Checks if a given directory is cleanable and identifies the
/// dependency subdirectories
///
/// # Arguments
/// `path` - The path to check
///
/// # Returns
/// The identified project, or None if the given path is not a project
pub fn detect_cleanable_project(path: &Path, config: &Config) -> Option<Project> {
    // A project can only be a directory
    if !path.is_dir() {
        return None;
    }

    // Create an empty project so we can add cleanable directories to it
    let mut project = Project::new(path.clone());

    // This flag will keep track of whether we've found a project
    let mut is_project = false;

    for filename in [".swpfile", ".cleanuprc"].iter() {
        if exists_in_path(path, filename) {
            project.with_type("sweep-file".into());
            project.load_swpfile(filename);

            // If a .swpfile file is found, it overrides the default paths so we can return early
            return Some(project);
        }
    }

    // Check config entries
    for entry in &config.entries {
        if exists_in_path(path, &entry.trigger) {
            is_project = true;
            project.with_type(entry.name.clone());

            for d in &entry.disposables {
                project.add_cleanable_dir_if_exists(d);
            }
        }
    }

    if is_project {
        return Some(project);
    } else {
        return None;
    }
}

#[cfg(test)]
mod test {
    use super::detect_cleanable_project;
    use crate::utils::test_utils;

    use crate::config::{Config, Entry};

    fn simple_config() -> Config {
        Config {
            entries: vec![
                Entry {
                    name: "rust".into(),
                    trigger: "Cargo.toml".into(),
                    disposables: ["target"].iter().map(|&s| s.into()).collect::<Vec<String>>(),
                },
                Entry {
                    name: "npm".into(),
                    trigger: "package.json".into(),
                    disposables: ["node_modules", ".cache", "build", "dist"]
                        .iter().map(|&s| s.into()).collect::<Vec<String>>(),
                },
                Entry {
                    name: "java/pom".into(),
                    trigger: "pom.xml".into(),
                    disposables: ["target", ".gradle", "build"]
                        .iter().map(|&s| s.into()).collect::<Vec<String>>(),
                }
            ],
        }
    }

    /// Creates the provided files and directories in a temporary directory,
    /// then runs `detect_cleanable_project` on that directory and verifies
    /// that all cleanable directories have been identified.
    ///
    /// # Example
    /// ```rs
    /// test_project!(
    ///   files: ["Cargo.toml"], // The 'cargo.toml' file will be created
    ///   dirs: ["src", "target"], // The 'src' and 'target' directories will be created
    ///   cleanable: ["target"], // The 'target' directory should be identified as cleanable
    /// );
    ///
    /// // No cleanable directories
    /// test_project!(
    ///   files: ["Cargo.toml"],
    ///   dirs: ["src"]
    /// );
    /// ```
    macro_rules! test_project {
		(files: [$($f:expr),*], dirs: [$($d:expr),*]) => {
			test_project!(files: [$($f),*], dirs: [$($d),*], cleanable: []);
		};

		(files: [$($f:expr),*], dirs: [$($d:expr),*], cleanable: [$($c:expr),*]) => {
			test_utils::with_temp_dir(|dir| {
				$(test_utils::create_dir(dir, $d);)*
				$(test_utils::create_file(dir, $f);)*

                                let config = simple_config();
				let project = detect_cleanable_project(&dir, &config).expect("No project detected");
				$(assert!(project.is_cleanable_dir(&dir.join($c)));)*

				assert_eq!(project.into_cleanable_dirs().len(), {
					#[allow(unused_mut)]
					let mut i = 0;
					$(i += 1; $c;)*
					i
				});
			});
		};
	}

    #[test]
    fn rust() {
        test_project!(
            files: ["Cargo.toml"],
            dirs: ["src"]
        );

        test_project!(
            files: ["Cargo.toml"],
            dirs: ["src", "target"],
            cleanable: ["target"]
        );
    }

    #[test]
    fn nodejs() {
        test_project!(
            files: ["package.json"],
            dirs: ["src"]
        );

        test_project!(
            files: ["package.json"],
            dirs: ["src", "node_modules", ".cache", ".idea", "build", "dist"],
            cleanable: ["node_modules", ".cache", "build", "dist"]
        );
    }

    #[test]
    fn java() {
        test_project!(
            files: ["pom.xml"],
            dirs: ["src"]
        );

        test_project!(
            files: ["pom.xml"],
            dirs: ["src", "build"],
            cleanable: ["build"]
        );

        test_project!(
            files: ["pom.xml"],
            dirs: ["src", ".gradle", "build", "spec"],
            cleanable: [".gradle", "build"]
        );
    }

    #[test]
    fn empty_dir() {
        let config = simple_config();
        test_utils::with_temp_dir(|dir| {
            assert!(
                detect_cleanable_project(&dir, &config).is_none(),
                "Project detected in empty directory"
            );
        });
    }

    #[test]
    fn no_project() {
        let config = simple_config();
        test_utils::with_temp_dir(|dir| {
            test_utils::create_dir(dir, "not_a_project");
            test_utils::create_dir(dir, "another_test_directory");
            test_utils::create_file(dir, "no_project_here.txt");

            assert!(
                detect_cleanable_project(&dir, &config).is_none(),
                "Project detected in unrelated directory"
            );
        });
    }
}

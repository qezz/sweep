use std::path::PathBuf;

use crossbeam::queue::SegQueue;
use yansi::Color;

use crate::output;
use crate::Project;
use crate::Args;

use super::filter_by_modified_date::filter_by_modified_date;

/// Analyses a queue of projects loaded from `discover_projects()`
///
/// # Arguments
/// `projects` - The discovered projects
/// `settings` - The application settings struct
///
/// # Returns
/// All discovered cleanable directories
pub fn analyse_projects(projects: SegQueue<Project>, settings: &Args) -> Vec<PathBuf> {
    let filtered = if settings.all {
        output::println(
            "Skip",
            Color::Yellow,
            "--all flag set, ignoring last used time",
        );
        projects
    } else {
        filter_by_modified_date(projects)
    };

    if filtered.len() == 0 {
        return Vec::new();
    }

    let mut dirs = Vec::new();
    while let Ok(project) = filtered.pop() {
        output::println("ProjectType:", Color::Cyan, &project.typ());
        dirs.append(&mut project.into_cleanable_dirs());
    }

    dirs.sort();
    return dirs;
}

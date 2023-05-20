#![allow(clippy::missing_const_for_fn)]
use eyre::{bail, Result};
use git2::{BranchType, Repository, StatusOptions};
use ignore::{WalkBuilder, WalkState};
use indicatif::ProgressBar;
use serde_derive::Serialize;
use std::{
    path::Path,
    sync::{Arc, Mutex},
    time::Duration,
    vec,
};
use tabled::{object::Columns, Modify, Width};

#[derive(Default, Serialize, Debug)]
pub struct RepoAnalysis {
    pub current_branch: Option<String>,
    pub commits: Option<Changes>,
    pub branches: Option<BranchesState>,
}
#[derive(Default, Serialize, Debug)]
pub struct Changes {
    pub modified: usize,
    pub new: usize,
    pub deleted: usize,
}

impl Changes {
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.total() == 0
    }
    #[must_use]
    pub fn total(&self) -> usize {
        self.modified + self.new + self.deleted
    }
}
#[derive(Default, Serialize, Debug)]
pub struct BranchesState {
    pub ahead: Vec<String>,
    pub missing: Vec<String>,
}
impl BranchesState {
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.total() == 0
    }
    #[must_use]
    pub fn total(&self) -> usize {
        self.ahead.len() + self.missing.len()
    }
}

fn sum_changes(statuses: &git2::Statuses<'_>) -> Changes {
    let mut changes = Changes::default();

    statuses
        .iter()
        .filter(|e| e.status() != git2::Status::CURRENT)
        .for_each(|e| {
            let s = e.status();
            if s.contains(git2::Status::INDEX_NEW) || s.contains(git2::Status::WT_NEW) {
                changes.new += 1;
            }
            if s.contains(git2::Status::INDEX_MODIFIED)
                || s.contains(git2::Status::WT_MODIFIED)
                || s.contains(git2::Status::INDEX_RENAMED)
                || s.contains(git2::Status::INDEX_TYPECHANGE)
                || s.contains(git2::Status::WT_RENAMED)
                || s.contains(git2::Status::WT_TYPECHANGE)
            {
                changes.modified += 1;
            }
            if s.contains(git2::Status::INDEX_DELETED) || s.contains(git2::Status::WT_DELETED) {
                changes.deleted += 1;
            }
        });
    changes
}

#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::cast_precision_loss)]
fn changes_sparkline(changes: &Changes) -> String {
    use owo_colors::OwoColorize;
    let c = "•";
    let line = format!(
        "{}{}{}",
        c.repeat(changes.modified).yellow(),
        c.repeat(changes.new).green(),
        c.repeat(changes.deleted).red(),
    );
    line
}

fn find_non_pushed(repo: &Repository) -> Result<(Vec<String>, Vec<String>)> {
    let local_branches = match repo.branches(Some(BranchType::Local)) {
        Ok(branches) => branches.filter_map(Result::ok).map(|(branch, _)| branch),
        Err(err) => bail!("error getting local branch: {}", err),
    };

    let mut nonpushed_commits = vec![];
    let mut nonpushed_branches = vec![];
    for branch in local_branches {
        if let Ok(remote_branch) = branch.upstream() {
            let last_local_commit = branch.get().peel_to_commit()?;
            let last_remote_commit = remote_branch.get().peel_to_commit()?;

            if repo.graph_descendant_of(last_local_commit.id(), last_remote_commit.id())? {
                if let Ok(Some(name)) = branch.name() {
                    nonpushed_commits.push(name.to_string());
                }
            }
        } else if let Ok(Some(name)) = branch.name() {
            nonpushed_branches.push(name.to_string());
        }
    }

    Ok((nonpushed_commits, nonpushed_branches))
}

#[must_use]
pub fn analyze_path(path: &Path, include_branches: bool) -> Option<RepoAnalysis> {
    if path.exists() {
        if let Ok(repo) = Repository::open(path) {
            if !repo.is_bare() {
                let mut opts = StatusOptions::new();
                opts.include_ignored(false);
                opts.exclude_submodules(true);
                opts.include_untracked(true);

                let statuses = repo.statuses(Some(&mut opts)).expect("cannot stat repo");

                let changes = sum_changes(&statuses);
                let current_branch = repo
                    .head()
                    .ok()
                    .and_then(|h| h.shorthand().map(|s| s.to_string()));

                if include_branches {
                    let (ahead, missing) =
                        find_non_pushed(&repo).map_or_else(|_| (vec![], vec![]), |state| state);
                    if !changes.is_empty() || !ahead.is_empty() || !missing.is_empty() {
                        return Some(RepoAnalysis {
                            current_branch,
                            commits: Some(changes),
                            branches: Some(BranchesState { ahead, missing }),
                        });
                    }
                } else if !changes.is_empty() {
                    return Some(RepoAnalysis {
                        current_branch,
                        commits: Some(changes),
                        branches: None,
                    });
                }
            }
        }
    }
    None
}
/// Run over folders
///
///
/// # Errors
///
/// This function will return an error if IO or repository handling fails
pub fn run(path: &str, include_branches: bool) -> Result<bool> {
    let mb = Arc::new(Mutex::new(vec![]));

    let spin = ProgressBar::new_spinner();
    spin.enable_steady_tick(Duration::from_millis(100));
    WalkBuilder::new(path)
        .threads(num_cpus::get() * 2)
        .hidden(false)
        .max_depth(Some(4))
        .filter_entry(|e| e.file_type().map_or(false, |ft| ft.is_dir()))
        .build_parallel()
        .run(|| {
            let mb = mb.clone();
            Box::new(move |result| {
                if let Ok(result) = result {
                    let path = result.path();
                    if path.exists() {
                        if let Ok(repo) = Repository::open(path) {
                            if !repo.is_bare() {
                                let mut opts = StatusOptions::new();
                                opts.include_ignored(false);
                                opts.exclude_submodules(true);
                                opts.include_untracked(true);

                                let statuses =
                                    repo.statuses(Some(&mut opts)).expect("cannot stat repo");

                                let changes = sum_changes(&statuses);
                                if include_branches {
                                    let (ahead, missing) = find_non_pushed(&repo)
                                        .map_or_else(|_| (vec![], vec![]), |state| state);
                                    if !changes.is_empty()
                                        || !ahead.is_empty()
                                        || !missing.is_empty()
                                    {
                                        let mut arr = mb.lock().expect("cannot take lock");

                                        // header
                                        if arr.is_empty() {
                                            arr.push(vec![
                                                "repository".to_string(),
                                                "changes".to_string(),
                                                "ahead".to_string(),
                                                "missing".to_string(),
                                            ]);
                                        }
                                        arr.push(vec![
                                            path.to_str().unwrap_or_default().to_string(),
                                            changes_sparkline(&changes),
                                            ahead.join(", "),
                                            missing.join(", "),
                                        ]);
                                    }
                                } else if !changes.is_empty() {
                                    let mut arr = mb.lock().expect("cannot take lock");

                                    // header
                                    if arr.is_empty() {
                                        arr.push(vec![
                                            "repository".to_string(),
                                            "changes".to_string(),
                                        ]);
                                    }
                                    arr.push(vec![
                                        path.to_str().unwrap_or_default().to_string(),
                                        changes_sparkline(&changes),
                                    ]);
                                }
                            }
                        }
                        return WalkState::Skip;
                    }
                }
                ignore::WalkState::Continue
            })
        });

    spin.finish_and_clear();

    let rows = mb.lock().expect("cannot take lock").clone();
    if !rows.is_empty() {
        let mut builder = tabled::builder::Builder::default();
        for row in &rows {
            builder.add_record(row);
        }
        let mut table = builder.build();
        table.with(tabled::Style::modern());
        table.with(Modify::new(Columns::first()).with(Width::wrap(70)));
        table.with(Modify::new(Columns::new(1..)).with(Width::wrap(20).keep_words()));
        let report = format!("{table}\n");
        print!("{report}");
        println!("total: {}", rows.len() - 1);
    }
    Ok(true)
}

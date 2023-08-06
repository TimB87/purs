use clap::{Arg, ArgAction, ArgMatches, Command};
use colored::Colorize;
use git2::{self, Repository, StatusOptions};
use itertools::join;
use std::env;
use std::path::PathBuf;

fn tico(path: &str, home_dir: Option<&str>) -> String {
    let tico = match home_dir {
        Some(dir) => path.replace(dir, "~"),
        None => path.to_owned(),
    };

    let sections = tico.chars().filter(|&x| x == '/').count();
    let mut shortened = String::with_capacity(tico.len());

    let mut count = 0;
    let mut skip_char = false;

    for c in tico.chars() {
        match c {
            '~' => {
                if !skip_char {
                    shortened.push(c);
                }
            }
            '.' => {
                skip_char = false;
                shortened.push(c);
            }
            '/' => {
                skip_char = false;
                count += 1;
                shortened.push(c);
            }
            _ => {
                if skip_char && count < sections {
                    continue;
                } else {
                    skip_char = true;
                    shortened.push(c);
                }
            }
        }
    }

    shortened
}

fn shorten_path(cwd: &str) -> String {
    let home_dir = env::var("HOME").ok();

    let friendly_path = match home_dir {
        Some(home) => cwd.replace(&home, "~"),
        None => cwd.to_string(),
    };

    tico(&friendly_path, Option::None)
}

fn repo_status(r: &Repository, detailed: bool) -> Option<String> {
    let (ahead, behind) = if detailed {
        get_ahead_behind(r)?
    } else {
        (0, 0)
    };

    let (index_change, wt_change, conflicted, untracked) = count_files_statuses(r)?;

    let mut out = vec![];

    if let Some(name) = get_head_shortname(r) {
        out.push(name.cyan());
    }

    if ahead > 0 {
        out.push(format!("↑{}", ahead).cyan());
    }

    if behind > 0 {
        out.push(format!("↓{}", behind).cyan());
    }

    if index_change == 0 && wt_change == 0 && conflicted == 0 && untracked == 0 {
        out.push("✔".green());
    } else {
        if index_change > 0 {
            out.push(format!("♦ {}", index_change).green());
        }
        if conflicted > 0 {
            out.push(format!("✖ {}", conflicted).red());
        }
        if wt_change > 0 {
            out.push(format!("✚ {}", wt_change).bright_yellow());
        }
        if untracked > 0 {
            out.push("…".bright_yellow());
        }
    }

    if let Some(action) = get_action(r) {
        out.push(format!(" {}", action).purple());
    }

    Some(join(out.iter(), " "))
}

fn get_ahead_behind(r: &Repository) -> Option<(usize, usize)> {
    let head = r.head().ok()?;
    if !head.is_branch() {
        return None;
    }

    let head_name = head.shorthand()?;
    let head_branch = r.find_branch(head_name, git2::BranchType::Local).ok()?;
    if let Ok(upstream) = head_branch.upstream() {
        let head_oid = head.target()?;
        let upstream_oid = upstream.get().target()?;
        return r.graph_ahead_behind(head_oid, upstream_oid).ok();
    }

    None
}

fn get_head_shortname(r: &Repository) -> Option<String> {
    let head = r.head().ok()?;

    if let Some(shorthand) = head.shorthand() {
        if shorthand != "HEAD" {
            return Some(shorthand.to_string());
        }
    }

    Some(format!(":{}", head.target().unwrap()))
}

fn count_files_statuses(r: &Repository) -> Option<(usize, usize, usize, usize)> {
    let mut opts = StatusOptions::new();
    opts.include_untracked(true);

    let statuses = r.statuses(Some(&mut opts)).ok()?;

    let index_change = statuses
        .iter()
        .filter(|entry| {
            entry.status().intersects(
                git2::Status::INDEX_NEW
                    | git2::Status::INDEX_MODIFIED
                    | git2::Status::INDEX_DELETED
                    | git2::Status::INDEX_RENAMED
                    | git2::Status::INDEX_TYPECHANGE,
            )
        })
        .count();

    let wt_change = statuses
        .iter()
        .filter(|entry| {
            entry.status().intersects(
                git2::Status::WT_MODIFIED
                    | git2::Status::WT_DELETED
                    | git2::Status::WT_TYPECHANGE
                    | git2::Status::WT_RENAMED,
            )
        })
        .count();

    let conflicted = statuses
        .iter()
        .filter(|entry| entry.status().contains(git2::Status::CONFLICTED))
        .count();

    let untracked = statuses
        .iter()
        .filter(|entry| entry.status().contains(git2::Status::WT_NEW))
        .count();

    Some((index_change, wt_change, conflicted, untracked))
}

// Based on https://github.com/zsh-users/zsh/blob/ed4e37e45c2f5761981cdc6027a5d6abc753176a/Functions/VCS_Info/Backends/VCS_INFO_get_data_git#L11
fn check_file_exists(paths: &[PathBuf]) -> Option<String> {
    paths
        .iter()
        .find(|path| path.exists())
        .map(|path| path.file_name().unwrap().to_string_lossy().to_string())
}

fn get_action(r: &Repository) -> Option<String> {
    let gitdir = r.path();
    let mut tmp_paths = Vec::new();

    // List of paths to check for existence
    tmp_paths.push(gitdir.join("rebase-apply"));
    tmp_paths.push(gitdir.join("rebase"));
    tmp_paths.push(gitdir.join("..").join(".dotest"));

    if check_file_exists(
        &tmp_paths
            .iter()
            .map(|p| p.join("rebasing"))
            .collect::<Vec<_>>(),
    )
    .is_some()
    {
        return Some("rebase".to_string());
    }
    if check_file_exists(
        &tmp_paths
            .iter()
            .map(|p| p.join("applying"))
            .collect::<Vec<_>>(),
    )
    .is_some()
    {
        return Some("am".to_string());
    }
    if check_file_exists(&tmp_paths).is_some() {
        return Some("am/rebase".to_string());
    }

    tmp_paths.clear();
    tmp_paths.push(gitdir.join("rebase-merge").join("interactive"));
    tmp_paths.push(gitdir.join(".dotest-merge").join("interactive"));
    if check_file_exists(&tmp_paths).is_some() {
        return Some("rebase-i".to_string());
    }

    tmp_paths.clear();
    tmp_paths.push(gitdir.join("rebase-merge"));
    tmp_paths.push(gitdir.join(".dotest-merge"));
    if check_file_exists(&tmp_paths).is_some() {
        return Some("rebase-m".to_string());
    }

    if gitdir.join("MERGE_HEAD").exists() {
        return Some("merge".to_string());
    }

    if gitdir.join("BISECT_LOG").exists() {
        return Some("bisect".to_string());
    }

    if gitdir.join("CHERRY_PICK_HEAD").exists() {
        if gitdir.join("sequencer").exists() {
            return Some("cherry-seq".to_string());
        } else {
            return Some("cherry".to_string());
        }
    }

    if gitdir.join("sequencer").exists() {
        return Some("cherry-or-revert".to_string());
    }

    None
}

pub fn display(sub_matches: &ArgMatches) {
    let my_path = env::current_dir().unwrap();
    let display_path = shorten_path(my_path.to_str().unwrap()).blue();

    let branch = match Repository::discover(my_path) {
        Ok(repo) => repo_status(&repo, sub_matches.get_flag("git-detailed")),
        Err(_e) => None,
    };
    let display_branch = branch.unwrap_or_default().cyan();

    if sub_matches.get_flag("newline") {
        println!();
    }
    println!("{display_path} {display_branch}");
}

pub fn cli_arguments() -> clap::Command {
    Command::new("precmd")
        .arg(
            Arg::new("git-detailed")
                .long("git-detailed")
                .help("Prints detailed git status")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("newline")
                .long("newline")
                .short('n')
                .help("Prints a blank line before the precmd")
                .action(ArgAction::SetTrue),
        )
}

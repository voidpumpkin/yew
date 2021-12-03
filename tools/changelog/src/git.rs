use crate::package::Package;
use crate::CliArgs;
use anyhow::{Context, Result};
use git2::Repository;

pub type Logs = Vec<(String, String, String)>;

pub struct CategorizedLogs {
    pub fixes: Logs,
    pub features: Logs,
}

pub fn open_repository() -> Repository {
    match Repository::open(".") {
        Err(err) => {
            eprintln!("Error: could not open repository: {}", err);
            std::process::exit(1);
        }
        Ok(repo) => repo,
    }
}

pub fn get_categorized_logs(cli_args: &mut CliArgs, package: Package) -> Result<CategorizedLogs> {
    let mut revwalk = cli_args.repo.revwalk()?;
    revwalk.set_sorting(git2::Sort::TOPOLOGICAL)?;
    let from_object = cli_args
        .repo
        .revparse_single(&cli_args.from)
        .context("Could not find `from` revision")?;
    let to_object = cli_args
        .repo
        .revparse_single(&cli_args.to)
        .context("Could not find `to` revision")?;
    revwalk.hide(from_object.id())?;
    revwalk.push(to_object.id())?;

    let mut logs = Vec::new();

    for oid in revwalk {
        let oid = oid?;
        let commit = cli_args.repo.find_commit(oid)?;
        let first_line = commit
            .message()
            .context("Invalid UTF-8 in commit message")?
            .lines()
            .next()
            .context("Missing commit message")?;
        let author = commit.author();
        let email = author.email().context("Missing author's email")?;

        if email.contains("dependabot") {
            continue;
        }

        let (issue, first_line) =
            if let Some(caps) = cli_args.re_issue.captures_iter(first_line).last() {
                let first_line_stripped = vec![
                    &first_line[..caps.get(0).unwrap().start()],
                    &first_line[caps.get(0).unwrap().end()..],
                ]
                .join("");
                (caps[1].to_string(), first_line_stripped)
            } else {
                eprintln!("Missing issue for commit: {}", oid);
                continue;
            };

        let user = cli_args
            .github_users
            .find_user_by_commit_author(email, oid.to_string())
            .with_context(|| format!("Could not find GitHub user for commit: {}", oid))?;

        let is_issue_for_this_package = cli_args
            .github_issue_labels
            .is_issue_for_this_package(issue.clone(), package.clone())
            .with_context(|| format!("Could not find GitHub issue: {}", issue))?;

        if !is_issue_for_this_package {
            continue;
        }

        logs.push((first_line.to_string(), user.to_owned(), issue.to_owned()));
    }
    let (fixes, features) = logs
        .into_iter()
        .partition(|(msg, _, _)| msg.to_lowercase().contains("fix"));

    Ok(CategorizedLogs { fixes, features })
}

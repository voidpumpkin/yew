mod git;
mod github_issue_labels;
mod github_users;
mod package;
mod run;
mod write_new_changelog;

use anyhow::Result;
use git::open_repository;
use github_issue_labels::GitHubIssueLabels;
use github_users::GitHubUsers;
use run::run;
use structopt::StructOpt;

fn main() -> Result<()> {
    let cli_args = CliArgs::from_args();
    run(cli_args)
}

#[derive(StructOpt)]
pub struct CliArgs {
    /// package to generate changelog for
    package: String,

    /// From commit.
    from: String,

    /// To commit.
    #[structopt(default_value = "HEAD")]
    to: String,

    #[structopt(skip = open_repository())]
    repo: git2::Repository,

    #[structopt(skip)]
    github_users: GitHubUsers,

    #[structopt(skip)]
    github_issue_labels: GitHubIssueLabels,

    #[structopt(skip = regex::Regex::new(r"\s*\(#(\d+)\)").unwrap())]
    re_issue: regex::Regex,
}

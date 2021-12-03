use anyhow::{bail, Result};
use serde::Deserialize;
use std::collections::HashMap;

use crate::package::Package;

#[derive(Deserialize, Debug)]
pub struct GitHubIssueLabelApi {
    name: String,
}

#[derive(Debug, Default)]
pub struct GitHubIssueLabels {
    cache: HashMap<String, Option<Vec<String>>>,
}

impl GitHubIssueLabels {
    pub fn is_issue_for_this_package(&mut self, issue: String, package: Package) -> Option<bool> {
        let labels = self
            .cache
            .entry(issue.clone())
            .or_insert_with(|| match Self::query_issue_labels(&issue) {
                Ok(value) => value,
                Err(err) => {
                    eprintln!("Error: {}", err);
                    None
                }
            })
            .as_deref()?;
        let package_labels = package.as_labels();

        Some(labels.iter().any(|label| package_labels.contains(label)))
    }

    fn query_issue_labels(q: &str) -> Result<Option<Vec<String>>> {
        std::thread::sleep(std::time::Duration::from_secs(1));
        let issue_labels = reqwest::blocking::Client::new();
        let resp = issue_labels
            .get(format!(
                "https://api.github.com/repos/yewstack/yew/issues/{}/labels",
                q,
            ))
            .header("user-agent", "reqwest")
            .header("accept", "application/vnd.github.v3+json")
            .send()?;
        let status = resp.status();
        if !status.is_success() {
            if let Some(remaining) = resp.headers().get("x-ratelimit-remaining") {
                if remaining == "0" {
                    bail!("GitHub API limit reached.");
                }
            }
            bail!("GitHub API request error: {}", status);
        }
        let body = resp.json::<Vec<GitHubIssueLabelApi>>()?;

        let label_names: Vec<String> = body.into_iter().map(|label| label.name).collect();

        Ok(Some(label_names))
    }
}

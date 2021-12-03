use anyhow::{bail, Result};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct GitHubCommitApi {
    author: GitHubCommitAuthorApi,
}

#[derive(Deserialize, Debug)]
pub struct GitHubCommitAuthorApi {
    login: String,
}

#[derive(Debug, Default)]
pub struct GitHubUsers {
    cache: HashMap<String, Option<String>>,
}

impl GitHubUsers {
    pub fn find_user_by_commit_author(
        &mut self,
        key: impl Into<String>,
        commit: impl AsRef<str>,
    ) -> Option<&str> {
        self.cache
            .entry(key.into())
            .or_insert_with(|| match Self::query_commit(commit) {
                Ok(value) => value,
                Err(err) => {
                    eprintln!("Error: {}", err);
                    None
                }
            })
            .as_deref()
    }

    fn query_commit(q: impl AsRef<str>) -> Result<Option<String>> {
        std::thread::sleep(std::time::Duration::from_secs(1));
        let client = reqwest::blocking::Client::new();
        let resp = client
            .get(format!(
                "https://api.github.com/repos/yewstack/yew/commits/{}",
                q.as_ref(),
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
        let body = resp.json::<GitHubCommitApi>()?;

        Ok(Some(body.author.login))
    }
}

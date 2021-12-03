use crate::git::CategorizedLogs;
use anyhow::Result;
use std::fs::File;
use std::io::Write;

pub fn write_new_changelog(file: &mut File, package: String, logs: CategorizedLogs) -> Result<()> {
    let CategorizedLogs { fixes, features } = logs;
    writeln!(
        file,
        "## ✨ {} **x.y.z** *({})*",
        package,
        chrono::Utc::now().format("%Y-%m-%d")
    )?;
    writeln!(file)?;
    writeln!(file, "#### Changelog")?;
    writeln!(file)?;

    writeln!(file, "- #### 🛠 Fixes")?;
    writeln!(file)?;
    for (msg, user, issue) in fixes {
        writeln!(
            file,
            "  - {msg}. [[@{user}](https://github.com/{user}), [#{issue}](https://github.com/yewstack/yew/pull/{issue})]",
            msg = msg,
            user = user,
            issue = issue
        )?;
    }
    writeln!(file)?;

    writeln!(file, "- #### ⚡️ Features")?;
    writeln!(file)?;
    for (msg, user, issue) in features {
        writeln!(
            file,
            "  - {msg}. [[@{user}](https://github.com/{user}), [#{issue}](https://github.com/yewstack/yew/pull/{issue})]",
            msg = msg,
            user = user,
            issue = issue
        )?;
    }

    writeln!(file)?;
    Ok(())
}

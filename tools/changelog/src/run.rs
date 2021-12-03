use crate::git::get_categorized_logs;
use crate::package::Package;
use crate::write_new_changelog::write_new_changelog;
use crate::CliArgs;
use anyhow::{Context, Result};
use std::convert::TryInto;
use std::fs;
use std::io;

pub fn run(mut cli_args: CliArgs) -> Result<()> {
    let package: Package = cli_args.package.as_str().try_into()?;

    let mut old_changelog =
        fs::File::open("CHANGELOG.md").context("could not open CHANGELOG.md for reading")?;
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("CHANGELOG.md.new")
        .context("could not open CHANGELOG.md.new for writing")?;

    let categorized_logs = get_categorized_logs(&mut cli_args, package)?;

    write_new_changelog(&mut file, cli_args.package, categorized_logs)?;

    io::copy(&mut old_changelog, &mut file)?;

    drop(old_changelog);
    drop(file);

    fs::remove_file("CHANGELOG.md").context("Could not delete CHANGELOG.md")?;
    fs::rename("CHANGELOG.md.new", "CHANGELOG.md")
        .context("Could not replace CHANGELOG.md with CHANGELOG.md.new")?;

    Ok(())
}

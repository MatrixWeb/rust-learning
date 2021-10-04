use anyhow::anyhow;
mod cli;
use structopt::StructOpt;
mod tasks;

use cli::{Aciton::*, CommandLineArgs};
use tasks::Task;

use std::path::PathBuf;


fn find_default_journal_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".rusty-journal.json");
        path
    })
}

fn main() -> anyhow::Result<()> {
    let CommandLineArgs {
        action,
        journal_file,
    } = CommandLineArgs::from_args();

    let journal_file = journal_file
    .or_else(find_default_journal_file)
    .ok_or(anyhow!("Failed to find journal file."))?;

    match action{
        Add { text } => tasks::add_task(journal_file, Task::new(text)),
        List => tasks::list_tasks(journal_file),
        Done { positon } => tasks::complete_task(journal_file, positon),
    }?;
    Ok(())
}

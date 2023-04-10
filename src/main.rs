mod cli;
mod tasks;

use anyhow::anyhow;
use structopt::StructOpt;
use cli::{Action::*, CommandLineArgs};
use tasks::Task;

fn main() -> anyhow::Result<()> {
	let CommandLineArgs {
		action, journal_file
	} = CommandLineArgs::from_args();

	let journal_file = journal_file.ok_or(anyhow!("Failed to found journal file"))?;

	match action {
		Add {text} => tasks::add_task(journal_file, Task::new(text)),
		List => tasks::list_tasks(journal_file),
		Done {position} => tasks::complete_task(journal_file, position)
	}?;

	Ok(())
}	

mod things;
mod reporter;

use reporter::{MarkdownReporter, Reporter};

use things::task::Task;
use anyhow::Result;

fn main() -> Result<()> {
    let today = Task::today()?;
    let reported: Vec<Task> = today.into_iter().filter(|task| task.has_tag("Report")).collect();
    println!("{}", MarkdownReporter.report(reported));

    Ok(())
}

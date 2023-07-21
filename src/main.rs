mod things;
mod reporter;

use reporter::{MarkdownReporter, Reporter};

use things::task::Task;
use anyhow::Result;
use clap::{Parser, ValueEnum};

#[derive(ValueEnum, Copy, Clone, Eq, PartialEq)]
enum Modes {
    /// Generate a report containing projected work for the day and a morning message
    Morning,
    /// Generate a report that is intended to be used for sharing what major tasks were completed
    /// in the last cycle.
    Cycle,
    /// Generate a report for what was actually done today and a signoff message
    Signoff,
}

impl Modes {
    fn format_tasks(&self, task_report: &str) -> String {
        match self {
            Modes::Morning => format!("Starting\n\n{}", task_report),
            Modes::Signoff => format!("Stopping now\n\n{}", task_report),
            Modes::Cycle => format!("*Cycle Report*\n\n{}", task_report),
        }
    }
}

impl Default for Modes {
    fn default() -> Modes {
        Modes::Morning
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct CliArgs {
    /// A list of tags to filter requests on
    #[arg(short, long)]
    tags: Vec<String>,

    /// Control what type of report to generate
    #[arg(short, long, default_value_t = Modes::default())]
    #[clap(value_enum)]
    mode: Modes,
}

fn main() -> Result<()> {
    let args = CliArgs::parse();
    let tasks = match args.mode {
        Modes::Morning => Task::today(),
        Modes::Signoff => Task::logbook_today(),
        Modes::Cycle => Task::logbook_this_cycle(),
    }?;
    let reported: Vec<Task> = tasks.into_iter().filter(|task| {
        args.tags.iter().all(|tag| task.has_tag(tag))
    }).collect();
    let report = args.mode.format_tasks(&MarkdownReporter.report(reported));
    println!("{report}");

    Ok(())
}

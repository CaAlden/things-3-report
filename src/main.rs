mod things;
mod reporter;
mod emoji;
mod names;

use reporter::{MarkdownReporter, Reporter, Resolution, ReportOptions};

use things::task::{Task, Status};
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
    fn format_tasks(&self, tasks: Vec<Task>, tags: &Vec<String>, sanitize_names: bool) -> String {
        match self {
            Modes::Morning => {
                let task_report = MarkdownReporter.report(tasks, &ReportOptions {
                    resolution: Resolution::FullTask,
                    tags: tags.to_vec(),
                    sanitize_names,
                });
                format!("{}\n\n{}", emoji::pick(3).join(" "), task_report)
            },
            Modes::Signoff => {
                let task_report = MarkdownReporter.report(tasks, &ReportOptions {
                    resolution: Resolution::FullTask,
                    tags: tags.to_vec(),
                    sanitize_names,
                });
                format!("Stopping now\n\n{}", task_report)
            },
            Modes::Cycle => {
                let further_filtered = tasks.into_iter().filter(|t| {
                    if let Some(p) = &t.project {
                        return p.status == Status::Completed;
                    }
                    return false;
                }).collect::<Vec<Task>>();
                let task_report = MarkdownReporter.report(further_filtered, &ReportOptions {
                    resolution: Resolution::Project,
                    tags: tags.to_vec(),
                    sanitize_names,
                });
                format!("*Cycle Report*\n\n{}", task_report)
            },
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

    /// By default, any @<name> style tags will be sanitized in the output to avoid @-mentions in
    /// slack. This is done by replacing vowel characters with look unicode lookalikes. If this
    /// flag is set then the names will be passed through unsanitized
    #[arg(long, default_value_t = false)]
    no_sanitize: bool,
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
    let report = args.mode.format_tasks(reported, &args.tags, !args.no_sanitize);
    println!("{report}");

    Ok(())
}

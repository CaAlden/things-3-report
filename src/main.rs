mod things;
mod reporter;
mod emoji;
mod names;

use reporter::{MarkdownReporter, Reporter, Resolution, ReportOptions};

use things::task::{Task, Status};
use anyhow::Result;
use clap::{Parser, ValueEnum};

#[derive(ValueEnum, Copy, Clone, Eq, PartialEq)]
enum ReportTypes {
    /// report projected work for the day and a morning message
    Morning,
    /// report what major tasks were completed in the last cycle.
    Cycle,
    /// report what was actually done today and a signoff message
    Signoff,
}

impl ReportTypes {
    fn format_tasks(&self, tasks: Vec<Task>, tags: &Vec<String>, sanitize_names: bool) -> String {
        match self {
            ReportTypes::Morning => {
                let task_report = MarkdownReporter.report(tasks, &ReportOptions {
                    resolution: Resolution::FullTask,
                    tags: tags.to_vec(),
                    sanitize_names,
                });
                format!("{}\n\n{}", emoji::pick(3).join(" "), task_report)
            },
            ReportTypes::Signoff => {
                let task_report = MarkdownReporter.report(tasks, &ReportOptions {
                    resolution: Resolution::FullTask,
                    tags: tags.to_vec(),
                    sanitize_names,
                });
                format!("Stopping now\n\n{}", task_report)
            },
            ReportTypes::Cycle => {
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

impl Default for ReportTypes {
    fn default() -> ReportTypes {
        ReportTypes::Morning
    }
}

/// A program that generates Slack flavor markdown reports from Things 3 todo list items.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct CliArgs {
    /// A list of tags to filter todos by. Only todo list items with every tag will be reported
    #[arg(short, long)]
    tags: Vec<String>,

    /// A list of tags to specifically omit from the results. Only todo list items WITHOUT these
    /// tags will be included
    #[arg(short, long)]
    omit: Vec<String>,

    /// Select the type of report to generate
    #[arg(short, long, default_value_t = ReportTypes::default())]
    #[clap(value_enum)]
    report: ReportTypes,

    /// By default, any @<name> style tags will be sanitized in the output to avoid @-mentions in
    /// Slack. This is done by replacing vowel characters with unicode lookalikes. If this
    /// flag is set then the names will be passed through unsanitized.
    #[arg(long, default_value_t = false)]
    no_sanitize: bool,
}

fn main() -> Result<()> {
    let args = CliArgs::parse();
    let tasks = match args.report {
        ReportTypes::Morning => Task::today(),
        ReportTypes::Signoff => Task::logbook_today(),
        ReportTypes::Cycle => Task::logbook_this_cycle(),
    }?;
    let mut reported: Vec<Task> = tasks.into_iter().filter(|task| {
        // Filter down to tasks with all selected tags and without any of the omitted tags
        args.tags.iter().all(|tag| task.has_tag(tag)) && !args.omit.iter().any(|tag| task.has_tag(tag))
    }).collect();
    reported.sort_by(|a, b| {
        a.completion_date.cmp(&b.completion_date)
    });
    let report = args.report.format_tasks(reported, &args.tags, !args.no_sanitize);
    println!("{report}");

    Ok(())
}

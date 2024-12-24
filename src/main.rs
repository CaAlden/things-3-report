mod things;
mod reporter;
mod emoji;
mod names;

use reporter::{MarkdownReporter, Reporter, Resolution, ReportOptions};

use things::task::Task;
use anyhow::Result;
use clap::{Parser, ValueEnum};

#[derive(ValueEnum, Copy, Clone, Eq, PartialEq)]
enum ListType {
    /// Generate a report from the Things today list
    Today,
    /// Generate a report from the Things logbook
    Logbook,
}

impl ListType {
    fn format_tasks(&self, tasks: Vec<Task>, tags: &Vec<String>, sanitize_names: bool, resolution: Resolution) -> String {
        match self {
            ListType::Today => {
                MarkdownReporter.report(tasks, &ReportOptions {
                    resolution,
                    tags: tags.to_vec(),
                    sanitize_names,
                })
            },
            ListType::Logbook => {
                MarkdownReporter.report(tasks, &ReportOptions {
                    resolution,
                    tags: tags.to_vec(),
                    sanitize_names,
                })
            },
        }
    }
}

impl Default for ListType {
    fn default() -> ListType {
        ListType::Today
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
    #[arg(short, long, default_value_t = ListType::default())]
    #[clap(value_enum)]
    list: ListType,

    /// By default, any @<name> style tags will be sanitized in the output to avoid @-mentions in
    /// Slack. This is done by replacing vowel characters with unicode lookalikes. If this
    /// flag is set then the names will be passed through unsanitized.
    #[arg(long, default_value_t = false)]
    no_sanitize: bool,

    /// An ISO date string for when to filter tasks from.
    /// Defaults to midnight this morning if unset. Not used for the today list
    #[arg(long)]
    from: Option<String>,

    /// An ISO date string for when to filter tasks until.
    /// Defaults to 1 second before midnight tonight if unset. Not used for the today list
    #[arg(long)]
    to: Option<String>,

    /// An optional message to include at the beginning of the report. If omitted, 3 random emojis
    /// will be included instead
    #[arg(short, long)]
    message: Option<String>,

    /// Choose a resolution for the report. This will determine how much detail is included in the
    /// output. Default is "FullTask"
    #[arg(short, long, default_value_t = Resolution::default())]
    #[clap(value_enum)]
    resolution: Resolution,
}

fn main() -> Result<()> {
    let args = CliArgs::parse();
    let now = chrono::Local::now();
    let from = args.from.unwrap_or_else(|| now.date().and_hms(0, 0, 0).to_rfc3339());
    let to = args.to.unwrap_or_else(|| (now + chrono::Duration::days(1)).date().and_hms(0, 0, 0).to_rfc3339());

    let message = args.message.unwrap_or_else(|| emoji::pick(3).join(" "));

    let tasks = match args.list {
        ListType::Today => Task::today(&from, &to),
        ListType::Logbook => Task::logbook(&from, &to),
    }?;
    let mut reported: Vec<Task> = tasks.into_iter().filter(|task| {
        // Filter down to tasks with all selected tags and without any of the omitted tags
        args.tags.iter().all(|tag| task.has_tag(tag)) && !args.omit.iter().any(|tag| task.has_tag(tag))
    }).collect();
    reported.sort_by(|a, b| {
        a.completion_date.cmp(&b.completion_date)
    });
    let report = args.list.format_tasks(reported, &args.tags, !args.no_sanitize, args.resolution);
    println!("{message}\n\n{report}");

    Ok(())
}

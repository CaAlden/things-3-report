use crate::things::task::{Task, Status};
use crate::names::sanitize_names;

/// Given a notes field and a list of possible tags for sections, return the content of triple tick
/// blocks containing those tags
///
/// # Examples
/// ```
/// extract_tagged_notes(
///     "\`\`\`report
///     Something
///     \`\`\`",
///     vec![String::from("report")],
/// ); // -> "Something"
/// ```
///
fn extract_tagged_notes(notes: &str, tags: &Vec<String>) -> Vec<String> {
    notes
        .split("```")
        .into_iter()
        .map(|section| -> (&str, Option<&String>) {
            (section, tags.iter().find(|t| section.starts_with(*t)))
        })
        .filter(|(_section, tag)| { tag.is_some() })
        .map(|(section, tag)| {
            let start_tag = tag.unwrap();
            section.strip_prefix(start_tag)
                .map(|s| s.trim())
                .expect("Failed to strip start tag prefix").to_string()
        })
        .collect()
}


#[derive(Debug)]
pub struct ProjectTree {
    id: String,
    title: String,
    notes: Option<String>,
    tags: Vec<String>,
    tasks: Vec<Task>,
}

#[derive(Debug)]
pub struct AreaTree {
    id: String,
    title: String,
    projects: Vec<ProjectTree>,
    hanging_tasks: Vec<Task>,
}

#[derive(Debug)]
pub struct ThingsTree {
    areas: Vec<AreaTree>,
    hanging_projects: Vec<ProjectTree>,
    hanging_tasks: Vec<Task>,
}

impl AreaTree {
    fn new(id: &str, title: &str) -> AreaTree {
        AreaTree {
            id: id.to_string(),
            title: title.to_string(),
            projects: vec![],
            hanging_tasks: vec![],
        }
    }
    fn add_task(&mut self, task: Task) {
        if let Some(project) = &task.project {
            if let Some(matched_project) = self.projects.iter_mut().find(|p| p.id == project.id) {
                matched_project.tasks.push(task);
            } else {
                self.projects.push(ProjectTree {
                    id: project.id.clone(),
                    title: project.title.clone(),
                    notes: project.notes.clone(),
                    tags: project.tags.clone(),
                    tasks: vec![task],
                });
            }
        } else {
            self.hanging_tasks.push(task);
        }
    }
}

impl ThingsTree {
    pub fn new() -> ThingsTree {
        ThingsTree { areas: vec![], hanging_tasks: vec![], hanging_projects: vec![] }
    }
    pub fn add_task(&mut self, task: Task) {
        if let Some(area) = &task.area {
            if let Some(matched_area) = self.areas.iter_mut().find(|a| a.id == area.id) {
                matched_area.add_task(task);
            } else {
                let mut new_area = AreaTree::new(&area.id, &area.title);
                new_area.add_task(task);
                self.areas.push(new_area);
            }
        } else {
            if let Some(project) = &task.project {
                if let Some(matched_project) = self.hanging_projects.iter_mut().find(|p| p.id == project.id) {
                    matched_project.tasks.push(task);
                } else {
                    self.hanging_projects.push(ProjectTree {
                        id: project.id.clone(),
                        title: project.title.clone(),
                        notes: project.notes.clone(),
                        tags: project.tags.clone(),
                        tasks: vec![task],
                    });
                }
            } else {
                self.hanging_tasks.push(task);
            }
        }
    }


    pub fn from_tasks(tasks: Vec<Task>) -> ThingsTree {
        let mut tree = ThingsTree::new();
        for task in tasks.into_iter() {
            tree.add_task(task);
        }
        return tree;
    }
}

pub enum Resolution {
    FullTask,
    Project,
}

pub struct ReportOptions {
    pub resolution: Resolution,
    pub tags: Vec<String>,
    pub sanitize_names: bool,
}

pub trait Reporter {
    fn report_task(&mut self, task: &Task, depth: usize, options: &ReportOptions) -> String;
    fn report_project(&mut self, project: &ProjectTree, depth: usize, options: &ReportOptions) -> String;
    fn report_single_area(&mut self, area: &AreaTree, options: &ReportOptions) -> String;
    fn report_multiple_areas(&mut self, areas: &Vec<AreaTree>, options: &ReportOptions) -> String;
    fn report(&mut self, tasks: Vec<Task>, options: &ReportOptions) -> String {
        let tree = ThingsTree::from_tasks(tasks);
        let untracked_tasks = tree.hanging_tasks
            .iter()
            .map(|t| self.report_task(t, 0, options))
            .collect::<Vec<String>>()
            .join("\n");
        let area_tasks: String = match tree.areas.len() {
            0 => "".to_string(),
            1 => self.report_single_area(&tree.areas[0], options),
            _ => self.report_multiple_areas(&tree.areas, options),
        };

        let separator = if area_tasks == "" || untracked_tasks == "" {
            ""
        } else {
            "\n\n"
        };

        format!("{}{}{}", area_tasks, separator, untracked_tasks)
    }
}

pub struct MarkdownReporter;

impl Reporter for MarkdownReporter {
    fn report_task(&mut self, task: &Task, depth: usize, options: &ReportOptions) -> String {
        let relevant_notes = task.notes.clone()
            .map(|notes| extract_tagged_notes(&notes, &options.tags))
            .unwrap_or(vec![])
            .iter()
            .map(|l| format!("\n{}- {}", String::from(" ").repeat(depth + 4), l))
            .collect::<Vec<String>>()
            .join("");
        let title = if task.status == Status::Canceled {
            format!("~{}~", task.title)
        } else {
            task.title.to_string()
        };
        let mut output = format!("\n{}- {}{}", String::from(" ").repeat(depth), title, relevant_notes);
        if options.sanitize_names {
            output = sanitize_names(&output, &task.tags);
        }

        output
    }
    fn report_project(&mut self, project: &ProjectTree, depth: usize, options: &ReportOptions) -> String {
        let resolution = &options.resolution;
        let relevant_notes = project.notes.clone()
            .map(|notes| extract_tagged_notes(&notes, &options.tags))
            .unwrap_or(vec![])
            .iter()
            .map(|l| format!("\n{}- {}", String::from(" ").repeat(depth + 4), l))
            .collect::<Vec<String>>()
            .join("");
        let mut output = match resolution {
            Resolution::FullTask => {
                let tasks = project.tasks
                    .iter()
                    .map(|t| self.report_task(t, depth + 4, options))
                    .collect::<Vec<String>>()
                    .join("");
                format!("{}{}{}{}", String::from(" ").repeat(depth), project.title, relevant_notes, tasks)
            },
            Resolution::Project => {
                format!("{}- {}{}", String::from(" ").repeat(depth), project.title, relevant_notes)
            }
        };

        if options.sanitize_names {
            output = sanitize_names(&output, &project.tags);
        }

        output
    }
    fn report_single_area(&mut self, area: &AreaTree, options: &ReportOptions) -> String {
        let project_reports = area.projects
            .iter()
            .map(|p| self.report_project(p, 0, options))
            .collect::<Vec<String>>()
            .join("\n");
        let untracked_tasks = area.hanging_tasks
            .iter()
            .map(|t| self.report_task(t, 0, options))
            .collect::<Vec<String>>()
            .join("");
        let separator = if project_reports == "" || untracked_tasks == "" {
            ""
        } else {
            "\n\n"
        };
        match options.resolution {
            Resolution::FullTask => format!("{}{}{}", project_reports, separator, untracked_tasks),
            Resolution::Project => format!("{}", project_reports)
        }
    }
    fn report_multiple_areas(&mut self, areas: &Vec<AreaTree>, options: &ReportOptions) -> String {
        match areas.len() {
            0 => "".to_string(),
            1 => self.report_single_area(&areas[0], options),
            _ => {
                areas.iter().map(|area| {
                    let single = self.report_single_area(area, options);
                    format!("*{}*\n{}", area.title, single)
                })
                .collect::<Vec<String>>()
                .join("\n\n")
            },
        }
    }
}

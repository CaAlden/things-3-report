use crate::things::task::Task;

#[derive(Debug)]
pub struct ProjectTree {
    id: String,
    title: String,
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
            id: id.clone().to_string(),
            title: title.clone().to_string(),
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

pub trait Reporter {
    fn report_task(&mut self, task: &Task, depth: usize) -> String;
    fn report_project(&mut self, project: &ProjectTree, depth: usize) -> String;
    fn report_single_area(&mut self, area: &AreaTree) -> String;
    fn report_multiple_areas(&mut self, areas: &Vec<AreaTree>) -> String;
    fn report(&mut self, tasks: Vec<Task>) -> String {
        let tree = ThingsTree::from_tasks(tasks);
        let untracked_tasks = tree.hanging_tasks
            .iter()
            .map(|t| self.report_task(t, 0))
            .collect::<Vec<String>>()
            .join("\n");
        let area_tasks: String = match tree.areas.len() {
            0 => "".to_string(),
            1 => self.report_single_area(&tree.areas[0]),
            _ => self.report_multiple_areas(&tree.areas),
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
    fn report_task(&mut self, task: &Task, depth: usize) -> String {
        format!("{}- {}", String::from(" ").repeat(depth), task.title)
    }
    fn report_project(&mut self, project: &ProjectTree, depth: usize) -> String {
        let tasks = project.tasks.iter().map(|t| self.report_task(t, depth + 4)).collect::<Vec<String>>().join("\n");
        format!("{}{}\n{}", String::from(" ").repeat(depth), project.title, tasks)
    }
    fn report_single_area(&mut self, area: &AreaTree) -> String {
        let project_tasks = area.projects.iter().map(|p| self.report_project(p, 0)).collect::<Vec<String>>().join("\n");
        let untracked_tasks = area.hanging_tasks.iter().map(|t| self.report_task(t, 0)).collect::<Vec<String>>().join("\n");
        let separator = if project_tasks == "" || untracked_tasks == "" {
            ""
        } else {
            "\n\n"
        };
        format!("{}{}{}", project_tasks, separator, untracked_tasks)
    }
    fn report_multiple_areas(&mut self, areas: &Vec<AreaTree>) -> String {
        match areas.len() {
            0 => "".to_string(),
            1 => self.report_single_area(&areas[0]),
            _ => {
                areas.iter().map(|area| {
                    let single = self.report_single_area(area);
                    format!("*{}*\n{}", area.title, single)
                })
                .collect::<Vec<String>>()
                .join("\n\n")
            },
        }
    }
}

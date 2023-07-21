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
    hanging_tasks: Vec<Task>,
}

impl ProjectTree {
    /// Add the task to this project if it belongs here, otherwise pass it back out.
    pub fn try_take_task(&mut self, mut task: Task) -> Option<Task> {
        if let Some(proj) = task.project {
            if proj.id == self.id {
                task.project = Some(proj);
                self.tasks.push(task);
                return None;
            }
            task.project = Some(proj);
            return Some(task);
        }
        return Some(task);
    }
}

impl AreaTree {
    pub fn add_new_project_and_task(&mut self, mut task: Task) {
        if let Some(proj) = task.project {
            let id = proj.id.clone();
            let title = proj.title.clone();
            task.project = Some(proj);
            self.projects.push(ProjectTree {
                id,
                title,
                tasks: vec![task],
            });
        } else {
            self.hanging_tasks.push(task);
        }
    }
    pub fn try_take_task(&mut self, mut task: Task) -> Option<Task> {
        if let Some(area) = task.area {
            if area.id == self.id {
                task.area = Some(area);
                let mut maybe_task = Some(task);
                for proj in self.projects.iter_mut() {
                    if let Some(t) = maybe_task {
                        maybe_task = proj.try_take_task(t);
                    }
                }
                // Here, the task belongs in this area but there was no project for it.
                maybe_task.map(|t| self.add_new_project_and_task(t));
                return None;
            }
            task.area = Some(area);
            return Some(task);
        }
        return Some(task);
    }
}

impl ThingsTree {
    pub fn new() -> ThingsTree {
        ThingsTree { areas: vec![], hanging_tasks: vec![] }
    }

    pub fn add_new_area_and_task(&mut self, mut task: Task) {
        if let Some(area) = task.area {
            let id = area.id.clone();
            let title = area.title.clone();
            task.area = Some(area);
            let mut area_tree = AreaTree  {
                id,
                title,
                projects: vec![],
                hanging_tasks: vec![],
            };
            let took = area_tree.try_take_task(task);
            if took.is_some() {
                panic!("Area should have matched the task because it was created with the task");
            }
            self.areas.push(area_tree);
        } else {
            self.hanging_tasks.push(task);
        }
    }

    pub fn try_take_task(&mut self, task: Task) {
        let mut maybe_task = Some(task);
        for area in self.areas.iter_mut() {
            if let Some(t) = maybe_task {
                maybe_task = area.try_take_task(t);
            }
        }
        maybe_task.map(|t| self.add_new_area_and_task(t));
    }

    pub fn from_tasks(tasks: Vec<Task>) -> ThingsTree {
        let mut tree = ThingsTree::new();
        for task in tasks.into_iter() {
            tree.try_take_task(task);
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

use std::str::from_utf8;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use osascript;
use serde_json;
use chrono::{DateTime, Utc};

#[derive(Deserialize, Debug, Eq, PartialEq)]
pub enum Status {
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "incomplete")]
    Incomplete,
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "canceled")]
    Canceled,
}

#[derive(Deserialize, Debug)]
pub struct Project {
    pub id: String,
    pub title: String,
    pub tags: Vec<String>,
    pub notes: Option<String>,
    pub status: Status,
}

#[derive(Deserialize, Debug)]
pub struct Area {
    pub id: String,
    pub title: String,
}

#[derive(Deserialize, Debug)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub notes: Option<String>,
    pub status: Status,
    pub tags: Vec<String>,
    pub completion_date: Option<DateTime<Utc>>,
    pub project: Option<Project>,
    pub area: Option<Area>,
}

#[derive(Serialize, Debug)]
pub struct TaskParams {
    // ISO string for the start date
    from: String,
    // ISO string for the end date
    to: String,
}

impl Task {
    /// A Helper for loading Tasks from json returned by an osascript
    fn from_script(script_bytes: &[u8], params: TaskParams) -> Result<Vec<Task>> {
        let script = osascript::JavaScript::new(from_utf8(script_bytes)?);
        let raw_json: String = script.execute_with_params(params)?;
        let tasks: Vec<Task> = serde_json::from_str(&raw_json)?;
        Ok(tasks)
    }

    /// Get all tasks in the today list from Things
    pub fn today(from: &String, to: &String) -> Result<Vec<Task>> {
        Task::from_script(include_bytes!("today.js"), TaskParams { from: from.to_string(), to: to.to_string() })
    }

    /// Get all tasks in the logbook list from Things
    pub fn logbook(from: &String, to: &String) -> Result<Vec<Task>> {
        Task::from_script(include_bytes!("logbook.js"), TaskParams { from: from.to_string(), to: to.to_string() })
    }

    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.contains(&String::from(tag))
    }
}

use std::str::from_utf8;
use serde::Deserialize;
use anyhow::Result;
use osascript;
use serde_json;
use chrono::{DateTime, Utc};

#[derive(Deserialize, Debug)]
pub enum Status {
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "incomplete")]
    Incomplete,
    #[serde(rename = "open")]
    Open,
}

#[derive(Deserialize, Debug)]
pub struct Project {
    pub id: String,
    pub title: String,
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

impl Task {
    /// A Helper for loading Tasks from json returned by an osascript
    fn from_script(script_bytes: &[u8]) -> Result<Vec<Task>> {
        let script = osascript::JavaScript::new(from_utf8(script_bytes)?);
        let raw_json: String = script.execute()?;
        let tasks: Vec<Task> = serde_json::from_str(&raw_json)?;
        Ok(tasks)
    }

    /// Get all tasks in the today list from Things
    pub fn today() -> Result<Vec<Task>> {
        Task::from_script(include_bytes!("today.js"))
    }

    /// Get all tasks in the logbook list from Things
    pub fn logbook() -> Result<Vec<Task>> {
        Task::from_script(include_bytes!("logbook.js"))
    }

    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.contains(&String::from(tag))
    }
}

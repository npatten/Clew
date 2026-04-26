use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Backlog,
    Todo,
    InProgress,
    Done,
    Abandoned,
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Backlog => write!(f, "backlog"),
            Status::Todo => write!(f, "todo"),
            Status::InProgress => write!(f, "in_progress"),
            Status::Done => write!(f, "done"),
            Status::Abandoned => write!(f, "abandoned"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Increment {
    pub id: u32,
    pub status: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abandoned_reason: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    /// Preserves any frontmatter fields the CLI doesn't know about.
    /// Load-bearing for the extensibility model — do not remove.
    #[serde(flatten)]
    pub extra: HashMap<String, yaml_serde::Value>,
}

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

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
    #[serde(with = "clew_timestamp")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "clew_timestamp")]
    pub updated_at: DateTime<Utc>,

    /// Preserves any frontmatter fields the CLI doesn't know about.
    /// Load-bearing for the extensibility model — do not remove.
    /// BTreeMap (not HashMap) for deterministic serialization order; the
    /// markdown+frontmatter output IS our agent-facing API and must not drift.
    #[serde(flatten)]
    pub extra: BTreeMap<String, yaml_serde::Value>,
}

mod clew_timestamp {
    use chrono::{DateTime, Utc};
    use serde::{de::Error, Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%Y-%m-%dT%H:%M:%SZ";

    pub fn serialize<S>(timestamp: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&timestamp.format(FORMAT).to_string())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;

        if !value.ends_with('Z') {
            return Err(D::Error::custom(
                "timestamp must be UTC with a 'Z' suffix, e.g. 2026-04-26T10:00:00Z",
            ));
        }

        if value.contains('.') {
            return Err(D::Error::custom(
                "timestamp must use second precision; subseconds are not allowed",
            ));
        }

        let parsed = DateTime::parse_from_rfc3339(&value).map_err(D::Error::custom)?;
        if parsed.timestamp_subsec_nanos() != 0 {
            return Err(D::Error::custom(
                "timestamp must use second precision; subseconds are not allowed",
            ));
        }

        Ok(parsed.with_timezone(&Utc))
    }
}

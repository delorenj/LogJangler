use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEvent {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub event_type: EventType,
    pub context: EventContext,
    pub payload: EventPayload,
    #[serde(default)]
    pub extensions: HashMap<String, serde_json::Value>,
}

impl LogEvent {
    pub fn new(event_type: EventType, context: EventContext, payload: EventPayload) -> Self {
        Self {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            event_type,
            context,
            payload,
            extensions: HashMap::new(),
        }
    }

    pub fn routing_key(&self) -> String {
        match self.event_type {
            EventType::CommandStart => "logjangler.events.command.start",
            EventType::CommandOutput => "logjangler.events.command.output",
            EventType::CommandComplete => "logjangler.events.command.complete",
        }.to_string()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum EventType {
    CommandStart,
    CommandOutput,
    CommandComplete,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventContext {
    pub session_id: String,
    pub user: String,
    pub host: String,
    pub cwd: PathBuf,
    pub shell: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventPayload {
    pub raw: String,
    pub plain: String,
}

impl EventPayload {
    pub fn new(raw: impl Into<String>) -> Self {
        let raw = raw.into();
        let plain = raw.clone(); // TODO: Strip ANSI codes
        Self { raw, plain }
    }
}

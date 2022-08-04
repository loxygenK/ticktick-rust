use serde::Deserialize;
use serde_repr::Deserialize_repr;

pub type DateTimeString = String;

// Referring https://developer.ticktick.com/api#/openapi.

#[derive(Deserialize_repr)]
#[repr(u32)]
pub enum PriorityPayload {
    None = 0,
    Low = 1,
    Medium = 3,
    High = 5
}

#[derive(Deserialize_repr)]
#[repr(u32)]
pub enum StatusPayload {
    Normal = 0,
    Completed = 1,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskPayload {
    pub id: String,
    pub project_id: String,
    pub title: String,
    pub all_day: bool,
    pub completed_time: DateTimeString,
    pub content: String,
    pub desc: String,
    pub due_date: DateTimeString,
    pub items: ChecklistItemPayload,
    pub priority: PriorityPayload,
    pub reminders: Vec<String>,
    pub repeat: String,
    pub sort_order: u64,
    pub start_date: DateTimeString,
    pub status: Vec<StatusPayload>,
    pub time_zone: String
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChecklistItemPayload {
    pub id: String,
    pub title: String,
    pub status: Vec<StatusPayload>,
    pub completed_time: DateTimeString,
    pub is_all_day: bool,
    pub sort_order: u64,
    pub start_date: DateTimeString,
    pub time_zone: String
}

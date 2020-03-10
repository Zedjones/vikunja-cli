use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use super::user::User;
use super::label::Label;

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    assignees: Option<Vec<User>>,
    created: DateTime<Utc>,
    #[serde(rename = "createdBy")]
    created_by: User,
    description: String,
    done: bool,
    #[serde(rename = "doneAt")]
    done_at: Option<DateTime<Utc>>,
    #[serde(rename = "dueDate")]
    due_date: Option<DateTime<Utc>>,
    #[serde(rename = "endDate")]
    end_date: Option<DateTime<Utc>>,
    #[serde(rename = "hexColor")]
    hex_color: String,
    id: i32,
    identifier: String,
    index: i32,
    labels: Option<Vec<Label>>,
    #[serde(rename = "listID")]
    list_id: i32,
    #[serde(rename = "percentDone")]
    percent_done: f32,
    priority: i32,
    #[serde(rename = "reminderDates")]
    reminder_dates: Option<Vec<DateTime<Utc>>>,
    #[serde(rename = "repeatAfter")]
    repeat_after: i32,
    #[serde(rename = "startDate")]
    start_date: Option<DateTime<Utc>>,
    text: String,
    updated: DateTime<Utc>
}
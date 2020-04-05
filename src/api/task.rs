use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use super::user::User;
use super::label::Label;

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub assignees: Option<Vec<User>>,
    pub created: DateTime<Utc>,
    #[serde(rename = "createdBy")]
    pub created_by: User,
    pub description: String,
    pub done: bool,
    #[serde(rename = "doneAt")]
    pub done_at: Option<DateTime<Utc>>,
    #[serde(rename = "dueDate")]
    pub due_date: Option<DateTime<Utc>>,
    #[serde(rename = "endDate")]
    pub end_date: Option<DateTime<Utc>>,
    #[serde(rename = "hexColor")]
    pub hex_color: String,
    pub id: i32,
    pub identifier: String,
    pub index: i32,
    pub labels: Option<Vec<Label>>,
    #[serde(rename = "listID")]
    pub list_id: i32,
    #[serde(rename = "percentDone")]
    pub percent_done: f32,
    pub priority: i32,
    #[serde(rename = "reminderDates")]
    pub reminder_dates: Option<Vec<DateTime<Utc>>>,
    #[serde(rename = "repeatAfter")]
    pub repeat_after: i32,
    #[serde(rename = "startDate")]
    pub start_date: Option<DateTime<Utc>>,
    pub text: String,
    pub updated: DateTime<Utc>
}
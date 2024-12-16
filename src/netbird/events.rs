use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Event {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub activity: String,
    pub activity_code: String,
    pub initiator_id: String,
    pub initiator_name: String,
    pub initiator_email: String,
    pub target_id: String,
    pub meta: serde_json::Value,
}

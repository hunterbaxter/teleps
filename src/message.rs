use chrono::{offset::Utc, serde::ts_nanoseconds_option, DateTime, Duration};
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::{
    fmt,
    fmt::{Display, Formatter},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub sender_ip: String,
    pub receiver_ip: String,
    topic: String,
    #[serde(with = "ts_nanoseconds_option")]
    pub send_time: Option<DateTime<Utc>>,
    #[serde(with = "ts_nanoseconds_option")]
    pub receive_time: Option<DateTime<Utc>>,
}

impl Message {
    pub fn new(
        sender_ip: String,
        receiver_ip: String,
        topic: String,
        send_time: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            sender_ip,
            receiver_ip,
            send_time,
            receive_time: None,
            topic,
        }
    }

    pub fn new_from_string(data: &str) -> Result<Self> {
        serde_json::from_str(data)
    }

    pub fn to_string(&self) -> Result<String> {
        serde_json::to_string(&self)
    }

    pub fn duration(&self) -> Option<Duration> {
        match (self.receive_time, self.send_time) {
            (None, Some(_)) | (Some(_), None) | (None, None) => None,
            (Some(x), Some(y)) => Some(x - y),
        }
    }

    pub fn add_recieve_time(&mut self, recieve_time: DateTime<Utc>) {
        self.receive_time = Some(recieve_time);
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "message sent at {} from {} -> {}",
            self.send_time.unwrap(),
            self.sender_ip,
            self.receiver_ip
        )
    }
}

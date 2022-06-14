use chrono::{offset::Utc, serde::ts_seconds_option, DateTime, Duration};
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct Message {
    sender_ip: String,
    receiver_ip: String,
    #[serde(with = "ts_seconds_option")]
    send_time: Option<DateTime<Utc>>,
    #[serde(with = "ts_seconds_option")]
    receive_time: Option<DateTime<Utc>>,
    topic: String,
}

impl Message {
    fn new(
        sender_ip: String,
        receiver_ip: String,
        send_time: Option<DateTime<Utc>>,
        topic: String,
    ) -> Self {
        Self {
            sender_ip,
            receiver_ip,
            send_time,
            receive_time: None,
            topic,
        }
    }

    fn new_from_string(data: &str) -> Result<Message> {
        serde_json::from_str(data)
    }

    fn to_string(self) -> Result<String> {
        serde_json::to_string(&self)
    }

    fn duration(self) -> Option<Duration> {
        match (self.receive_time, self.send_time) {
            (None, Some(_)) | (Some(_), None) | (None, None) => None,
            (Some(x), Some(y)) => Some(x - y),
        }
    }

    fn add_recieve_time(&mut self, recieve_time: DateTime<Utc>) {
        self.receive_time = Some(recieve_time);
    }
}

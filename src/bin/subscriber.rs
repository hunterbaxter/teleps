// use chrono::{offset::Utc, DateTime};
use kafka::consumer::Consumer;
use std::{str, time::SystemTime};
use teleps::message::Message;

const TOPIC: &str = "test";

fn main() {
    let mut consumer = Consumer::from_hosts(vec!["localhost:9092".to_owned()])
        .with_topic(TOPIC.to_owned())
        .create()
        .unwrap();
    loop {
        for ms in consumer.poll().unwrap().iter() {
            for m in ms.messages() {
                match str::from_utf8(m.value) {
                    Ok(m) => {
                        // WARN: not handling error if parsing message fails
                        let mut message: Message = Message::new_from_string(m).unwrap();
                        println!("{}", message);
                        message.add_recieve_time(SystemTime::now().into());
                        println!("message time = {}\n", message.duration().unwrap());
                    }
                    Err(e) => println!("received invalid utf8 sequence: {}", e),
                }
            }
            consumer
                .consume_messageset(ms)
                .expect("I don't know why this would fail");
        }
        consumer.commit_consumed().unwrap();
    }
}

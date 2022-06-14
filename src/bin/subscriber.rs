use chrono::{offset::Utc, DateTime};
use kafka::consumer::Consumer;
use std::{str, time::SystemTime};

const TOPIC: &str = "test";

fn main() {
    let mut consumer = Consumer::from_hosts(vec!["localhost:9092".to_owned()])
        .with_topic(TOPIC.to_owned())
        .create()
        .unwrap();
    loop {
        for ms in consumer.poll().unwrap().iter() {
            for m in ms.messages() {
                // println!("{:?}", m); // if want to view raw bytes
                match str::from_utf8(m.value) {
                    Ok(m) => {
                        // println!("{}", m);
                        match DateTime::parse_from_str(m, "%+") {
                            Ok(t) => {
                                // println!("{}",t);
                                // TODO: this is quite annoying
                                let send_time = DateTime::<Utc>::from(t);
                                let receive_time: DateTime<Utc> = SystemTime::now().into();
                                let elapsed_time = receive_time - send_time;
                                println!("elapsed time = {}", elapsed_time);
                            },
                            Err(e) => println!("did not parse time = {}",e)
                        }
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

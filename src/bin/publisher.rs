use kafka::producer::{Producer, Record, RequiredAcks};
// use std::fmt::Write;
use std::time::Duration;
use std::thread::sleep;
// use std::env; // set if need to debug

const TOPIC: &str = "test";

fn main() {
    // env::set_var("RUST_BACKTRACE", "1"); // set if need to debug
    let mut producer = Producer::from_hosts(vec!["localhost:9092".to_owned()])
        .with_ack_timeout(Duration::from_secs(1))
        .with_required_acks(RequiredAcks::One)
        .create()
        .unwrap();
    // let mut buf = String::with_capacity(2);
    let mut buf = String::new();
    for i in 1..10 {
        buf.push_str(&format!("message number = {}", i));
        // let _ = write!(&mut buf, "{}", i); // more efficient, but less flexible
        producer
            .send(&Record::from_value(TOPIC, buf.as_bytes()))
            .unwrap();
        buf.clear();
        sleep(Duration::from_secs(5));
    }
}

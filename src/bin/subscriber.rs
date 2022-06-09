use kafka::consumer::Consumer;
use std::str;

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
                    Ok(m) => println!("{}",m),
                    Err(e) => println!("received invalid utf8 sequence: {}", e)
                }
            }
            consumer.consume_messageset(ms).expect("I don't know why this would fail");
        }
        consumer.commit_consumed().unwrap();
    }
}

use kafka::producer::{Producer, Record, RequiredAcks};
use chrono::{
    DateTime,
    offset::Utc,
};
use std::{
    thread::sleep,
    time::{Duration, SystemTime},
};
use opentelemetry::{
    global, sdk,
    trace::{TraceError, Tracer},
};

const TOPIC: &str = "test";

fn init_tracer() -> Result<sdk::trace::Tracer, TraceError> {
    opentelemetry_jaeger::new_pipeline().install_simple()
}

fn main() -> Result<(), opentelemetry::trace::TraceError> {
    global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());
    let tracer = init_tracer().expect("failed to initialize tracer");

    tracer.in_span("doing_work", |_| {
        let mut producer = Producer::from_hosts(vec!["localhost:9092".to_owned()])
            .with_ack_timeout(Duration::from_secs(1))
            .with_required_acks(RequiredAcks::One)
            .create()
            .unwrap();
        // let mut buf = String::with_capacity(2);
        let mut buf = String::new();
        for _ in 1..10 {
            let system_time = SystemTime::now();
            let datetime: DateTime<Utc> = system_time.into();
            // buf.push_str(&format!("{}", datetime.format("%Y-%m-%dT%H%:%M:%S.%f")));
            buf.push_str(&format!("{}", datetime.format("%+")));

            println!("sending: {}", buf);
            // let _ = write!(&mut buf, "{}", i); // more efficient, but less flexible
            producer
                .send(&Record::from_value(TOPIC, buf.as_bytes()))
                .unwrap();
            buf.clear();
            sleep(Duration::from_secs(2));
        }
    });
    global::shutdown_tracer_provider();
    Ok(())
}

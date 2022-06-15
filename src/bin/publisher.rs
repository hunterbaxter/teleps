use kafka::producer::{Producer, Record, RequiredAcks};
use local_ip_address::local_ip;
use opentelemetry::{
    global, sdk,
    trace::{TraceError, Tracer},
};
use std::{
    // fmt::format,
    thread::sleep,
    time::{Duration, SystemTime},
};
use teleps::message::Message;

const TOPIC: &str = "test";
const RECEIVER_IP: &str = "localhost";
const RECEIVER_PORT: &str = "9092";

fn init_tracer() -> Result<sdk::trace::Tracer, TraceError> {
    opentelemetry_jaeger::new_pipeline().install_simple()
}

fn main() -> Result<(), opentelemetry::trace::TraceError> {
    global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());
    let tracer = init_tracer().expect("failed to initialize tracer");

    tracer.in_span("doing_work_oltp", |_| {
        let mut producer =
            Producer::from_hosts(vec![RECEIVER_IP.to_string() + ":" + RECEIVER_PORT])
                .with_ack_timeout(Duration::from_secs(1))
                .with_required_acks(RequiredAcks::One)
                .create()
                .unwrap();
        for _ in 1..10 {
            let message = Message::new(
                local_ip().unwrap().to_string(),
                RECEIVER_IP.to_string(),
                TOPIC.to_string(),
                Some(SystemTime::now().into()),
            );
            println!("sending: {:?}", message);
            producer
                .send(&Record::from_value(
                    TOPIC,
                    message.to_string().unwrap().as_bytes(),
                ))
                .unwrap();
            sleep(Duration::from_secs(2));
        }
    });
    global::shutdown_tracer_provider();
    Ok(())
}

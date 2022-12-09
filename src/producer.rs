use std::time::Duration;
use kafka::producer::{Producer, Record, RequiredAcks};
use kafka::error::Error;
fn main() {

}

fn produce_message() -> Result<(), Error> {
    let mut producer = Producer::from_hosts(vec!["localhost:9092".to_owned()])
        .with_ack_timeout(Duration::from_secs(1))
        .with_required_acks(RequiredAcks::One)
        .create()?;

    let record = Record::from_value("my-topic", "hello kafka");
    producer.send(&record)?;
    Ok(())
}



use rdkafka::config::ClientConfig;
use rdkafka::producer::{BaseProducer, BaseRecord, Producer};
use std::time::Duration;


pub async fn producer(payload:String) {
    let producer: BaseProducer = ClientConfig::new()
    .set("bootstrap.servers", "0.0.0.0:9092")
    .create()
    .expect("Producer creation error");

    producer.send(
        BaseRecord::to("Hacker_News")
            .payload(&payload)
            .key("and this is a key")
    ).expect("Failed to enqueue");

    // Poll at regular intervals to process all the asynchronous delivery events.
    for _ in 0..10 {
        producer.poll(Duration::from_millis(100));
    }

// And/or flush the producer before dropping it.
    let _ = producer.flush(Duration::from_secs(1));


}
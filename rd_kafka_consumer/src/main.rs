mod consumer;

use consumer::consume_and_print;

#[tokio::main]
async fn main() {
    consume_and_print("0.0.0.0:9092", "group_id", &["Hacker_News"]).await;
}

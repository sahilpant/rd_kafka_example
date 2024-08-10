mod hn;
mod producer;
// mod consumer;

use actix_web::{get, web::{self, Json}, App, HttpResponse, HttpServer, Responder};
// use consumer::consume_and_print;
use hn::fetch_hn_stories;
use producer::producer;

#[get("/hn/{search_term}")]
async fn search(search_term: web::Path<String>) -> impl Responder {
    let res = fetch_hn_stories(search_term.to_string(),1).await.unwrap();
    serde_json::to_string(&res)
}

#[get("/hn/produce/{search_term}")]
async fn produce(search_term: web::Path<String>) -> impl Responder {
    let res = fetch_hn_stories(search_term.to_string(),1).await.unwrap();
    let msg = serde_json::to_string(&res).unwrap();
    producer(msg).await;
    HttpResponse::Ok().body("Published SuccessFully")
}



#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    // tokio::task::spawn_blocking( || async {
    //     consume_and_print("0.0.0.0:9092", "group_id", &["Hacker_News"]).await;
    // });
    HttpServer::new(|| {
        App::new()
            .service(search)
            .service(produce)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
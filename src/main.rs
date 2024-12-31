use rogue_coda::{init_logging, Server};

#[tokio::main]
async fn main() {
    let _ = init_logging();
    let server = Server::create("0.0.0.0:2632".to_owned())
        .await
        .expect("should be able to create server");
    server
        .start()
        .await
        .expect("should be able to start server");
}

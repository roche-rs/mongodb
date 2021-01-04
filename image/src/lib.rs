pub mod functions;

use async_std::sync::Arc;
use mongodb::Client;

#[derive(Clone, Debug)]
pub struct State {
    client: Arc<Client>,
}

#[cfg(test)]
#[async_std::test]
async fn run_lib() {
    use dotenv::dotenv;
    use mongodb::{options::ClientOptions, Client};
    use std::env;
    use tide::prelude::*;
    dotenv().ok();
    let mongodb_conn = match env::var("MONGODB_CONNECTION_STRING") {
        Ok(val) => val,
        Err(e) => panic!("Error getting MONGODB_CONNECTION_STRING: {}", e),
    };

    let mut client_options = match ClientOptions::parse(&mongodb_conn).await {
        Ok(c) => c,
        Err(e) => panic!("Client Options Failed: {}", e),
    };

    client_options.app_name = Some("roche-mongodb-service".to_string());

    let client = match Client::with_options(client_options) {
        Ok(c) => c,
        Err(e) => panic!("Client Creation Failed: {}", e),
    };

    let engine = State {
        client: Arc::new(client),
    };

    use tide_testing::TideTestingExt;
    let app = functions::handler(engine);
    let response_json: serde_json::value::Value = app.get("/").recv_json().await.unwrap();

    assert_eq!(response_json, json!([]));
}

#[async_std::test]
async fn default() {
    use tide_testing::TideTestingExt;
    let app = functions::default();
    let resp_string = app.get("/").recv_string().await.unwrap();
    assert!(resp_string.contains("httpbin.org"));
}
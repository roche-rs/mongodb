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
    use dotenv;
    use mongodb::{options::ClientOptions, Client};
    use std::env;
    use std::path::Path;
    use tide::prelude::*;

    let dirname = env::current_dir().unwrap();
    let envlocation = format!("{}/src/.env", dirname.display());
    let env_path = Path::new(&envlocation);
    dotenv::from_path(env_path).unwrap();

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

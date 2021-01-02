use async_mongodb_session::MongodbSessionStore;
use async_std::sync::Arc;
use async_std::task;
use dotenv::dotenv;
use mongodb::{options::ClientOptions, Client};
use std::env;

mod functions;

#[derive(Clone, Debug)]
pub struct State {
    client: Arc<Client>,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let address = format!("0.0.0.0:{}", port);
    task::block_on(async {
    
        tide::log::start();
        if cfg!(debug_assertions) {
            dotenv().ok();
        } else {
            println!("Not a debug build so ignoring .env file")
        }
    
        let mongodb_conn = match env::var("MONGODB_CONNECTION_STRING") {
            Ok(val) => val,
            Err(e) => panic!("Error getting MONGODB_CONNECTION_STRING: {}", e),
        };
    
        let mut client_options = match ClientOptions::parse(&mongodb_conn).await {
            Ok(c) => c,
            Err(e) => panic!("Client Options Failed: {}", e),
        };
        // Manually set an option
        // Maybe look at ENV_VARS here?
        client_options.app_name = Some("roche-mongodb-service".to_string());
    
        // Get a handle to the deployment.
        let client = match Client::with_options(client_options) {
            Ok(c) => c,
            Err(e) => panic!("Client Creation Failed: {}", e),
        };
    
        let engine = State {
            client: Arc::new(client),
        };
        let mut app = tide::with_state(engine);
        app.with(tide::sessions::SessionMiddleware::new(
            MongodbSessionStore::new(&mongodb_conn.as_str(), "async-mongodb", "tide-sessions").await?,
            std::env::var("TIDE_SECRET")
                .expect(
                    "Please provide a TIDE_SECRET value of at \
                         least 32 bytes in order to run this example",
                )
                .as_bytes(),
        ));
        let substate = app.state().clone();
        app.at("/").nest(functions::handler(substate));
        println!("      Running server on: http://localhost:{}/", port);
        app.listen(address).await?;
        Ok(())
    })
}

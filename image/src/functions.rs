use async_std::stream::StreamExt;
use serde::{Deserialize, Serialize};
use wither::bson::{doc, oid::ObjectId};
use wither::Model;
// use wither::prelude::*;

// Define a model. Simple as deriving a few traits.
#[derive(Debug, Model, Serialize, Deserialize)]
pub struct City {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    #[model(index(index = "dsc", with(field = "country", index = "dsc")))]
    pub name: String,
    pub country: String,
    pub description: String,
}

pub fn handler(state: super::State) -> tide::Server<super::State> {
    
    let mut api = tide::with_state(state);

    api.at("/").get(|req: tide::Request<super::State>| async move {
        let state = &req.state();
        let db = &state.client.database("test");
        let mut cursor = City::find(&db.clone(), None, None).await?;
        let mut docs: Vec<City> = Vec::new();

        while let Some(city) = cursor.next().await {
            docs.push(city?);
        }
        Ok(serde_json::to_value(docs).unwrap())
    });

    api
}

pub fn default() -> tide::Server<()> {
    let mut api = tide::new();
    api.at("/").get(|req: tide::Request<()>| async move {
        // See request attributes
        // https://docs.rs/tide/0.15.0/tide/struct.Request.html
        println!("Request Method : {}", req.method());

        // See surf documentation
        // https://docs.rs/surf/2.1.0/surf/
        let uri = "https://httpbin.org/get";
        let string: String = surf::get(uri).recv_string().await?;
        Ok(string)
    });
    api
}

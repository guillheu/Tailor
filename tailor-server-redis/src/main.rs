mod dynamic_metadata_fields;

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_json;
use serde::{Serialize};
use serde_json::{Value, Map};
use handlebars::{Handlebars};
use rocket::request::{FromRequest, Request, Outcome};
use rocket::fs::{FileServer};
use rocket::{Config};
use rocket::response::{content};

use std::net::{IpAddr, Ipv4Addr};
use std::fs;
use std::collections::HashMap;
use std::io::{stdout, Write};

#[derive(Debug, Serialize)]
struct DynamicMetadataFields{
    headers: HashMap<String, String>,
}



#[rocket::async_trait]
impl<'r> FromRequest<'r> for DynamicMetadataFields {
    type Error = ();
    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let mut map = HashMap::new();
        for header in req.headers().iter() {
            map.insert(header.name().to_string(), header.value().to_string());
        }
        Outcome::Success(DynamicMetadataFields{headers: map})
    }
}



lazy_static::lazy_static! {
    // static ref METADATA: Value = serde_json::from_str(&METADATA_TEMPLATE_STR).unwrap();
    static ref METADATA_TEMPLATE_STR: String = fs::read_to_string("templates/metadata.json.hbs").unwrap();
    static ref HTML_TEMPLATE_STR: String = fs::read_to_string("templates/nft.html.hbs").unwrap();
    static ref HANDLEBARS: Handlebars<'static> = {
        let mut handlebars = Handlebars::new();

        //Registering helpers here

        handlebars.register_helper("get-timestamp-seconds", Box::new(dynamic_metadata_fields::get_remote_timestamp_seconds));
        handlebars.register_helper("static-string", Box::new(dynamic_metadata_fields::static_string_helper));
        assert!(handlebars.register_template_string("NFT", &*HTML_TEMPLATE_STR).is_ok());
        assert!(handlebars.register_template_string("metadata", &*METADATA_TEMPLATE_STR).is_ok());
        handlebars
    };
}

#[get("/metadata/<id>")]
fn get_metadata(id: &str, dynamic_metadata_fields: DynamicMetadataFields) -> content::Json<String> {

    let metadata = &serde_json::from_str::<Value>(&HANDLEBARS.render("metadata", &dynamic_metadata_fields)
        .unwrap())
        .unwrap()[id.parse::<usize>().unwrap()];      //TODO : change this logic so that we instead look up the tokenIDs until we find a match
    content::Json(metadata.to_string())
}


#[get("/?<id>")]
fn get_data(id: u32, request_data: DynamicMetadataFields) -> content::Html<String> {
    let metadata: Value = serde_json::from_str(&HANDLEBARS.render("metadata", &request_data).unwrap()).unwrap();
    let mut map = Map::new();
    for el in metadata.as_array().unwrap() {
        let mut attr = Map::new();
        for attribute in el["attributes"].as_array().unwrap() {
            attr.insert(attribute["trait_type"].as_str().unwrap().to_string(), attribute["value"].clone());
        }
        map.insert(el["tokenID"].as_u64().unwrap().to_string(), json!(attr));
    }
    let attributes = json!(map);

    // let r = ATTRIBUTES.to_string();
    let r = HANDLEBARS.render("NFT", &(attributes[id.to_string()])).unwrap();
    content::Html(r)
}

#[get("/")]
fn get_directions() -> content::Html<String> {
content::Html(r#"<h1>Greetings traveller</h1>
<p>If you're looking for metadata, try the /metadata/<id> route.</p>
<p>If you're looking for the NFT renders, the animations should be under /?id=<id>.</p>
<p>Otherwise, any static content (found in the static folder) should be directly accessible from the root.</p>
"#.to_string())
}

#[launch]
fn rocket() -> _ {
    let mut config = Config::release_default();
    config.port = 8080;
    config.address = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
    rocket::custom(config)
        .mount("/", routes![get_metadata, get_data, get_directions])
        .mount("/", FileServer::from("static"))
}

#[macro_use]
extern crate rocket;

use rocket::{
    get,
    serde::{json::Json, Serialize},
};
use rocket_okapi::{
    openapi, openapi_get_routes,
    swagger_ui::{make_swagger_ui, SwaggerUIConfig},
    JsonSchema,
};

#[derive(Serialize, JsonSchema)]
struct Response {
    reply: String,
}

#[openapi]
#[get("/")]
fn basic_controller() -> Json<Response> {
    Json(Response {
        reply: "This will be shown on the homepage".to_string(),
    })
}

fn get_docs() -> SwaggerUIConfig {
    SwaggerUIConfig {
        url: "/openapi.json".to_string(),
        ..Default::default()
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", openapi_get_routes![basic_controller])
        .mount("/swagger", make_swagger_ui(&get_docs()))
}

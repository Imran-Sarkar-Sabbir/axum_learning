use axum::{routing::get, Router};
mod hello_world;
mod mirror_body_string;

use hello_world::hello_world;
use mirror_body_string::mirror_body_string;

pub fn get_routes() -> Router<()> {
    Router::new()
    .route("/", get(hello_world))
    .route("/mirror-body-string", get(mirror_body_string))
}

use rocket::serde::json::Json;
use serde::Serialize;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Serialize)]
pub(crate) struct Version<'r> {
    name: &'r str,
    version: &'r str,
}

#[get("/")]
pub(crate) fn version_info() -> Json<Version<'static>> {
    Json(Version {
        name: "furo",
        version: VERSION,
    })
}

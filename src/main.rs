#[macro_use]
extern crate rocket;

mod version;

use crate::version::version_info;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![version_info])
}

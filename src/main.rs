use handler::{create_linkshorten_handler, create_linkshorten_render, redirect_handler, list_linkshorten_render};
use rocket_dyn_templates::Template;

mod handler;
mod model;

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    let app_data = model::AppState::new();

    rocket::build()
        .attach(Template::fairing())
        .manage(app_data)
        .mount(
            "/",
            routes![
                redirect_handler,
                create_linkshorten_handler,
                create_linkshorten_render,
                list_linkshorten_render
            ],
        )
}

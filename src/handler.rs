use crate::model::{AppState, Link, LinkInput};
use rand::RngCore;
use rocket::{
    form::Form,
    http,
    response,
    Response, State,
};
use rocket_dyn_templates::{context, Template};

pub struct Redirect {
    to: String,
}
impl Redirect {
    fn new(to: String) -> Redirect {
        Redirect { to }
    }
}
impl<'r> response::Responder<'r, 'static> for Redirect {
    fn respond_to(self, _request: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        Response::build()
            .status(http::Status::TemporaryRedirect)
            .header(http::Header::new("Location", self.to))
            .ok()
    }
}

#[get("/<id>")]
pub async fn redirect_handler<'a>(
    id: &str,
    data: &State<AppState>,
) -> Result<Redirect, (http::Status, String)> {
    let links = data.links.lock().expect("Failed to get link");
    let link = links.iter().find(|x| x.shorten == id);

    if let Some(link) = link {
        return Ok(Redirect::new(link.original.to_owned()));
    }

    Err((http::Status::NotFound, String::from("Link not found")))
}

#[post("/create", data = "<link_input>")]
pub async fn create_linkshorten_handler(
    link_input: Form<LinkInput>,
    data: &State<AppState>,
) -> Template {
    let link_input = link_input.into_inner();
    let mut links = data.links.lock().expect("Failed to get link");

    let mut bytes = [0u8; 5];
    rand::thread_rng().fill_bytes(&mut bytes);

    let shorten = hex::encode(&bytes);
    let link = Link {
        original: link_input.original.clone(),
        shorten: shorten.clone(),
    };

    links.push(link.clone());

    Template::render("create", context! { link: link.shorten })
}

#[get("/create")]
pub async fn create_linkshorten_render() -> Template {
    Template::render("create", context! {})
}

#[get("/list")]
pub async fn list_linkshorten_render(data: &State<AppState>) -> Template {
    let db = data.links.lock().expect("Failed to get link");
    let links = db.iter().map(|x| format!("{}: {}", &x.original, &x.shorten)).collect::<Vec<String>>();

    Template::render("list", context! { links })
}

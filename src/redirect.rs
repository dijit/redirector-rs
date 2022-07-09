use rocket::{
    response::status::NotFound,
    response::Redirect,
};

use crate::urls::URLS;

#[rocket::get("/<short>")]
pub async fn get_redirect(
    short: String,
) -> Result<Redirect, NotFound<String>> {
    let url = URLS.get::<str>(&short.to_string());
    match url {
        Some(u) => Ok(Redirect::permanent(u.to_string())),
        None => Err(NotFound("Not Found".to_string())),
    }
}

#[rocket::head("/<short>")]
pub async fn head_redirect(
    short: String,
) -> Result<Redirect, NotFound<String>> {
    get_redirect(short).await
}

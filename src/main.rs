extern crate rocket;

use rocket::shield::Frame;
use rocket::shield::XssFilter;
use rocket::shield::Hsts;

#[rocket::main]
async fn main() {
    let figment = rocket::Config::figment().merge(("ident", false));

    let _ = rocket::custom(figment)
        .attach(
        rocket::shield::Shield::default()
                .enable(Hsts::default())
                .enable(XssFilter::Enable)
                .enable(Frame::SameOrigin)
        )
        .attach(url::backend::stage())
        .launch()
        .await;
}

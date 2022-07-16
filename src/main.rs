#[macro_use]
extern crate rocket;

#[rocket::main]
async fn main() {
    let figment = rocket::Config::figment()
        .merge(("ident", false));

    let _ = rocket::custom(figment)
        .mount("/",
           routes![
               url::backend::submit,
               url::backend::head_redirect,
               url::backend::get_redirect,
           ],
        )
        .launch().await;
}

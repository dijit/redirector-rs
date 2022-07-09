#[macro_use]
extern crate rocket;

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/",
           routes![
                url::redirect::head_redirect,
                url::redirect::get_redirect,
           ],
        )
        .launch().await;
}
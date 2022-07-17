extern crate rocket;

#[rocket::main]
async fn main() {
    let figment = rocket::Config::figment().merge(("ident", false));

    let _ = rocket::custom(figment)
        .attach(url::backend::stage())
        .launch()
        .await;
}

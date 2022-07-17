/* http status codes and headers */
use rocket::{
    form::{Form, FromForm},
    get,
    http::ContentType,
    response::status::{Created, NoContent, NotFound},
    response::Redirect,
};

/* build a database backend */
use rocket::fairing::AdHoc;
use rocket::routes;

use rocket_db_pools::sqlx::{self, Row};
use rocket_db_pools::{Connection, Database};
use rocket_include_static_resources::{static_resources_initializer, static_response_handler};

#[derive(FromForm)]
pub struct Submission<'r> {
    #[field(validate = len(1..35))]
    r#short: &'r str,
    r#destination: &'r str,
}

#[rocket::post(
    "/submit",
    format = "application/x-www-form-urlencoded",
    data = "<submission>"
)]
pub async fn submit(
    mut db: Connection<Db>,
    submission: Form<Submission<'_>>,
) -> Result<(ContentType, Created<String>)> {
    sqlx::query(
        "INSERT INTO urls (short, destination) \
                        VALUES ($1, $2) \
                        ON CONFLICT ON CONSTRAINT urls_pkey DO \
                        UPDATE SET destination = $2",
    )
    .bind(submission.short)
    .bind(submission.destination)
    .execute(&mut *db)
    .await?;

    Ok((
        ContentType::HTML,
        Created::new("/").body(format!(
            r#"<title>Inserted</title>
<link rel="stylesheet" media="all" href="insert.css" />
<div class="segment">
    <h1>Congratulations: Added</h1>
<ul><b><u>{}</u></b></ul>
<img src="/favicon-16.png" alt="->" style="width:32px;height:32px;filter:invert(40%);">
<ul><b><u>{}</u></b></ul>
</div>"#,
            submission.short, submission.destination
        ))
    ))
}

fn not_found(req: &String) -> (ContentType, String) {
    (
        ContentType::HTML,
        format!(
            r#"<title>Insert</title>
<link rel="stylesheet" media="all" href="insert.css" />
<div class="segment">
    <h1>Not Found: Perhaps Add it?</h1>
    <form action="/submit" method="post">
    <label>
        <input
            type="short"
            name="short"
            placeholder="slug_here"
            required="required"
            value="{}"
        />
    </label>
    <label>
        <input
            type="url"
            name="destination"
            placeholder="http://example.com"
            required="required"
        />
    </label>
        <button type="submit" class="red">
            <i class="icon ion-arrow-forward-circle-outline"></i>
            Submit
        </button>
    </form>
</div>"#,
            req
        ),
    )
}

#[rocket::get("/<short>")]
pub async fn get_redirect(
    mut db: Connection<Db>,
    short: String,
) -> Result<Redirect, NotFound<(ContentType, String)>> {
    let url = sqlx::query(
        "SELECT destination FROM urls \
                WHERE short = $1",
    )
    .bind(&short)
    .fetch_optional(&mut *db)
    .await
    .expect("SQL Error");

    match &url {
        Some(u) => {
            let url_unwrapped: String = u.get(0);
            Ok(Redirect::permanent(url_unwrapped.to_string()))
        }
        None => Err(NotFound(not_found(&short))),
    }
}

#[rocket::get("/")]
async fn list(_db: Connection<Db>) -> (ContentType, String) {
    not_found(&"".to_string())
}

#[rocket::head("/<short>")]
pub async fn head_redirect(
    db: Connection<Db>,
    short: String,
) -> Result<Redirect, NotFound<(ContentType, String)>> {
    get_redirect(db, short).await
}

#[rocket::delete("/<short>")]
pub async fn delete(mut db: Connection<Db>, short: String) -> NoContent {
    sqlx::query(
        "DELETE FROM urls \
                WHERE short = $1",
    )
    .bind(&short)
    .execute(&mut *db)
    .await
    .expect("Error executing SQL");
    NoContent
}

#[derive(Database)]
#[database("url_db")]
pub struct Db(sqlx::PgPool);

type Result<T, E = rocket::response::Debug<sqlx::Error>> = std::result::Result<T, E>;

/* FIXME: rocket_db_pools SQLx does not allow enabling the "migrations" macro feature
async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    match Db::fetch(&rocket) {
        Some(db) => match sqlx::migrate!("db/migrations").run(&**db).await {
            Ok(_) => Ok(rocket),
            Err(e) => {
                error!("Failed to initialize SQLx database: {}", e);
                Err(rocket)
            }
        }
        None => Err(rocket),
    }
}
*/

static_response_handler! {
    "/favicon.ico" => favicon => "favicon",
    "/favicon-16.png" => favicon_png => "favicon-png",
    "/insert.css" => insert_css => "insert-css",
    "/insert.html" => insert_html => "insert-html",
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("SQLx Stage", |rocket| async {
        rocket
            .attach(Db::init())
            /* FIXME: rocket_db_pools SQLx does not allow enabling the "migrations" macro feature
            .attach(AdHoc::try_on_ignite("SQLx Migrations", run_migrations))
             */
            .attach(
                static_resources_initializer!(
                    "favicon" => "static/favicon.ico",
                    "favicon-png" => "static/favicon-16.png",
                    "insert-css" => "static/insert.css",
                    "insert-html" => "static/insert.html",
                )
            )
            .mount(
                "/",
                routes![
                    favicon,
                    favicon_png,
                    list,
                    submit,
                    get_redirect,
                    head_redirect,
                    delete,
                    insert_css
                ],
            )
    })
}

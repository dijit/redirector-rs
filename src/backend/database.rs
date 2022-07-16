//mod cockroach;

use rocket::{
    response::status::NotFound,
    response::Redirect,
    http::ContentType,
    form::{Form, FromForm},
};


#[derive(FromForm)]
pub struct Submission<'r> {
    #[field(validate = len(1..))]
    r#short: &'r str,
    r#destination: &'r str,
}

#[rocket::post("/submit", format = "application/x-www-form-urlencoded", data = "<submission>")]
pub async fn submit(
    submission: Form<Submission<'_>>
) -> String {
    format!("{} -> {}", submission.short, submission.destination)
}

fn not_found(req: &String) -> (ContentType, String) {
    (
        ContentType::HTML,
        format!(r#"
<form action="/submit" method="post">
<input type="short" id="short" name="short" value="{}">
<input type="url" id="destination" name="destination">
<input type="submit" value="Submit">
</form>
        "#, req)
    )
}

#[rocket::get("/<short>")]
pub async fn get_redirect(
    short: String,
) -> Result<Redirect, NotFound<(ContentType, String)>> {

    Err(NotFound(
        not_found(&short)
    ))

/*
    let url = None;
    match url {
        Some(u) => Ok(Redirect::permanent(u.to_string())),
        None => Err(
            NotFound(
                not_found(&short)
            )
        ),
    }
*/
}

#[rocket::head("/<short>")]
pub async fn head_redirect(
    short: String,
) -> Result<Redirect, NotFound<(ContentType, String)>> {
    get_redirect(short).await
}

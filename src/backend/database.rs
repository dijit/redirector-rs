mod cockroach;

use rocket::form::{Form, FromForm};

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
    "Not implemented".to_string()
}

fn not_found_form(req: &String) -> (ContentType, String) {
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

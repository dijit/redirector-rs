/* jinja rendering */
use askama::Template;

use rocket::{
    get, head, post,
    routes,
    http::ContentType,
    fairing::AdHoc,
    response::{
        Redirect,
        status::NotFound,
    },
};

use rocket_include_static_resources::{static_resources_initializer, static_response_handler};

fn not_found(req: &String) -> (ContentType, String) {
    //// Backend error message for config file does not include the possibility of submitting URL
    (ContentType::Plain, format!("I couldn't find '{}' and I am running without a database backend. \
             Try something else?", req))
}

#[post("/submit")]
pub async fn submit(
) -> String {
    "Not implemented".to_string()
}

#[derive(Template)]
#[template(path = "list.html")]
struct ListTemplate<'a> {
    urls: &'a Vec<String>,
}

#[get("/")]
async fn landing() -> (ContentType, String) {
    let mut veccy: Vec<String> = Vec::new();
    for (x,_) in URLS.iter() {
        veccy.push(x.to_string());
    }
    let out: ListTemplate = ListTemplate{urls: &veccy};
    (ContentType::HTML, out.render().unwrap())
}

#[get("/<short>")]
pub async fn get_redirect(
    short: String,
) -> Result<Redirect, NotFound<(ContentType, String)>> {
    let url = URLS.get::<str>(&short.to_string());
    match url {
        Some(u) => Ok(Redirect::permanent(u.to_string())),
        None => Err(
            NotFound(
                not_found(&short)
            )
        ),
    }
}

#[head("/<short>")]
pub async fn head_redirect(
    short: String,
) -> Result<Redirect, NotFound<(ContentType, String)>> {
    get_redirect(short).await
}



use std::collections::HashMap;
use toml::Value;
use toml::value::Table;

use lazy_static::lazy_static;

lazy_static! {
    //pub static ref URLS: HashMap<&'static str, &'static str> = {
    pub static ref URLS: HashMap<String, String> = {
        load_toml().unwrap()
    };
}

fn load_toml_to_string(file: Option<String>) -> Result<Table, ()> {
    let toml_file: String;
    match file {
        Some(f) => {
            toml_file = f.to_string();
        }
        None => {
            toml_file = "./urls.toml".to_string();
        }
    }
    let toml_content = std::fs::read_to_string(toml_file).unwrap();
    let url_info: Value = toml::from_str(&toml_content.to_string())
        .expect("Unable to load toml string");
    let ret = url_info["urls"].as_table().unwrap();
    Ok(Table::from(ret.to_owned()))
}

fn load_toml_str_to_hashmap(toml_str: Table) -> Result<HashMap<String, String>, ()> {
    let mut map: HashMap<String, String> = HashMap::new();
    for (k, v) in toml_str
    {
        map.insert(k, v.as_str().expect("Value for item was not a string").to_string());
    }
    Ok(map)
}

pub fn load_toml() -> Result<HashMap<String, String>, ()> {
    //FIXME: get toml from env
    let stringy = load_toml_to_string(None).unwrap();
    load_toml_str_to_hashmap(stringy)
}

static_response_handler! {
    "/favicon.ico" => favicon => "favicon",
    "/favicon-16.png" => favicon_png => "favicon-png",
    "/list.css" => list_css => "list-css",
    "/list.js" => list_js => "list-js",
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("attaching config_file routes", |rocket| async {
        rocket
            .attach(static_resources_initializer!(
                "favicon" => "static/favicon.ico",
                "favicon-png" => "static/favicon-16.png",
                "list-css" => "static/list.css",
                "list-js" => "static/list.js",
            ))
            .mount("/", routes![
                landing,
                list_css,
                list_js,
                favicon,
                favicon_png,
                submit,
                get_redirect,
                head_redirect
            ]
            )
    })
}
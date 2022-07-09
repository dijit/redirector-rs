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
    dbg!(&ret);
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
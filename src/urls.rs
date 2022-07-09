use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref URLS: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("handbook", "https://google.com");
        map.insert("darkscience", "https://darkscience.net");
        map
    };
}
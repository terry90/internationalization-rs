use glob::glob;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::sync::RwLock;

type Key = String;
type Locale = String;
type Value = String;

lazy_static! {
    pub static ref TR: RwLock<HashMap<Key, HashMap<Locale, Value>>> = RwLock::new(HashMap::new());
}

pub fn read_files(pattern: &str) -> Vec<String> {
    let mut contents = Vec::new();
    for entry in glob(pattern).expect("Failed to read glob pattern") {
        let file = File::open(entry.unwrap()).expect("Failed to open the file");
        let mut reader = BufReader::new(file);
        let mut content = String::new();
        reader
            .read_to_string(&mut content)
            .expect("Failed to read the file");
        contents.push(content);
    }
    contents
}

pub fn load_i18n(content: String) {
    let res: HashMap<String, HashMap<String, String>> =
        serde_json::from_str(&content).expect("Cannot parse I18n file");
    TR.write().unwrap().extend(res);
}

/// Translates by key
///
/// # Panics
///
/// If a key is missing, the code will panic
/// If a locale is not present for the key, ths will also panic
///
/// # Example
/// ```no-run
/// use internationalization::t;
///
/// fn main() {
///     init_i18n!("locales/*.json", "fr", "en");
///     
///     let res = t("err.not_allowed");
///     assert_eq!("You are not allowed to do this", res);
/// }
/// ```
pub fn t(key: &str, locale: &str) -> String {
    match TR.read().unwrap().get(key) {
        Some(trs) => match trs.get(locale) {
            Some(value) => value.to_owned(),
            None => panic!("Missing language ({}) for key: {}", locale, key),
        },
        None => panic!("Missing key: {}", key),
    }
}

use glob::glob;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

type Key = String;
type Locale = String;
type Value = String;
type Translations = HashMap<Key, HashMap<Locale, Value>>;

fn read_locales() -> Translations {
    let mut translations: Translations = HashMap::new();

    let build_directory = std::env::var("PWD").unwrap();
    let locales = format!("{}/**/locales/**/*.json", build_directory);
    println!("Reading {}", &locales);

    for entry in glob(&locales).expect("Failed to read glob pattern") {
        let entry = entry.unwrap();
        println!("cargo:rerun-if-changed={}", entry.display());
        let file = File::open(entry).expect("Failed to open the file");
        let mut reader = std::io::BufReader::new(file);
        let mut content = String::new();
        reader
            .read_to_string(&mut content)
            .expect("Failed to read the file");
        let res: HashMap<String, HashMap<String, String>> =
            serde_json::from_str(&content).expect("Cannot parse locale file");
        translations.extend(res);
    }
    translations
}

fn generate_code(translations: Translations) -> proc_macro2::TokenStream {
    let mut branches = Vec::<TokenStream>::new();

    for (key, trs) in translations {
        let mut langs = Vec::<TokenStream>::new();
        for (lang, tr) in trs {
            let l = quote! {
                #lang => #tr,
            };
            langs.push(l)
        }
        let branch = quote! {
            (#key, $lang:expr) => {
                match $lang.as_ref() {
                    #(#langs)*
                    e => panic!("Missing language: {}", e)
                }
            };
        };
        branches.push(branch);
    }

    quote! {
        #[macro_export]
        macro_rules! t {
            #(#branches)*
            ($key:expr, $lang:expr) => {
                compile_error!("Missing translation");
            }
        }
    }
}

fn write_code(code: TokenStream) {
    let dest = std::env::var("OUT_DIR").unwrap();
    let mut output = File::create(&std::path::Path::new(&dest).join("i18n.rs")).unwrap();
    output
        .write(code.to_string().as_bytes())
        .expect("Cannot write generated i18n code");
}

fn main() {
    let translations = read_locales();
    let code = generate_code(translations);
    println!("{}", &code);
    write_code(code);
}

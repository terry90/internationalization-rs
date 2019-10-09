//! # Internationalization

//! [![Crates.io Version](https://img.shields.io/crates/v/internationalization.svg)](https://crates.io/crates/internationalization)
//! ![LICENSE](https://img.shields.io/crates/l/internationalization)
//! [![Coverage Status](https://coveralls.io/repos/github/terry90/internationalization-rs/badge.svg?branch=master)](https://coveralls.io/github/terry90/internationalization-rs?branch=master)
//! [![Build Status](https://travis-ci.org/terry90/internationalization-rs.svg?branch=master)](https://travis-ci.org/terry90/internationalization-rs)
//!
//! An simple compile time i18n implementation in Rust.
//! It throws a compilation error if the translation key is not present, but since the `lang` argument is dynamic it will panic if the language has not been added for the matching key.

//! > API documentation [https://crates.io/crates/internationalization](https://crates.io/crates/internationalization)

//! ## Usage

//! Have a `locales/` folder somewhere in your app, root, src, anywhere. with `.json` files, nested in folders or not.
//! It uses a glob pattern: `**/locales/**/*.json` to match your translation files.

//! the files must look like this:

//! ```json
//! {
//!   "err.answer.all": {
//!     "fr": "Échec lors de la récupération des réponses",
//!     "en": "Failed to retrieve answers"
//!   },
//!   "err.answer.delete.failed": {
//!     "fr": "Échec lors de la suppression de la réponse",
//!     "en": "Failed to delete answer"
//!   }
//! }
//! ```

//! Any number of languages can be added, but you should provide them for everything since it will panic if a language is not found when queried for a key.

//! In your app, just call the `t!` macro

//! ```rust
//! use internationalization::t;
//!
//! fn main() {
//!     let lang = "en";
//!     let res = t!("err.not_allowed", lang);

//!     assert_eq!("You are not allowed to do this", res);
//! }
//! ```
//!
//! If the key is missing, your code will not compile

//! ```rust,compile_fail
//! use internationalization::t;
//!
//! fn main() {
//!     let lang = "en";
//!     let res = t!("missing key", lang);

//!     // Code will not compile
//! }
//! ```

//! ## Installation

//! Internationalization is available on [crates.io](https://crates.io/crates/internationalization), include it in your `Cargo.toml`:

//! ```toml
//! [dependencies]
//! internationalization = "0.0.2"
//! ```

//! Then include it in your code like this:

//! ```rust,ignore
//! #[macro_use]
//! extern crate internationalization;
//! ```

//! Or use the macro where you want to use it:

//! ```rust
//! use internationalization::t;
//! ```

//! ## Note

//! Internationalization will not work if no `PWD` env var is set at compile time.

include!(concat!(env!("OUT_DIR"), "/i18n.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_translates() {
        assert_eq!(t!("key.test", "en"), "This is a test");
        assert_eq!(t!("key.test", "fr"), "C'est un test");
        assert_eq!(
            t!("err.not_allowed", "fr"),
            "Vous n'êtes pas autorisé à faire cela"
        );
        assert_eq!(
            t!("err.not_allowed", "en"),
            "You are not allowed to do this"
        );
    }
    #[test]
    #[should_panic]
    fn it_fails_to_translate() {
        assert_eq!(t!("key.test", "es"), "Hola");
    }
}

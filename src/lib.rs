//! # Internationalization
//! An simple i18n implementation in Rust.

//! > API documentation [https://crates.io/crates/internationalization](https://crates.io/crates/internationalization)

//! ## Usage
//!
//! ```
//! use internationalization::{init_i18n, t};
//!
//! fn main() {
//!     init_i18n!("locales/*.json", "fr", "en");
//!     
//!     let res = t("err.not_allowed");
//!     assert_eq!("You are not allowed to do this", res);
//! }
//! ```

pub mod i18n;
pub use i18n::t;

#[macro_export]
macro_rules! init_i18n {
    ( $path:expr, $( $lang:expr ),* ) => {
        use internationalization::i18n::{load_i18n, read_files};
        for content in read_files($path) {
            load_i18n(content)
        }
    };
}

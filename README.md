# Internationalization

An simple i18n implementation in Rust.

> API documentation [https://crates.io/crates/internationalization](https://crates.io/crates/internationalization)

## Usage

```rust
use internationalization::{init_i18n, t};
fn main() {
    init_i18n!("locales/*.json", "fr", "en");

    let res = t("err.not_allowed");
    assert_eq!("You are not allowed to do this", res);
}
```

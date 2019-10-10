# Locales

![LICENSE](https://img.shields.io/crates/l/locales)
[![Crates.io Version](https://img.shields.io/crates/v/locales.svg)](https://crates.io/crates/locales)
[![Coverage Status](https://coveralls.io/repos/github/terry90/internationalization-rs/badge.svg?branch=master)](https://coveralls.io/github/terry90/internationalization-rs?branch=master)
[![Build Status](https://travis-ci.org/terry90/internationalization-rs.svg?branch=master)](https://travis-ci.org/terry90/internationalization-rs)

An simple compile time i18n implementation in Rust.
It throws a compilation error if the translation key is not present, but since the `lang` argument is dynamic it will panic if the language has not been added for the matching key.

> API documentation [https://crates.io/crates/locales](https://crates.io/crates/locales)

## Usage

Have a `locales/` folder somewhere in your app, root, src, anywhere. with `.json` files, nested in folders or not.
It uses a glob pattern: `**/locales/**/*.json` to match your translation files.

the files must look like this:

```json
{
  "err.user.not_found": {
    "fr": "Utilisateur introuvable: $email, $id",
    "en": "User not found: $email, $id"
  },
  "err.answer.all": {
    "fr": "Échec lors de la récupération des réponses",
    "en": "Failed to retrieve answers"
  },
  "err.answer.delete.failed": {
    "fr": "Échec lors de la suppression de la réponse",
    "en": "Failed to delete answer"
  }
}
```

Any number of languages can be added, but you should provide them for everything since it will panic if a language is not found when queried for a key.

In your app, just call the `t!` macro

```rust
use locales::t;

fn main() {
    let lang = "en";
    let res = t!("err.not_allowed", lang);

    assert_eq!("You are not allowed to do this", res);
}
```

You can use interpolation, any number of argument is OK but you should note that they have to be sorted alphabetically.
To use variables, call the `t!` macro like this:

```rust
use locales::t;

fn main() {
    let lang = "en";
    let res = t!("err.user.not_found", email: "me@localhost", id: "1", lang);

    assert_eq!("User not found: me@localhost, ID: 1", res);
}
```

## Installation

Locales is available on [crates.io](https://crates.io/crates/locales), include it in your `Cargo.toml`:

```toml
[dependencies]
locales = "0.1.0"
```

Then include it in your code like this:

```rust
#[macro_use]
extern crate locales;
```

Or use the macro where you want to use it:

```rust
use locales::t;
```

## Note

Locales will not work if no `PWD` env var is set at compile time.

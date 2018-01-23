# rust-stackedconfig

Treat multiple nested config objects (e.g. JSON or YAML files) as a single
config object with precedence.

[![Latest Version](https://img.shields.io/crates/v/stackedconfig.svg)](https://crates.io/crates/stackedconfig)
[![Rust Documentation](https://img.shields.io/badge/api-rustdoc-blue.svg)](https://bheklilr.github.io/rust-stackedconfig/stackedconfig/)

## Why should I use this library?

You shouldn't, it isn't anywhere near done yet and I'm definitely not the best
at keeping up with side projects.  **This software is pre-alpha.**

## Why does this library exist?

This is a fun, small side-project for me to learn rust better, but my goal is
for it to be good enough eventually for someone else to use seriously.

## But why I want to use it?

Have you ever had the situation where your application had complex
configuration?  Something like this contrived example?

```yaml
notifications:
    email:
        address: user@example.com
        on_new_post: false
        on_new_message: true
appearance:
    theme: dark
    font:
        family: Arial
        size: 14pt
web:
    proxy:
        http: http://proxy.com
        https: https://proxy.com
    ssl_verify: true
    trusted_hosts:
        - crates.io
        - rust-lang.org
credentials:
    username: null
    token: null
```

And you wanted to have multiple config files, such as on the system level in
`$APPDIR/app_config.yaml`, on the user level at `$HOME/app_config.yaml`, and
on the project level at `$PROJECTDIR/app_config.yaml`.  Managing multiple config
files with fallback is a pain.  This library aims to make it easy to work with:

```rust
extern crate serde_json;
extern crate stackedconfig;

use serde_json::{Value, from_str};
use stackedconfig::{ConfigStack};

fn main() {
    // Normally would load these from disk
    let system_conf: Value = from_str(r#"
        {
            "notifications": {
                "email": {
                    "on_new_post": true,
                    "on_new_message": true
                }
            },
            "appearance":{
                "theme": "light",
                "font": {
                    "family": "Arial",
                    "size": "14pt"
                }
            }
            "web": {
                "proxy": {
                    "http": "http://proxy.com",
                    "https": "https://proxy.com"
                },
                "ssl_verify": true,
                "trusted_hosts": []
            },
            "credentials": {
                "username": null,
                "token": null
            }
        }
    "#)?;
    let user_conf: Value = from_str(r#"
        {
            "notifications": {
                "email": {
                    "address": "user@example.com"
                }
            },
            "appearance": {
                "theme": "dark"
            },
            "web": {
                "trusted_hosts": ["crates.io", "rust-lang.org"]
            },
            "credentials": {
                "username": "bheklilr",
                "token": "123456abcdef"
            }
        }
    "#)?;
    let stack = ConfigStack::new()
        .stack(system_conf)
        .stack(user_conf);
    // Nested access is available, separator can be customized to any character
    // with the `with_sep` method.
    println!("{}", stack.get("notifications/email/address")?.unwrap());
    // Calls to `get` return a `ConfigStack`, so sub-sections could be passed to
    // the relevant parts of your application
    println!("{}", stack.get("appearance")?.get("theme")?.unwrap());
    // The last config stacked takes precedence when doing lookups.
    // Values can be unwrapped from the stack for actual use later.
    println!("{}", stack.get("web/proxy/http")?.unwrap());
}
```

## Documentation

The docs are build using `rustdoc` and are hosted
[here](https://bheklilr.github.io/rust-stackedconfig/stackedconfig/).

## Formats supported

Currently this library supports the following formats, each implemented as an
optional feature:

* JSON via `serde_json`
* YAML via `serde_yaml`
* HJSON via `serde-hjson`
* BSON via `bson`

If you want to be able to support other formats, or your own custom type, just
implement the `stackedconfig::Gettable` trait.

## License

I put it under MIT, but honestly I really don't care what anyone does with this
code.  It is provided as-is.  If you find a bug, feel free to submit an issue,
or even better submit a PR fixing it.

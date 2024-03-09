# urlshortener-rs
[![Crates badge](https://img.shields.io/crates/v/urlshortener.svg)](https://crates.io/crates/urlshortener)
[![CI](https://github.com/iddm/urlshortener-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/iddm/urlshortener-rs/actions/workflows/ci.yml)
[![](https://docs.rs/urlshortener/badge.svg)](https://docs.rs/urlshortener)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)


A very simple urlshortener for Rust.

This library aims to implement as much URL shortener services as possible and to provide an interface as
minimal and simple as possible. For easing pain with dependency hell, the library provides request objects
since 0.9.0 version which can be used for performing requests via user http-client library.

## MSRV
The minimum supported rust version is bumped to 1.63 just because one of the dependencies.
The code itself should work fine with Rust version 1.46, and, perhaps, even lower versions.

## Implementations

Currently the following URL shorteners are implemented:

With authentication:

- `goo.gl`
- `bit.ly`
- `kutt.it` (supports self hosting)

Without authentication:

- `bn.gy`
- `is.gd`
- `v.gd`
- `bam.bz`
- `fifo.cc`
- `tiny.ph`
- `tny.im`
- `s.coop`
- `bmeo.org`
- `hmm.rs`
- `url-shortener.io`
- `biturl.top`

The following services are supported, but are discouraged from use, due to
restrictions such as rate limits:

- `tinyurl.com`
- `psbe.co`
- `rlu.ru`
- `sirbz.com`
- `hec.su`
- `abv8.me`
- `nowlinks.net`

## Usage **without** "client" feature

You can make a `Request` object without "client" feature only via provider functions:

```rust
extern crate urlshortener;

use urlshortener::providers::{Provider, self};

fn main() {
    let long_url = "https://google.com";
    let key = "MY_API_KEY";
    let req = providers::request(long_url, &Provider::GooGl { api_key: key.to_owned() });
    println!("A request object for shortening URL via GooGl: {:?}", req);
}
```

## Usage with "client" feature

Without authentication

```rust
extern crate urlshortener;

use urlshortener::client::UrlShortener;

fn main() {
    let us = UrlShortener::new().unwrap();
    let long_url = "https://google.com";
    println!("Short url for google: {:?}", us.try_generate(long_url, None));
}
```

With authentication (**Goo.Gl**)

```rust
extern crate urlshortener;

use urlshortener::{ client::UrlShortener, providers::Provider };

fn main() {
    let us = UrlShortener::new().unwrap();
    let long_url = "https://google.com";
    let key = "MY_API_KEY";
    println!("Short url for google: {:?}", us.generate(long_url, Provider::GooGl { api_key: key.to_owned() }));
}
```

Combined (**Goo.Gl** + **Is.Gd**)

```rust
extern crate urlshortener;

use urlshortener::{ client::UrlShortener, providers::Provider };

fn main() {    
    let us = UrlShortener::new().unwrap();
    let providers = vec![
        Provider::GooGl { api_key: "MY_API_KEY".to_owned() },
        Provider::IsGd,
    ];
    let long_url = "https://rust-lang.org";
    println!("Short url for google: {:?}", us.try_generate(long_url, Some(providers)));
}
```


## License

This project is [licensed under the MIT license](https://github.com/iddm/urlshortener-rs/blob/master/LICENSE).

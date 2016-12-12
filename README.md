# urlshortener-rs
[![](https://meritbadge.herokuapp.com/urlshortener)](https://crates.io/crates/urlshortener) [![](https://travis-ci.org/vityafx/urlshortener-rs.svg?branch=master)](https://travis-ci.org/vityafx/urlshortener-rs) [![](https://img.shields.io/badge/docs-online-2020ff.svg)](https://vityafx.github.io/urlshortener-rs/master/urlshortener/)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)


A very simple urlshortener for Rust.

This library aims to implement as much URL shortener services as possible and to provide an interface as
minimal and simple as possible.

## Implementations

Currently the following URL shorteners are implemented:

With authentication:

- `goo.gl`
- `bit.ly`

Without authentication:

- `bn.gy`
- `is.gd`
- `readability.com`
- `v.gd`
- `bam.bz`
- `fifo.cc`
- `tiny.ph`
- `tny.im`
- `s.coop`
- `bmeo.org`
- `hmm.rs`
- `url-shortener.io`

The following services are supported, but are discouraged from use, due to
restrictions such as rate limits:

- `tinyurl.com`
- `psbe.co`
- `rlu.ru`
- `sirbz.com`
- `hec.su`
- `abv8.me`
- `nowlinks.net`


## Usage

Without authentication

```rust
extern crate urlshortener;

use urlshortener::UrlShortener;

fn main() {
    let us = UrlShortener::new();
    let long_url = "https://google.com";
    println!("Short url for google: {:?}", us.try_generate(long_url, None));
}
```

With authentication (**Goo.Gl**)

```rust
extern crate urlshortener;

use urlshortener::{ UrlShortener, Provider };

fn main() {
    let us = UrlShortener::new();
    let long_url = "https://google.com";
    let key = "MY_API_KEY";
    println!("Short url for google: {:?}", us.generate(long_url, Provider::GooGl { api_key: key.to_owned() }));
}
```

Combined (**Goo.Gl** + **Is.Gd**)

```rust
extern crate urlshortener;

use urlshortener::{ UrlShortener, Provider };

fn main() {
    use urlshortener::UrlShortener;
    
    let us = UrlShortener::new();
    let providers = vec![
        Provider::GooGl { api_key: "MY_API_KEY".to_owned() },
        Provider::IsGd,
    ];
    let long_url = "https://rust-lang.org";
    let _short_url = 
    println!("Short url for google: {:?}", us.try_generate(long_url, Some(providers)));
}
```


## License

This project is [licensed under the MIT license](https://github.com/vityafx/urlshortener-rs/blob/master/LICENSE).

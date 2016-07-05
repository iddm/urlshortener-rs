# urlshortener-rs [![](https://meritbadge.herokuapp.com/urlshortener)](https://crates.io/crates/urlshortener) [![](https://travis-ci.org/vityafx/urlshortener-rs.svg?branch=master)](https://travis-ci.org/vityafx/urlshortener-rs) [![](https://img.shields.io/badge/docs-online-2020ff.svg)](https://vityafx.github.io/urlshortener-rs/master/urlshortener/) 


A very-very simple urlshortener for Rust.

This library aims to implement only url shorten services which does not require any authentication (Google, Bit.ly not in this list) and to provide interface as minimal and simpler as possible.

The reason of this as that users of such libraries might need only to get the shorten url instead of using the whole service with authentication. That is also a reason this library aims to have at least dependencies as possible.

At this moment 3 url-shorters are implemented: `is.gd`, `v.gd`, `bn.gy`.

## Usage
```rust
extern crate urlshortener;

use urlshortener::UrlShortener;

fn main() {
    let us = UrlShortener::new();
    println!("Short url for google: {:?}", us.try_get("http://google.com"));
}
```

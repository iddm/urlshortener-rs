# urlshortener-rs [![](https://meritbadge.herokuapp.com/urlshortener)](https://crates.io/crates/urlshortener) [![](https://travis-ci.org/vityafx/urlshortener-rs.svg?branch=master)](https://travis-ci.org/vityafx/urlshortener-rs)


A very-very simple urlshortener for Rust.

Currently it uses `is.gd` service.
It will be developed soon to use more services.

This library aims to implement only url shorten services which does not require any authentication (Google, Bit.ly not in this list) and to provide interface as minimal and simpler as possible.

The reason of this as that users of such libraries might need only to get the shorten url instead of using the whole service with authentication. That is also a reason this library aims to have only one dependency - `hyper`.

## Usage
```rust
extern crate urlshortener;

use urlshortener::get_short_url;

fn main() {
    println!("Short url for google: {:?}", get_short_url("http://google.com"));
}
```

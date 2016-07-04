# urlshortener-rs [![](https://meritbadge.herokuapp.com/urlshortener)](https://crates.io/crates/urlshortener) [![Build Status](https://travis-ci.org/vityafx/urlshortener-rs?branch=master)](https://travis-ci.org/vityafx/urlshortener-rs)


A very-very simple urlshortener for Rust.

Currently it uses `is.gd` service.
It will be developed soon to use more services.

## Usage
```
extern crate urlshortener;

use urlshortener::get_short_url;

fn main() {
    println!("Short url for google: {:?}", get_short_url("http://google.com"));
}
```

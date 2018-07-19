//! Generate a short URL by specifying which provider to use.

extern crate urlshortener;

use urlshortener::{client::UrlShortener, providers::Provider};

fn main() {
    let long_url = "https://doc.rust-lang.org/std/";

    let us = UrlShortener::new().unwrap();
    let short_url = us.generate(long_url, &Provider::IsGd);

    println!("{:?}", short_url);
}

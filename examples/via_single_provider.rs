//! Generate a short URL by specifying which provider to use.

extern crate urlshortener;

use urlshortener::{Provider, UrlShortener};

fn main() {
    let long_url = "https://doc.rust-lang.org/std/";

    let us = UrlShortener::new();
    let short_url = us.generate_via_provider(long_url, Provider::IsGd);

    println!("{:?}", short_url);
}

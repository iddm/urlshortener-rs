//! Generate a short URL by specifying which provider to use.

extern crate urlshortener;

use urlshortener::client::UrlShortener;
use urlshortener::providers::Provider;

fn main() {
    let long_url = "https://doc.rust-lang.org/std/";

    let us = UrlShortener::new().unwrap();
    let key = "MY_API_KEY";
    let short_url = us.generate(
        long_url,
        &Provider::GooGl {
            api_key: key.to_owned(),
        },
    );

    println!("{:?}", short_url);
}

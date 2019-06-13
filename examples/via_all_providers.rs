//! Generate a short URL by attempting all providers one by one, until a short
//! URL has been successfully generated.

extern crate urlshortener;

use urlshortener::client::UrlShortener;

fn main() {
    let long_url = "https://doc.rust-lang.org/std/";

    let us = UrlShortener::new().unwrap();
    let short_url = us.generate(long_url, &urlshortener::providers::Provider::IsGd);

    println!("{:?}", short_url);
}

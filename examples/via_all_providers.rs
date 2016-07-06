//! Generate a short URL by attempting all providers one by one, until a short
//! URL has been successfully generated.

extern crate urlshortener;

use urlshortener::UrlShortener;

fn main() {
    let long_url = "https://doc.rust-lang.org/std/";

    let us = UrlShortener::new();
    let short_url = us.try_generate(long_url);

    println!("{:?}", short_url);
}

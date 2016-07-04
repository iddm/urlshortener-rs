extern crate hyper;

use std::io::{
    Read,
    Error,
    ErrorKind,
};

pub fn get_short_url(url: &str) -> Result<String, Error> {
    let client = hyper::Client::new();
    let service_url = &format!("http://is.gd/create.php?format=simple&url={}", url);
    let mut response = client.get(service_url).send().unwrap();
    if response.status.is_success() {
        let mut short_url = String::new();
        if try!(response.read_to_string(&mut short_url)) > 0 {
            return Ok(short_url);
        }
    }
    return Err(Error::new(ErrorKind::Other, "Something went wrong"));
}

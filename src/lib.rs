//! A very-very easy library for retrieving short urls.
#![warn(missing_docs)]

#[macro_use]
extern crate log;
extern crate hyper;

use std::io::{
    Read,
    Error,
    ErrorKind,
};
use std::time::Duration;

pub mod providers;
use providers::{
    Provider,
    IsGdProvider,
    VGdProvider,
    BnGyProvider,
};
    
/// Url shortener - the way to retrieve a short url.
pub struct UrlShortener {
    client: hyper::Client,
    providers: Vec<Box<Provider>>,
}
impl UrlShortener {
    /// Creates new `UrlShortener`.
    pub fn new() -> UrlShortener {
        let mut client = hyper::Client::new();
        client.set_read_timeout(Some(Duration::from_secs(3)));

        UrlShortener {
            client: client,
            providers: vec![
                Box::new(IsGdProvider),
                Box::new(VGdProvider),
                Box::new(BnGyProvider),
            ],
        }
    }

    /// Returns a reference for provider by looking up it's name.
    ///
    /// # Example
    /// ```ignore
    /// let us = UrlShortener::new();
    /// let long_url = "http://google.com";
    /// // Getting the `is.gd` provider and use it.
    /// if let Some(p) = us.get_provider_by_name("is.gd") {
    ///     let short_url = us.get_with_provider(long_url, p);
    /// }
    /// ```
    pub fn get_provider_by_name(&self, name: &str) -> Option<&Provider> {
        for p in &self.providers {
            if p.name() == name {
                return Some(&**p)
            }
        }
        None
    }

    /// Tries to get a short url from all defined providers.
    /// First it attempts to use one and if it fails - choose another.
    /// # Example
    /// ```ignore
    /// extern crate urlshortener;
    /// 
    /// use urlshortener::UrlShortener;
    ///
    /// fn main() {
    ///     let us = UrlShortener::new();
    ///     println!("Short url for google: {:?}", us.try_get("http://google.com"));
    /// }
    /// ```
    pub fn try_get(&self, url: &str) -> Result<String, Error> {
        for p in &self.providers {
            let res = self.get_with_provider(url, &**p);
            if let Ok(s) = res {
                return Ok(s) 
            } else {
                warn!("Failed to get short link from the service [{}]: {}",
                      p.name(),
                      res.unwrap_err());
            }
        }
        error!("Failed to get short link from any service");
        Err(Error::new(ErrorKind::Other, "Failed to get short link from any service"))
    }

    /// Attempts to get a short url using specified provider.
    ///
    /// ```ignore
    /// let us = UrlShortener::new();
    /// let long_url = "http://google.com";
    /// let short_url = us.get_with_provider(long_url, &urlshortener::IsGdProvider);
    /// ```
    pub fn get_with_provider(&self, url: &str, provider: &Provider) -> Result<String, Error> {
        let mut response = provider.prepare_request(url, &self.client).send().unwrap();
        if response.status.is_success() {
            let mut short_url = String::new();
            if try!(response.read_to_string(&mut short_url)) > 0 {
                if let Some(s) = provider.parse_response(&short_url) {
                    return Ok(s)
                } else {
                    return Err(Error::new(ErrorKind::Other, "Decode error"))
                }
            }
        }
        Err(Error::new(ErrorKind::Other, "Service is unavailable"))
    }
}

//! A very-very easy library for retrieving short urls.
#![warn(missing_docs)]

#[macro_use]
extern crate log;
extern crate hyper;

pub mod providers;

use std::io::{
    Read,
    Error,
    ErrorKind,
};
use std::time::Duration;
pub use providers::{Provider, providers};
use providers::{parse, prepare};
use hyper::Client;

/// Url shortener - the way to retrieve a short url.
pub struct UrlShortener {
    client: Client,
}
impl UrlShortener {
    /// Creates new `UrlShortener`.
    pub fn new() -> UrlShortener {
        let mut client = hyper::Client::new();
        client.set_read_timeout(Some(Duration::from_secs(3)));

        UrlShortener {
            client: client,
        }
    }

    /// Try to generate a short URL from each provider, iterating over each
    /// provider until a short URL is successfully generated.
    ///
    /// # Example
    ///
    /// ```ignore
    /// extern crate urlshortener;
    ///
    /// use urlshortener::UrlShortener;
    ///
    /// fn main() {
    ///     let us = UrlShortener::new();
    ///     println!("Short url for google: {:?}", us.generate("http://google.com"));
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an `Error<ErrorKind::Other>` if there is an error generating a
    /// short URL from all providers.
    pub fn generate(&self, url: &str) -> Result<String, Error> {
        let mut providers = providers();

        let x = 0usize;

        loop {
            if providers.len() == 0 {
                break
            }

            // This would normally have the potential to panic, except that a
            // check to ensure there is an element at this index is performed.
            let provider = providers.remove(x);
            let res = self.generate_via_provider(url, provider);

            if let Ok(s) = res {
                return Ok(s)
            } else {
                warn!("Failed to get short link from service: {}",
                      res.unwrap_err());
            }
        }
        error!("Failed to get short link from any service");
        Err(Error::new(ErrorKind::Other, "Failed to get short link from any service"))
    }

    /// Attempts to get a short URL using the specified provider.
    ///
    /// ```ignore
    /// use urlshortener::Provider;
    /// let us = UrlShortener::new();
    /// let long_url = "http://google.com";
    /// let short_url = us.generate_via_provider(long_url, Provider::IsGd);
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an `Error<<ErrorKind::Other>` if there is an error generating a
    /// short URL from the given provider due to either:
    ///
    /// a. a decode error;
    /// b. the service being unavailable
    pub fn generate_via_provider(&self, url: &str, provider: Provider) -> Result<String, Error> {
        let mut response = prepare(url, &self.client, provider)
            .send()
            .unwrap();

        if response.status.is_success() {
            let mut short_url = String::new();
            if try!(response.read_to_string(&mut short_url)) > 0 {
                if let Some(s) = parse(&short_url, provider) {
                    return Ok(s)
                } else {
                    return Err(Error::new(ErrorKind::Other, "Decode error"))
                }
            }
        }
        Err(Error::new(ErrorKind::Other, "Service is unavailable"))
    }
}

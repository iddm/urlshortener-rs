//! # urlshortener
//!
//! An easy library for retrieving short urls.
//!
//! ## Installation
//!
//! Add the following dependency to your project's `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! urlshortener = "0.4"
//! ```
//!
//! And add this to your root file:
//!
//! ```no_run
//! extern crate urlshortener;
//! ```
//!
//! ## Usage
//!
//! Creating a short URL via a specified provider is very simple:
//!
//! ```no_run
//! use urlshortener::{Provider, UrlShortener};
//!
//! let us = UrlShortener::new();
//! let short_url = us.generate("https://my-long-url.com", Provider::IsGd);
//! assert!(short_url.is_ok());
//! ```
//!
//! Or attempting all URL shorteners until one is successfully generated:
//!
//! ```no_run
//! use urlshortener::UrlShortener;
//!
//! let us = UrlShortener::new();
//! let short_url = us.try_generate("https://my-long-url.com");
//! assert!(short_url.is_ok());
//! ```

#[macro_use]
extern crate log;
extern crate hyper;

pub mod providers;

pub use providers::{Provider, providers};

use providers::{parse, request};
use hyper::Client;
use std::io::{ErrorKind, Error, Read};
use std::time::Duration;

/// Url shortener - the way to retrieve a short url.
pub struct UrlShortener {
    client: Client,
}

impl UrlShortener {
    /// Creates new `UrlShortener` with default (3 seconds) timeout.
    pub fn new() -> UrlShortener {
        UrlShortener::with_timeout(3)
    }

    /// Creates new `UrlShortener` with custom read timeout.
    pub fn with_timeout(seconds: u64) -> UrlShortener {
        let mut client = hyper::Client::new();
        client.set_read_timeout(Some(Duration::from_secs(seconds)));

        UrlShortener {
            client: client,
        }
    }

    /// Try to generate a short URL from each provider, iterating over each
    /// provider until a short URL is successfully generated.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use urlshortener::UrlShortener;
    ///
    /// let us = UrlShortener::new();
    /// let long_url = "https://rust-lang.org";
    /// let short_url = us.try_generate(long_url);
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an `Error<ErrorKind::Other>` if there is an error generating a
    /// short URL from all providers.
    pub fn try_generate(&self, url: &str) -> Result<String, Error> {
        let mut providers = providers();

        let x = 0usize;

        loop {
            if providers.len() == 0 {
                break
            }

            // This would normally have the potential to panic, except that a
            // check to ensure there is an element at this index is performed.
            let provider = providers.remove(x);
            let res = self.generate(url, provider);

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
    /// # Examples
    ///
    /// ```no_run
    /// use urlshortener::{Provider, UrlShortener};
    ///
    /// let us = UrlShortener::new();
    /// let long_url = "http://rust-lang.org";
    /// let short_url = us.generate(long_url, Provider::IsGd);
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an `std::io::Error` if there is an error generating a
    /// short URL from the given provider due to either:
    ///
    /// a. a decode error (ErrorKind::Other);
    /// b. the service being unavailable (ErrorKind::ConnectionAborted)
    pub fn generate(&self, url: &str, provider: Provider) -> Result<String, Error> {
        let response_opt = request(url, &self.client, provider);

        if let Some(mut response) = response_opt {
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
        }
        Err(Error::new(ErrorKind::ConnectionAborted, "Service is unavailable"))
    }
}

#[cfg(test)]
mod tests {
    use std::io::ErrorKind;

    #[test]
    fn providers() {
        let us = ::UrlShortener::with_timeout(5);
        let url = "http://stackoverflow.com";

        for provider in ::providers() {
            if let Some(err) = us.generate(url, provider).err() {
                assert!(err.kind() == ErrorKind::ConnectionAborted);
            }
        }
    }
}

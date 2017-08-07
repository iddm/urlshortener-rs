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
//! urlshortener = "0.7"
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
//! let us = UrlShortener::new().unwrap();
//! let short_url = us.generate("https://my-long-url.com", &Provider::IsGd);
//! assert!(short_url.is_ok());
//! ```
//!
//! Or attempting all URL shorteners until one is successfully generated:
//!
//! ```no_run
//! use urlshortener::UrlShortener;
//!
//! let us = UrlShortener::new().unwrap();
//! let short_url = us.try_generate("https://my-long-url.com", None);
//! assert!(short_url.is_ok());
//! ```
//! In order to use service with authentication use the appropriate provider directly:
//!
//! ```no_run
//! use urlshortener::{ UrlShortener, Provider };
//!
//! let us = UrlShortener::new().unwrap();
//! let key = "MY_API_KEY";
//! let short_url = us.generate("https://my-long-url.com", &Provider::GooGl { api_key:
//! key.to_owned() });
//! assert!(short_url.is_ok());
//! ```

#[macro_use]
extern crate log;
extern crate reqwest;
extern crate url;

mod providers;

pub use providers::{Provider, PROVIDERS};

use providers::{parse, request};
use reqwest::Client;
use std::io::{Error, ErrorKind, Read};
use std::time::Duration;


/// Url shortener: the way to retrieve a short url.
#[derive(Debug)]
pub struct UrlShortener {
    client: Client,
}

impl UrlShortener {
    /// Creates new `UrlShortener` with default (3 seconds) timeout.
    pub fn new() -> Result<UrlShortener, reqwest::Error> {
        UrlShortener::with_timeout(3)
    }

    /// Creates new `UrlShortener` with custom read timeout.
    pub fn with_timeout(seconds: u64) -> Result<UrlShortener, reqwest::Error> {
        let client = reqwest::ClientBuilder::new()?
            .timeout(Duration::from_secs(seconds))
            .build()?;

        Ok(UrlShortener { client: client })
    }

    /// Try to generate a short URL from each provider, iterating over each
    /// provider until a short URL is successfully generated.
    /// If you wish to override the list or providers or their priority,
    /// provide your own list of providers as second argument.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use urlshortener::UrlShortener;
    ///
    /// let us = UrlShortener::new().unwrap();
    /// let long_url = "https://rust-lang.org";
    /// let _short_url = us.try_generate(long_url, None);
    /// ```
    ///
    /// ```no_run
    /// use urlshortener::{UrlShortener, Provider};
    ///
    /// let us = UrlShortener::new().unwrap();
    /// let providers = [
    ///     Provider::GooGl { api_key: "MY_API_KEY".to_owned() },
    ///     Provider::IsGd,
    /// ];
    /// let long_url = "https://rust-lang.org";
    /// let _short_url = us.try_generate(long_url, Some(&providers));
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an `Error<ErrorKind::Other>` if there is an error generating a
    /// short URL from all providers.
    pub fn try_generate(
        &self,
        url: &str,
        use_providers: Option<&[Provider]>,
    ) -> Result<String, Error> {
        let providers = use_providers.unwrap_or(PROVIDERS);
        for provider in providers {
            // This would normally have the potential to panic, except that a
            // check to ensure there is an element at this index is performed.
            let res = self.generate(url, provider);

            if let Ok(s) = res {
                return Ok(s);
            } else {
                warn!(
                    "Failed to get short link from service: {}",
                    res.unwrap_err()
                );
            }
        }
        error!("Failed to get short link from any service");
        Err(Error::new(
            ErrorKind::Other,
            "Failed to get short link from any service",
        ))
    }

    /// Attempts to get a short URL using the specified provider.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use urlshortener::{Provider, UrlShortener};
    ///
    /// let us = UrlShortener::new().unwrap();
    /// let long_url = "http://rust-lang.org";
    /// let _short_url = us.generate(long_url, &Provider::IsGd);
    /// ```
    ///
    /// ```no_run
    /// use urlshortener::{Provider, UrlShortener};
    ///
    /// let us = UrlShortener::new().unwrap();
    /// let api_key = "MY_API_KEY".to_owned();
    /// let long_url = "http://rust-lang.org";
    /// let _short_url = us.generate(long_url, &Provider::GooGl { api_key: api_key });
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an `std::io::Error` if there is an error generating a
    /// short URL from the given provider due to either:
    ///
    /// a. a decode error (ErrorKind::Other);
    /// b. the service being unavailable (ErrorKind::ConnectionAborted)
    pub fn generate<S: Into<String>>(&self, url: S, provider: &Provider) -> Result<String, Error> {
        let response_opt = request(&url.into(), &self.client, provider);

        if let Some(mut response) = response_opt {
            if response.status().is_success() {
                let mut short_url = String::new();

                if try!(response.read_to_string(&mut short_url)) > 0 {
                    return parse(&short_url, provider)
                        .ok_or_else(|| Error::new(ErrorKind::Other, "Decode error"));
                }
            }
        }

        Err(Error::new(
            ErrorKind::ConnectionAborted,
            "Service is unavailable",
        ))
    }
}

#[cfg(test)]
mod tests {
    use std::io::ErrorKind;

    /// This test does not cover services which require authentication for obvious reasons.
    #[test]
    fn providers() {
        let us = ::UrlShortener::with_timeout(5).unwrap();
        let url = "http://stackoverflow.com";

        for provider in ::PROVIDERS {
            println!("Request shortening via provider: {}", provider.to_name());
            if let Some(err) = us.generate(url, provider).err() {
                println!("Error: {:?}", err);
                assert_eq!(err.kind(), ErrorKind::ConnectionAborted);
            }
        }
    }
}

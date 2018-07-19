use providers::{self, parse, request};
use reqwest::{self, Client};
use std::io::{Error, ErrorKind, Read};
use std::time::Duration;

//// Url shortener: the way to retrieve a short url.
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
        let client = reqwest::ClientBuilder::new()
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
    /// use urlshortener::client::UrlShortener;
    ///
    /// let us = UrlShortener::new().unwrap();
    /// let long_url = "https://rust-lang.org";
    /// let _short_url = us.try_generate(long_url, None);
    /// ```
    ///
    /// ```no_run
    /// use urlshortener::{client::UrlShortener, providers::Provider};
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
        use_providers: Option<&[providers::Provider]>,
    ) -> Result<String, Error> {
        let providers = use_providers.unwrap_or(providers::PROVIDERS);
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
    /// use urlshortener::{providers::Provider, client::UrlShortener};
    ///
    /// let us = UrlShortener::new().unwrap();
    /// let long_url = "http://rust-lang.org";
    /// let _short_url = us.generate(long_url, &Provider::IsGd);
    /// ```
    ///
    /// ```no_run
    /// use urlshortener::{providers::Provider, client::UrlShortener};
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
    pub fn generate<S: Into<String>>(
        &self,
        url: S,
        provider: &providers::Provider,
    ) -> Result<String, Error> {
        let req = request(&url.into(), provider);

        if let Some(mut response) = req.execute(&self.client) {
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
            "Could not create a request",
        ))
    }
}

#[cfg(test)]
mod tests {
    use client;
    use providers;
    use std::io::ErrorKind;

    /// This test does not cover services which require authentication for obvious reasons.
    #[test]
    fn providers() {
        let us = client::UrlShortener::with_timeout(5).unwrap();
        // let url = "http://stackoverflow.com";
        let url = "http://yandex.com";

        for provider in providers::PROVIDERS {
            println!("Request shortening via provider: {}", provider.to_name());
            if let Some(err) = us.generate(url, provider).err() {
                println!("Error: {:?}", err);
                assert_eq!(err.kind(), ErrorKind::ConnectionAborted);
            }
        }
    }
}

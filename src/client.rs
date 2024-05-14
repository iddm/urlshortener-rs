use crate::providers::{self, parse, request, ProviderError};
use reqwest::blocking::{Client, ClientBuilder};
use std::time::Duration;

/// Url shortener: the way to retrieve a short url.
#[derive(Debug, Clone)]
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
        let client = ClientBuilder::new()
            .timeout(Duration::from_secs(seconds))
            .build()?;

        Ok(UrlShortener { client })
    }

    /// Try to generate a short URL from each provider, iterating over each
    /// provider until a short URL is successfully generated.
    /// If you wish to override the list or providers or their priority,
    /// provide your own list of providers as second argument.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use urlshortener::client::UrlShortener;
    ///
    /// let us = UrlShortener::new().unwrap();
    /// let long_url = "https://rust-lang.org";
    /// let _short_url = us.try_generate(long_url, None);
    /// ```
    ///
    /// ```rust,no_run
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
    /// Returns an `Error<ProviderError>` if there is an error generating a
    /// short URL from all providers.
    ///
    /// # Notes
    ///
    /// This function has been deprecated since it does not bring any UX improvements.
    /// The body could be easily re-written as:
    ///
    /// ```rust,no_run
    /// use urlshortener::{client::UrlShortener, providers::Provider};
    ///
    /// let us = UrlShortener::new().unwrap();
    /// let providers = [
    ///     Provider::GooGl { api_key: "MY_API_KEY".to_owned() },
    ///     Provider::IsGd,
    /// ];
    /// let long_url = "https://rust-lang.org";
    /// let mut short_url = None;
    /// for provider in &providers {
    ///     if let Ok(short_url_res) = us.generate(long_url, provider) {
    ///         short_url = Some(short_url_res);
    ///         break;
    ///     }
    /// }
    ///
    /// ```
    #[deprecated(since = "1.0.0", note = "Please use `generate` directly instead.")]
    pub fn try_generate(
        &self,
        url: &str,
        use_providers: Option<&[providers::Provider]>,
    ) -> Result<String, ProviderError> {
        let providers = use_providers.unwrap_or(providers::PROVIDERS);
        for provider in providers {
            let res = self.generate(url, provider);

            if res.is_ok() {
                return res;
            }
        }

        Err(ProviderError::Connection)
    }

    /// Attempts to get a short URL using the specified provider.
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// use urlshortener::{providers::Provider, client::UrlShortener};
    ///
    /// let us = UrlShortener::new().unwrap();
    /// let long_url = "http://rust-lang.org";
    /// let _short_url = us.generate(long_url, &Provider::IsGd);
    /// ```
    ///
    /// ```rust,no_run
    /// use urlshortener::{providers::Provider, client::UrlShortener};
    ///
    /// let us = UrlShortener::new().unwrap();
    /// let api_key = "MY_API_KEY".to_owned();
    /// let long_url = "http://rust-lang.org";
    /// let _short_url = us.generate(long_url, &Provider::GooGl { api_key: api_key });
    /// ```
    pub fn generate<S: AsRef<str>>(
        &self,
        url: S,
        provider: &providers::Provider,
    ) -> Result<String, ProviderError> {
        let req = request(url.as_ref(), provider);

        if let Ok(response) = req.execute(&self.client) {
            response
                .text()
                .map_err(|_| ProviderError::Connection)
                .and_then(|t| parse(&t, provider))
        } else {
            Err(ProviderError::Connection)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::client;
    use crate::providers;

    /// This test does not cover services which require authentication for obvious reasons.
    #[test]
    fn providers() {
        let us = client::UrlShortener::with_timeout(5).unwrap();
        let url = "http://yandex.com";
        let mut valid = 0;

        for provider in providers::PROVIDERS {
            if let Err(e) = us.generate(url, provider) {
                println!("{:?} -> {:?}", provider, e);
            } else {
                valid += 1;
                println!("{:?} -> OK", provider);
            }
        }

        assert!(valid > 0, "There are no valid providers to use.");
    }
}

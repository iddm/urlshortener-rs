//! # urlshortener
//!
//! An easy library for retrieving short urls.
//!
//! ## Usage
//!
//! Creating a short URL via a specified provider is very simple:
//!
//! ```rust,no_run
//! use urlshortener::{providers::Provider, client::UrlShortener};
//!
//! let us = UrlShortener::new().unwrap();
//! let short_url = us.generate("https://my-long-url.com", &Provider::IsGd);
//! assert!(short_url.is_ok());
//! ```
//!
//! Or attempting all URL shorteners until one is successfully generated:
//!
//! ```rust,no_run
//! use urlshortener::client::UrlShortener;
//!
//! let us = UrlShortener::new().unwrap();
//! let short_url = us.try_generate("https://my-long-url.com", None);
//! assert!(short_url.is_ok());
//! ```
//! In order to use service with authentication use the appropriate provider directly:
//!
//! ```rust,no_run
//! use urlshortener::{ client::UrlShortener, providers::Provider };
//!
//! let us = UrlShortener::new().unwrap();
//! let key = "MY_API_KEY";
//! let short_url = us.generate("https://my-long-url.com", &Provider::GooGl { api_key:
//! key.to_owned() });
//! assert!(short_url.is_ok());
//! ```
#![deny(missing_docs)]
#![deny(warnings)]

/// A urlshortener http client for performing requests.
#[cfg(feature = "client")]
pub mod client;
pub mod providers;
/// A request builders for sending via http client.
pub mod request;

/// A prelude module with main useful stuff.
pub mod prelude {
    #[cfg(feature = "client")]
    pub use crate::client::*;
    pub use crate::providers::{Provider, PROVIDERS};
}

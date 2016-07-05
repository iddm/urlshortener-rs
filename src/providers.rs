//! Library service providers implementation.

extern crate hyper;

use hyper::client::{Client, RequestBuilder};

/// Used to specify which provider to use to generate a short URL.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Provider {
    /// https://bn.gy provider
    BnGy,
    /// https://is.gd provider
    IsGd,
    /// https://v.gd provider
    VGd,
}

impl Provider {
    /// Converts the Provider variant into its domain name equivilant
    pub fn to_name(&self) -> &str {
        match *self {
            Provider::BnGy => "bn.gy",
            Provider::IsGd => "is.gd",
            Provider::VGd => "v.gd",
        }
    }
}

/// Returns a vector of all `Provider` variants.
pub fn providers() -> Vec<Provider> {
    vec![Provider::BnGy, Provider::IsGd, Provider::VGd]
}

fn bngy_parse(res: &str) -> Option<String> {
    if res.is_empty() {
        return None
    }
    let string = res.to_owned();
    let iter = string.split("<ShortenedUrl>").skip(1).next();
    if iter.is_none() {
        return None
    }
    if let Some(string) = iter.unwrap().split("</ShortenedUrl>").next() {
        return Some(string.to_owned())
    }
    None
}

fn bngy_prepare<'a>(url: &str, client: &'a Client) -> RequestBuilder<'a> {
    client.get(&format!("https://bn.gy/API.asmx/CreateUrl?real_url={}", url))
}

fn isgd_parse(res: &str) -> Option<String> {
    Some(res.to_owned())
}

fn isgd_prepare<'a>(url: &str, client: &'a Client) -> RequestBuilder<'a> {
    client.get(&format!("https://is.gd/create.php?format=simple&url={}", url))
}

fn vgd_parse(res: &str) -> Option<String> {
    Some(res.to_owned())
}

fn vgd_prepare<'a>(url: &str, client: &'a Client) -> RequestBuilder<'a> {
    client.get(&format!("http://v.gd/create.php?format=simple&url={}", url))
}

/// Parses the response from a successful request to a provider into the
/// URL-shortened string.
pub fn parse(res: &str, provider: Provider) -> Option<String> {
    match provider {
        Provider::BnGy => bngy_parse(res),
        Provider::IsGd => isgd_parse(res),
        Provider::VGd => vgd_parse(res),
    }
}

/// Prepares the Hyper client for a connection to a provider, providing the long
/// URL to be shortened.
pub fn prepare<'a>(url: &str, client: &'a Client, provider: Provider) -> RequestBuilder<'a> {
    match provider {
        Provider::BnGy => bngy_prepare(url, client),
        Provider::IsGd => isgd_prepare(url, client),
        Provider::VGd => vgd_prepare(url, client),
    }
}

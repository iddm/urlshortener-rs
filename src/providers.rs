//! Library service providers implementation.

extern crate hyper;

use hyper::client::{Client, Response};

use std::str;

/// Used to specify which provider to use to generate a short URL.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Provider {
    /// https://bn.gy provider
    BnGy,
    /// https://is.gd provider
    IsGd,
    /// https://v.gd provider
    VGd,
    /// http://rlu.ru provider
    /// * Attention! If you send a lot of requests from one IP, it can be blocked. If you plan to add more then 100 URLs in one hour, please let the technical support know. Otherwise your IP can be blocked unexpectedly. Prior added URLs can be deleted.
    Rlu,
    /// http://bit.do provider
    BitDo,
}

impl Provider {
    /// Converts the Provider variant into its domain name equivilant
    pub fn to_name(&self) -> &str {
        match *self {
            Provider::BnGy => "bn.gy",
            Provider::IsGd => "is.gd",
            Provider::VGd => "v.gd",
            Provider::Rlu => "rlu.ru",
            Provider::BitDo => "bit.do",
        }
    }
}

/// Returns a vector of all `Provider` variants.
pub fn providers() -> Vec<Provider> {
    vec![
        Provider::BnGy,
        Provider::IsGd,
        Provider::VGd,
        Provider::Rlu,
        Provider::BitDo,
    ]
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

fn bngy_prepare(url: &str, client: &Client) -> Response {
    client.get(&format!("https://bn.gy/API.asmx/CreateUrl?real_url={}", url))
        .send()
        .unwrap()
}

fn isgd_parse(res: &str) -> Option<String> {
    Some(res.to_owned())
}

fn isgd_prepare(url: &str, client: &Client) -> Response {
    client.get(&format!("https://is.gd/create.php?format=simple&url={}", url))
        .send()
        .unwrap()
}

fn vgd_parse(res: &str) -> Option<String> {
    Some(res.to_owned())
}

fn vgd_prepare(url: &str, client: &Client) -> Response {
    client.get(&format!("http://v.gd/create.php?format=simple&url={}", url))
        .send()
        .unwrap()
}

fn rlu_parse(res: &str) -> Option<String> {
    Some(res.to_owned())
}

fn rlu_prepare(url: &str, client: &Client) -> Response {
    client.get(&format!("http://rlu.ru/index.sema?a=api&link={}", url))
        .send()
        .unwrap()
}

fn bitdo_parse(res: &str) -> Option<String> {
    Some(res.to_owned())
}

fn bitdo_prepare(url: &str, client: &Client) -> Response {
    let mut h_url = hyper::Url::parse("http://bit.do/mod_perl/url-shortener.pl").unwrap();
    h_url.query_pairs_mut()
        .append_pair("action", "shorten")
        .append_pair("url", url)
        .append_pair("url2", "site2")
        .append_pair("url_hash", "")
        .append_pair("url_stats_is_private", &0.to_string());
    let body = &*format!("action=shorten&url={}&url2=site2&url_hash=&url_stats_is_private=0", url)
        .into_bytes();
    client.post(h_url.as_str())
        .body(body)
        .send()
        .unwrap()

}

/// Parses the response from a successful request to a provider into the
/// URL-shortened string.
pub fn parse(res: &str, provider: Provider) -> Option<String> {
    match provider {
        Provider::BnGy => bngy_parse(res),
        Provider::IsGd => isgd_parse(res),
        Provider::VGd => vgd_parse(res),
        Provider::Rlu => rlu_parse(res),
        Provider::BitDo => bitdo_parse(res),
    }
}

/// Prepares the Hyper client for a connection to a provider, providing the long
/// URL to be shortened.
pub fn prepare(url: &str, client: &Client, provider: Provider) -> Response {
    match provider {
        Provider::BnGy => bngy_prepare(url, client),
        Provider::IsGd => isgd_prepare(url, client),
        Provider::VGd => vgd_prepare(url, client),
        Provider::Rlu => rlu_prepare(url, client),
        Provider::BitDo => bitdo_prepare(url, client),
    }
}

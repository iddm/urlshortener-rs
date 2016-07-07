//! Library service providers implementation.

extern crate hyper;

use hyper::client::{Client, Response};

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
    /// http://readability.com provider
    Rdd,
    /// http://psbe.co provider
    PsbeCo,
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
            Provider::Rdd => "readability.com",
            Provider::PsbeCo => "psbe.co",
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
        Provider::Rdd,

        // Latest elements should always be the worst services (ex: rate limit exists).
        Provider::Rlu,
        Provider::PsbeCo,
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

fn bngy_request(url: &str, client: &Client) -> Option<Response> {
    let resp = client.get(&format!("https://bn.gy/API.asmx/CreateUrl?real_url={}", url))
                     .send();
    if resp.is_ok() {
        return Some(resp.unwrap())
    }
    None
}

fn isgd_parse(res: &str) -> Option<String> {
    Some(res.to_owned())
}

fn isgd_request(url: &str, client: &Client) -> Option<Response> {
    let resp = client.get(&format!("https://is.gd/create.php?format=simple&url={}", url))
                     .send();
    if resp.is_ok() {
        return Some(resp.unwrap())
    }
    None
}

fn vgd_parse(res: &str) -> Option<String> {
    Some(res.to_owned())
}

fn vgd_request(url: &str, client: &Client) -> Option<Response> {
    let resp = client.get(&format!("http://v.gd/create.php?format=simple&url={}", url))
                     .send();
    if resp.is_ok() {
        return Some(resp.unwrap())
    }
    None
}

fn rlu_parse(res: &str) -> Option<String> {
    Some(res.to_owned())
}

fn rlu_request(url: &str, client: &Client) -> Option<Response> {
    let resp = client.get(&format!("http://rlu.ru/index.sema?a=api&link={}", url))
                     .send();
    if resp.is_ok() {
        return Some(resp.unwrap())
    }
    None
}

fn rdd_parse(res: &str) -> Option<String> {
    if res.is_empty() {
        return None
    }
    let string = res.to_owned();
    let value = string.split("\"rdd_url\"")
                      .nth(1).unwrap_or("")
                      .split(",").next().unwrap_or("")
                      .split("\"").nth(1);
    if let Some(string) = value {
        let mut short_url = string.to_owned();
        let _ = short_url.pop();
        return Some(short_url)
    }
    None
}

fn bitdo_parse(res: &str) -> Option<String> {
    Some(res.to_owned())
}

fn bitdo_request(url: &str, client: &Client) -> Option<Response> {
    let mut h_url = hyper::Url::parse("http://bit.do/mod_perl/url-shortener.pl").unwrap();
    h_url.query_pairs_mut()
        .append_pair("action", "shorten")
        .append_pair("url", url)
        .append_pair("url2", "site2")
        .append_pair("url_hash", "")
        .append_pair("url_stats_is_private", &0.to_string());
    let body = &*format!("action=shorten&url={}&url2=site2&url_hash=&url_stats_is_private=0", url)
        .into_bytes();
    let resp = client.post(h_url.as_str())
        .body(body)
        .send();

    if resp.is_ok() {
        return Some(resp.unwrap())
    }
    None
}

fn rdd_request(url: &str, client: &Client) -> Option<Response> {
    let body = &format!("url={}", url);
    let resp = client.post("https://readability.com/api/shortener/v1/urls")
                     .body(body)
                     .send();
    if resp.is_ok() {
        return Some(resp.unwrap())
    }
    None
}

fn psbeco_parse(res: &str) -> Option<String> {
    if res.is_empty() {
        return None
    }
    let string = res.to_owned();
    let iter = string.split("<ShortUrl>").skip(1).next();
    if iter.is_none() {
        return None
    }
    if let Some(string) = iter.unwrap().split("</ShortUrl>").next() {
        return Some(string.to_owned())
    }
    None
}

fn psbeco_request(url: &str, client: &Client) -> Option<Response> {
    let resp = client.get(&format!("http://psbe.co/API.asmx/CreateUrl?real_url={}", url))
                     .send();
    if resp.is_ok() {
        return Some(resp.unwrap())
    }
    None
}


/// Parses the response from a successful request to a provider into the
/// URL-shortened string.
pub fn parse(res: &str, provider: Provider) -> Option<String> {
    match provider {
        Provider::BitDo => bitdo_parse(res),
        Provider::BnGy => bngy_parse(res),
        Provider::IsGd => isgd_parse(res),
        Provider::VGd => vgd_parse(res),
        Provider::Rlu => rlu_parse(res),
        Provider::Rdd => rdd_parse(res),
        Provider::PsbeCo => psbeco_parse(res),
    }
}

/// Performs a request to the short link provider.
/// Response to be parsed or `None` on a error.
pub fn request(url: &str, client: &Client, provider: Provider) -> Option<Response> {
    match provider {
        Provider::BitDo => bitdo_request(url, client),
        Provider::BnGy => bngy_request(url, client),
        Provider::IsGd => isgd_request(url, client),
        Provider::VGd => vgd_request(url, client),
        Provider::Rlu => rlu_request(url, client),
        Provider::Rdd => rdd_request(url, client),
        Provider::PsbeCo => psbeco_request(url, client),
    }
}

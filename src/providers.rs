//! Library service providers implementation. 
extern crate hyper;

/// Provider abstraction trait
pub trait Provider {
    /// Name of the provider
    fn name(&self) -> &str;

    /// Prepare a request
    fn prepare_request<'a>(&self, url: &str, client: &'a hyper::Client) -> hyper::client::RequestBuilder<'a>;

    /// Try to parse the response of the request
    fn parse_response(&self, response: &str) -> Option<String>;
}

/// http://is.gd/ service provider implementation
pub struct IsGdProvider;
impl Provider for IsGdProvider {
    fn name(&self) -> &str {
        "is.gd"
    }

    fn prepare_request<'a>(&self, url: &str, client: &'a hyper::Client) -> hyper::client::RequestBuilder<'a> {
        client.get(&format!("http://is.gd/create.php?format=simple&url={}", url))
    }

    fn parse_response(&self, response: &str) -> Option<String> {
        Some(response.to_owned())
    }
}

/// http://v.gd/ service provider implementation
pub struct VGdProvider;
impl Provider for VGdProvider {
    fn name(&self) -> &str {
        "v.gd"
    }

    fn prepare_request<'a>(&self, url: &str, client: &'a hyper::Client) -> hyper::client::RequestBuilder<'a> {
        client.get(&format!("http://v.gd/create.php?format=simple&url={}", url))
    }

    fn parse_response(&self, response: &str) -> Option<String> {
        Some(response.to_owned())
    }
}

/// http://bn.gy/ service provider implementation
pub struct BnGyProvider;
impl Provider for BnGyProvider {
    fn name(&self) -> &str {
        "bn.gy"
    }

    fn prepare_request<'a>(&self, url: &str, client: &'a hyper::Client) -> hyper::client::RequestBuilder<'a> {
        client.get(&format!("http://bn.gy/API.asmx/CreateUrl?real_url={}", url))
    }

    // I did not want to use any xml-parser here so I decided just to do some hacks
    fn parse_response(&self, res: &str) -> Option<String> {
        if res.is_empty() {
            return None
        }
        let string = res.to_owned();
        let iter = string.split("<ShortenedUrl>").skip(1).next();
        if !iter.is_some() {
            return None
        }
        if let Some(string) = iter.unwrap().split("</ShortenedUrl>").next() {
            return Some(string.to_owned())
        }
        None
    }
}


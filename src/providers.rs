//! Library service providers implementation.

extern crate hyper;

use hyper::client::{Client, Response};
use hyper::header::ContentType;

/// Used to specify which provider to use to generate a short URL.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Provider {
    /// http://abv8.me provider
    /// Limits:
    /// * You may not shorten more than 20 unique URLs in a 3-minute span.
    /// * You may not shorten more than 60 unique URLs in a 15-minute span.
    Abv8,
    /// https://bam.bz provider
    BamBz,
    /// http://bmeo.org provider
    Bmeo,
    /// https://bn.gy provider
    BnGy,
    /// http://fifo.cc provider
    FifoCc,
    /// https://hec.su provider
    ///
    /// * Limited to 3000 API requests per day
    HecSu,
    /// https://is.gd provider
    IsGd,
    /// http://nowlinks.net provider
    NowLinks,
    /// http://phx.co.in provider
    /// After some time shows ads
    PhxCoIn,
    /// http://psbe.co provider
    PsbeCo,
    /// http://s.coop provider
    SCoop,
    /// http://readbility.com provider
    Rdd,
    /// http://rlu.ru provider
    ///
    /// * Attention! If you send a lot of requests from one IP, it can be
    /// blocked. If you plan to add more then 100 URLs in one hour, please let
    /// the technical support know. Otherwise your IP can be blocked
    /// unexpectedly. Prior added URLs can be deleted.
    Rlu,
    /// http://sirbz.com provider
    /// * Note: By default, you get 250 requests per 15 min for the SirBz URL Shortener API.
    SirBz,
    /// http://tinyurl.com provider
    /// * Note: this service does not provide any API.
    /// The implementation result depends on the service result web page.
    TinyUrl,
    /// http://tiny.ph provider
    TinyPh,
    /// http://tny.im provider
    TnyIm,
    /// https://v.gd provider
    VGd,
}

impl Provider {
    /// Converts the Provider variant into its domain name equivilant
    pub fn to_name(&self) -> &str {
        match *self {
            Provider::Abv8 => "abv8.me",
            Provider::BamBz => "bam.bz",
            Provider::Bmeo => "bmeo.org",
            Provider::BnGy => "bn.gy",
            Provider::FifoCc => "fifo.cc",
            Provider::HecSu => "hec.su",
            Provider::IsGd => "is.gd",
            Provider::NowLinks => "nowlinks.net",
            Provider::PhxCoIn => "phx.co.in",
            Provider::PsbeCo => "psbe.co",
            Provider::SCoop => "s.coop",
            Provider::SirBz => "sirbz.com",
            Provider::Rdd => "readability.com",
            Provider::Rlu => "rlu.ru",
            Provider::TinyUrl => "tinyurl.com",
            Provider::TinyPh => "tiny.ph",
            Provider::TnyIm => "tny.im",
            Provider::VGd => "v.gd",
        }
    }
}

/// Returns a vector of all `Provider` variants.
///
/// The providers which are discouraged from use due to limitations such as
/// rate limitations are at the end of the resultant vector.
pub fn providers() -> Vec<Provider> {
    vec![
        Provider::IsGd,
        Provider::BnGy,
        Provider::VGd,
        Provider::Rdd,
        Provider::BamBz,
        Provider::TinyPh,
        Provider::FifoCc,
        Provider::TnyIm,
        Provider::SCoop,
        Provider::Bmeo,

        // The following list are items that are discouraged from use.
        // Reason: rate limit (250 requests per 15 minutes)
        Provider::SirBz,
        // Reason: rate limit (100 requests per hour)
        Provider::Rlu,
        // Reason: rate limit (3000 requests per day)
        Provider::HecSu,
        // Reason: rate limit (20r/3min; 60r/15min for a UNIQUE urls only)
        Provider::Abv8,
        // Reason: does not provide an api
        Provider::TinyUrl,
        // Reason: unstable work
        Provider::PsbeCo,

        // The following list are items that show previews instead of direct links.
        Provider::NowLinks,

        // The following list are items that show ads and have a timeout before
        // you may go on the original link.
        Provider::PhxCoIn,
    ]
}

fn abv8_parse(res: &str) -> Option<String> {
    Some(res.to_owned())
}

fn abv8_request(url: &str, client: &Client) -> Option<Response> {
    client.get(&format!("http://abv8.me/?url={}", url))
        .send()
        .ok()
}

fn bambz_parse(res: &str) -> Option<String> {
    if res.is_empty() {
        return None
    }
    let string = res.to_owned();
    let value = string.split("\"url\"")
                      .nth(1).unwrap_or("")
                      .split(",").next().unwrap_or("")
                      .split("\"").nth(1);
    if let Some(string) = value {
        Some(string.to_owned().replace("\\", ""))
    } else {
        None
    }
}

fn bambz_request(url: &str, client: &Client) -> Option<Response> {
    client.post("https://bam.bz/api/short")
        .body(&format!("target={}", url))
        .header(ContentType::form_url_encoded())
        .send()
        .ok()
}

fn bmeo_parse(res: &str) -> Option<String> {
    if res.is_empty() {
        return None
    }
    let string = res.to_owned();
    let value = string.split("\"short\"")
                      .nth(1).unwrap_or("")
                      .split("\"").skip(1).next();
    if let Some(string) = value {
        Some(string.to_owned().replace("\\", ""))
    } else {
        None
    }
}

fn bmeo_request(url: &str, client: &Client) -> Option<Response> {
    client.get(&format!("http://bmeo.org/api.php?url={}", url))
        .send()
        .ok()
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
        Some(string.to_owned())
    } else {
        None
    }
}

fn bngy_request(url: &str, client: &Client) -> Option<Response> {
    client.get(&format!("https://bn.gy/API.asmx/CreateUrl?real_url={}", url))
        .send()
        .ok()
}

fn fifocc_parse(res: &str) -> Option<String> {
    if res.is_empty() {
        return None
    }
    let string = res.to_owned();
    let value = string.split("\"shortner\"")
                      .nth(1).unwrap_or("")
                      .split(",").next().unwrap_or("")
                      .split("\"").nth(1);
    if let Some(string) = value {
        let mut short_url = string.to_owned();
        short_url = format!("http://fifo.cc/{}", short_url);
        Some(short_url)
    } else {
        None
    }
}

fn fifocc_request(url: &str, client: &Client) -> Option<Response> {
    client.get(&format!("https://fifo.cc/api/v2?url={}", url))
        .send()
        .ok()
}


fn hecsu_parse(res: &str) -> Option<String> {
    if res.is_empty() {
        return None
    }
    let string = res.to_owned();
    let iter = string.split("<short>").skip(1).next();
    if iter.is_none() {
        return None
    }
    if let Some(string) = iter.unwrap().split("</short>").next() {
        Some(string.to_owned())
    } else {
        None
    }
}

fn hecsu_request(url: &str, client: &Client) -> Option<Response> {
    client.get(&format!("https://hec.su/api?url={}&method=xml", url))
        .send()
        .ok()
}

fn isgd_parse(res: &str) -> Option<String> {
    Some(res.to_owned())
}

fn isgd_request(url: &str, client: &Client) -> Option<Response> {
    client.get(&format!("https://is.gd/create.php?format=simple&url={}", url))
        .send()
        .ok()
}

fn nowlinks_parse(res: &str) -> Option<String> {
    Some(res.to_owned())
}

fn nowlinks_request(url: &str, client: &Client) -> Option<Response> {
    client.get(&format!("http://nowlinks.net/api?url={}", url))
        .send()
        .ok()
}

fn phxcoin_parse(res: &str) -> Option<String> {
    Some(res.to_owned())
}

fn phxcoin_request(url: &str, client: &Client) -> Option<Response> {
    client.get(&format!("http://phx.co.in/shrink.asp?url={}", url))
        .send()
        .ok()
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
        Some(string.to_owned())
    } else {
        None
    }
}

fn psbeco_request(url: &str, client: &Client) -> Option<Response> {
    client.get(&format!("http://psbe.co/API.asmx/CreateUrl?real_url={}", url))
        .send()
        .ok()
}

fn scoop_parse(res: &str) -> Option<String> {
    Some(res.to_owned())
}

fn scoop_request(url: &str, client: &Client) -> Option<Response> {
    client.get(&format!("http://s.coop/devapi.php?action=shorturl&url={}&format=RETURN", url))
        .send()
        .ok()
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
        Some(short_url)
    } else {
        None
    }
}

fn rdd_request(url: &str, client: &Client) -> Option<Response> {
    client.post("https://readability.com/api/shortener/v1/urls")
        .body(&format!("url={}", url))
        .send()
        .ok()
}

fn rlu_parse(res: &str) -> Option<String> {
    Some(res.to_owned())
}

fn rlu_request(url: &str, client: &Client) -> Option<Response> {
    client.get(&format!("http://rlu.ru/index.sema?a=api&link={}", url))
        .send()
        .ok()
}

fn sirbz_parse(res: &str) -> Option<String> {
    if res.is_empty() {
        return None
    }
    let string = res.to_owned();
    let value = string.split("\"short_link\"")
                      .nth(1).unwrap_or("")
                      .split("\"").skip(1).next();
    if let Some(string) = value {
        Some(string.to_owned())
    } else {
        None
    }
}

fn sirbz_request(url: &str, client: &Client) -> Option<Response> {
    client.post("http://sirbz.com/api/shorten_url")
        .body(&format!("url={}", url))
        .header(ContentType::form_url_encoded())
        .send()
        .ok()
}

fn tinyurl_parse(res: &str) -> Option<String> {
    if res.is_empty() {
        return None
    }
    let string = res.to_owned();
    let value = string.split("data-clipboard-text=\"")
                      .nth(1).unwrap_or("")
                      .split("\">").next();
    if let Some(string) = value {
        Some(string.to_owned())
    } else {
        None
    }
}

fn tinyurl_request(url: &str, client: &Client) -> Option<Response> {
    client.get(&format!("http://tinyurl.com/create.php?url={}", url))
        .send()
        .ok()
}

fn tinyph_parse(res: &str) -> Option<String> {
    if res.is_empty() {
        return None
    }
    let string = res.to_owned();
    let value = string.split("\"hash\"")
                      .nth(1).unwrap_or("")
                      .split("\"").skip(1).next();
    if let Some(string) = value {
        Some(format!("http://tiny.ph/{}", string))
    } else {
        None
    }
}

fn tinyph_request(url: &str, client: &Client) -> Option<Response> {
    client.post("http://tiny.ph/api/url/create")
        .body(&format!("url={}", url))
        .header(ContentType::form_url_encoded())
        .send()
        .ok()
}


fn tnyim_parse(res: &str) -> Option<String> {
    if res.is_empty() {
        return None
    }
    let string = res.to_owned();
    let iter = string.split("<shorturl>").skip(1).next();
    if iter.is_none() {
        return None
    }
    if let Some(string) = iter.unwrap().split("</shorturl>").next() {
        Some(string.to_owned())
    } else {
        None
    }
}

fn tnyim_request(url: &str, client: &Client) -> Option<Response> {
    client.get(&format!("http://tny.im/yourls-api.php?action=shorturl&url={}", url))
        .send()
        .ok()
}

fn vgd_parse(res: &str) -> Option<String> {
    Some(res.to_owned())
}

fn vgd_request(url: &str, client: &Client) -> Option<Response> {
    client.get(&format!("http://v.gd/create.php?format=simple&url={}", url))
        .send()
        .ok()
}


/// Parses the response from a successful request to a provider into the
/// URL-shortened string.
pub fn parse(res: &str, provider: Provider) -> Option<String> {
    match provider {
        Provider::Abv8 => abv8_parse(res),
        Provider::BamBz => bambz_parse(res),
        Provider::Bmeo => bmeo_parse(res),
        Provider::BnGy => bngy_parse(res),
        Provider::FifoCc => fifocc_parse(res),
        Provider::HecSu => hecsu_parse(res),
        Provider::IsGd => isgd_parse(res),
        Provider::NowLinks => nowlinks_parse(res),
        Provider::PhxCoIn => phxcoin_parse(res),
        Provider::PsbeCo => psbeco_parse(res),
        Provider::SCoop => scoop_parse(res),
        Provider::SirBz => sirbz_parse(res),
        Provider::Rdd => rdd_parse(res),
        Provider::Rlu => rlu_parse(res),
        Provider::TinyUrl => tinyurl_parse(res),
        Provider::TinyPh => tinyph_parse(res),
        Provider::TnyIm => tnyim_parse(res),
        Provider::VGd => vgd_parse(res),
    }
}

/// Performs a request to the short link provider.
/// Response to be parsed or `None` on a error.
pub fn request(url: &str, client: &Client, provider: Provider) -> Option<Response> {
    match provider {
        Provider::Abv8 => abv8_request(url, client),
        Provider::BamBz => bambz_request(url, client),
        Provider::Bmeo => bmeo_request(url, client),
        Provider::BnGy => bngy_request(url, client),
        Provider::FifoCc => fifocc_request(url, client),
        Provider::HecSu => hecsu_request(url, client),
        Provider::IsGd => isgd_request(url, client),
        Provider::NowLinks => nowlinks_request(url, client),
        Provider::PhxCoIn => phxcoin_request(url, client),
        Provider::PsbeCo => psbeco_request(url, client),
        Provider::SCoop => scoop_request(url, client),
        Provider::SirBz => sirbz_request(url, client),
        Provider::Rdd => rdd_request(url, client),
        Provider::Rlu => rlu_request(url, client),
        Provider::TinyUrl => tinyurl_request(url, client),
        Provider::TinyPh => tinyph_request(url, client),
        Provider::TnyIm => tnyim_request(url, client),
        Provider::VGd => vgd_request(url, client),
    }
}

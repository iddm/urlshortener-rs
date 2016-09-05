//! Library service providers implementation.

extern crate hyper;

use hyper::client::{Client, Response};
use hyper::header::ContentType;
use url::form_urlencoded::byte_serialize;

macro_rules! parse_xml_tag {
    ($fname: ident, $tag: expr) => {
        fn $fname(res: &str) -> Option<String> {
            res.split(&format!("<{}>", $tag))
                .nth(1)
                .unwrap_or("")
                .split(&format!("</{}>", $tag))
                .next()
                .map(String::from)
        }
    }
}

macro_rules! parse_json_tag {
    ($fname: ident, $tag: expr, $prefix: expr) => {
        fn $fname(res: &str) -> Option<String> {
            res.to_owned()
                .split(&format!("\"{}\"", $tag))
                .nth(1)
                .unwrap_or("")
                .split(",")
                .next()
                .unwrap_or("")
                .split("\"")
                .nth(1)
                .map(|v| format!("{}{}", $prefix, v.replace("\\", "")))
        }
    }
}

macro_rules! parse {
    ($name:ident) => {
        fn $name(res: &str) -> Option<String> {
            Some(res.to_owned())
        }
    };
}

macro_rules! request {
    ($name:ident, $method:ident, $req_url:expr) => {
        fn $name(url: &str, client: &Client) -> Option<Response> {
            let url = byte_serialize(url.as_bytes()).collect::<String>();
            client.$method(&format!($req_url, url))
                .send()
                .ok()
        }
    };

    (B, $name:ident, $method: ident, $req_url:expr, $body:expr) => {
        fn $name(url: &str, client: &Client) -> Option<Response> {
            client.$method($req_url)
                .body(&format!($body, url))
                .send()
                .ok()
        }
    };

    ($name:ident, $method:ident, $req_url:expr, $body:expr, $header:expr) => {
        fn $name(url: &str, client: &Client) -> Option<Response> {
            client.$method($req_url)
                .body(&format!($body, url))
                .header($header)
                .send()
                .ok()
        }
    };
}

/// Used to specify which provider to use to generate a short URL.
#[derive(Clone, Copy, Debug)]
pub enum Provider {
    /// http://abv8.me provider
    ///
    /// Notes:
    ///
    /// * You may not shorten more than 20 unique URLs within a 3-minute period.
    /// * You may not shorten more than 60 unique URLs within a 15-minute
    ///   period.
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
    /// Notes:
    ///
    /// * Limited to 3000 API requests per day
    HecSu,
    /// https://is.gd provider
    IsGd,
    /// http://nowlinks.net provider
    NowLinks,
    /// http://phx.co.in provider
    ///
    /// Notes:
    ///
    /// * After some time the service will display ads
    /// * Instead of redirecting, a preview page will be displayed
    /// * Currently unstable
    PhxCoIn,
    /// http://psbe.co provider
    PsbeCo,
    /// http://s.coop provider
    SCoop,
    /// http://readbility.com provider
    Rdd,
    /// http://rlu.ru provider
    ///
    /// Notes:
    ///
    /// * If you send a lot of requests from one IP, it can be
    ///   blocked. If you plan to add more then 100 URLs in one hour, please let
    ///   the technical support know. Otherwise your IP can be blocked
    ///   unexpectedly. Prior added URLs can be deleted.
    Rlu,
    /// http://sirbz.com provider
    ///
    /// Notes:
    ///
    /// * By default, you are limited to 250 requests per 15 minutes.
    SirBz,
    /// http://tinyurl.com provider
    ///
    /// Notes:
    ///
    /// * This service does not provide any API.
    /// * The implementation result depends on the service result web page.
    TinyUrl,
    /// http://tiny.ph provider
    TinyPh,
    /// http://tny.im provider
    TnyIm,
    /// http://url-shortener.io provider
    UrlShortenerIo,
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
            Provider::UrlShortenerIo => "url-shortener.io",
            Provider::VGd => "v.gd",
        }
    }
}

/// Returns a vector of all `Provider` variants. This list is in order of
/// provider quality.
///
/// The providers which are discouraged from use - due to problems such as rate
/// limitations - are at the end of the resultant vector.
///
/// Note that some providers may not provide a generated short URL because the
/// submitted URL may already be short enough and would not benefit from
/// shortening via their service.
pub fn providers() -> Vec<Provider> {
    vec![
        Provider::IsGd,
        Provider::BnGy,
        Provider::VGd,
        Provider::Rdd,
        Provider::BamBz,
        Provider::TinyPh,
        Provider::FifoCc,
        Provider::SCoop,
        Provider::Bmeo,
        Provider::UrlShortenerIo,

        // The following list are items that have long response sometimes:
        Provider::TnyIm,

        // The following list are items that are discouraged from use:

        // Reasons:
        //
        // * rate limit (250 requests per 15 minutes)
        // * does not accept short urls (ex: http://google.com)
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

        // The following list are items that show previews instead of direct
        // links.
        Provider::NowLinks,
    ]
}

parse!(abv8_parse);
request!(abv8_req, get, "http://abv8.me/?url={}");

parse_json_tag!(bambz_parse, "url", "");
request!(bambz_req,
         post,
         "https://bam.bz/api/short",
         "target={}",
         ContentType::form_url_encoded());

parse_json_tag!(bmeo_parse, "short", "");
request!(bmeo_req, get, "http://bmeo.org/api.php?url={}");

parse_xml_tag!(bngy_parse, "ShortenedUrl");
request!(bngy_req, get, "https://bn.gy/API.asmx/CreateUrl?real_url={}");

parse_json_tag!(fifocc_parse, "shortner", "http://fifo.cc/");
request!(fifocc_req, get, "https://fifo.cc/api/v2?url={}");

parse_xml_tag!(hecsu_parse, "short");
request!(hecsu_req, get, "https://hec.su/api?url={}&method=xml");

parse!(isgd_parse);
request!(isgd_req, get, "https://is.gd/create.php?format=simple&url={}");

parse!(nowlinks_parse);
request!(nowlinks_req, get, "http://nowlinks.net/api?url={}");

parse!(phxcoin_parse);
request!(phxcoin_req, get, "http://phx.co.in/shrink.asp?url={}");

parse_xml_tag!(psbeco_parse, "ShortUrl");
request!(psbeco_req, get, "http://psbe.co/API.asmx/CreateUrl?real_url={}");

parse!(scoop_parse);
request!(scoop_req,
         get,
         "http://s.coop/devapi.php?action=shorturl&url={}&format=RETURN");

parse_json_tag!(rdd_parse, "rdd_url", "");
request!(B,
         rdd_req,
         post,
         "https://readability.com/api/shortener/v1/urls",
         "url={}");

parse!(rlu_parse);
request!(rlu_req, get, "http://rlu.ru/index.sema?a=api&link={}");

parse_json_tag!(sirbz_parse, "short_link", "");
request!(sirbz_req,
         post,
         "http://sirbz.com/api/shorten_url",
         "url={}",
         ContentType::form_url_encoded());

fn tinyurl_parse(res: &str) -> Option<String> {
    res.split("data-clipboard-text=\"")
        .nth(1)
        .unwrap_or("")
        .split("\">")
        .next()
        .map(String::from)
}
request!(tinyurl_req, get, "http://tinyurl.com/create.php?url={}");

parse_json_tag!(tinyph_parse, "hash", "http://tiny.ph/");
request!(tinyph_req,
         post,
         "http://tiny.ph/api/url/create",
         "url={}",
         ContentType::form_url_encoded());

parse_xml_tag!(tnyim_parse, "shorturl");
request!(tnyim_req, get, "http://tny.im/yourls-api.php?action=shorturl&url={}");

parse!(urlshortenerio_parse);
request!(urlshortenerio_req,
         post,
         "http://url-shortener.io/shorten",
         "url_param={}",
         ContentType::form_url_encoded());

parse!(vgd_parse);
request!(vgd_req, get, "http://is.gd/create.php?format=simple&url={}");

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
        Provider::UrlShortenerIo => urlshortenerio_parse(res),
        Provider::VGd => vgd_parse(res),
    }
}

/// Performs a request to the short link provider.
/// Returns the parsed response on success or a `None` on error.
pub fn request(url: &str,
               client: &Client,
               provider: Provider)
               -> Option<Response> {
    match provider {
        Provider::Abv8 => abv8_req(url, client),
        Provider::BamBz => bambz_req(url, client),
        Provider::Bmeo => bmeo_req(url, client),
        Provider::BnGy => bngy_req(url, client),
        Provider::FifoCc => fifocc_req(url, client),
        Provider::HecSu => hecsu_req(url, client),
        Provider::IsGd => isgd_req(url, client),
        Provider::NowLinks => nowlinks_req(url, client),
        Provider::PhxCoIn => phxcoin_req(url, client),
        Provider::PsbeCo => psbeco_req(url, client),
        Provider::SCoop => scoop_req(url, client),
        Provider::SirBz => sirbz_req(url, client),
        Provider::Rdd => rdd_req(url, client),
        Provider::Rlu => rlu_req(url, client),
        Provider::TinyUrl => tinyurl_req(url, client),
        Provider::TinyPh => tinyph_req(url, client),
        Provider::TnyIm => tnyim_req(url, client),
        Provider::UrlShortenerIo => urlshortenerio_req(url, client),
        Provider::VGd => vgd_req(url, client),
    }
}

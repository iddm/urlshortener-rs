//! Library service providers implementation.

use crate::request as req;
use reqwest::header::HeaderMap;
use url::form_urlencoded;

/// A user agent for faking weird services.
const FAKE_USER_AGENT: &str =
    "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:58.0) Gecko/20100101 Firefox/58.0";

/// Describes the provider error.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ProviderError {
    /// Means there was a connection error. Usually when making a request.
    Connection,
    /// Means we were not able to deserialize the answer.
    Deserialize,
}

impl std::fmt::Display for ProviderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Connection => write!(
                f,
                "A connection problem occured when connecting to a provided."
            ),
            Self::Deserialize => write!(
                f,
                "Couldn't deserialize the shortened URL from the response."
            ),
        }
    }
}

impl std::error::Error for ProviderError {}

/// A slice of all `Provider` variants which do not require authentication.
/// This list is in order of provider quality.
///
/// The providers which are discouraged from use - due to problems such as rate
/// limitations - are at the end of the resultant slice.
///
/// Note that some providers may not provide a generated short URL because the
/// submitted URL may already be short enough and would not benefit from
/// shortening via their service.
pub const PROVIDERS: &[Provider] = &[
    Provider::IsGd,
    Provider::VGd,
    Provider::BamBz,
    Provider::TinyPh,
    Provider::FifoCc,
    Provider::SCoop,
    Provider::Bmeo,
    Provider::UrlShortenerIo,
    Provider::HmmRs,
    Provider::BitUrl,
    // The following list are items that have long response sometimes:
    Provider::TnyIm,
    // The following list are items that are discouraged from use:

    // Reasons:
    //
    // * rate limit (250 requests per 15 minutes)
    // * does not accept short urls (ex: <http://google.com>)
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
];

macro_rules! parse_xml_tag {
    ($fname:ident, $tag:expr) => {
        fn $fname(res: &str) -> Option<String> {
            res.split(&format!("<{}>", $tag))
                .nth(1)
                .unwrap_or("")
                .split(&format!("</{}>", $tag))
                .next()
                .map(String::from)
        }
    };
}

macro_rules! parse_json_tag {
    ($fname:ident, $tag:expr, $prefix:expr) => {
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
    };
}

macro_rules! parse_noop {
    ($name:ident) => {
        fn $name(res: &str) -> Option<String> {
            Some(res.to_owned())
        }
    };
}

macro_rules! request {
    ($name:ident, $method:expr, $req_url:expr) => {
        fn $name(url: &str) -> req::Request {
            let url = form_urlencoded::byte_serialize(url.as_bytes()).collect::<String>();
            req::Request {
                url: format!($req_url, url),
                body: None,
                content_type: None,
                user_agent: None,
                headers: None,
                method: $method,
            }
        }
    };

    (B, $name:ident, $method:expr, $req_url:expr, $body:expr) => {
        fn $name(url: &str) -> req::Request {
            req::Request {
                url: $req_url.to_owned(),
                body: Some(format!($body, url)),
                content_type: None,
                user_agent: None,
                headers: None,
                method: $method,
            }
        }
    };

    ($name:ident, $method:expr, $req_url:expr, $body:expr, $content_type:expr) => {
        fn $name(url: &str) -> req::Request {
            req::Request {
                url: $req_url.to_owned(),
                body: Some(format!($body, url)),
                content_type: Some($content_type),
                user_agent: None,
                headers: None,
                method: $method,
            }
        }
    };
}

/// Used to specify which provider to use to generate a short URL.
#[derive(Clone, Debug)]
pub enum Provider {
    /// <http://abv8.me> provider
    ///
    /// Notes:
    ///
    /// * You may not shorten more than 20 unique URLs within a 3-minute period.
    /// * You may not shorten more than 60 unique URLs within a 15-minute
    ///   period.
    Abv8,
    /// <https://bam.bz> provider
    BamBz,
    /// <https://bit.ly> provider
    BitLy {
        /// A token string which you may obtain on the provider web service page.
        token: String,
    },
    /// <https://biturl.top> provider
    BitUrl,
    /// <http://bmeo.org> provider
    Bmeo,
    /// <http://fifo.cc> provider
    FifoCc,
    /// <https://goo.gl> provider of Google
    GooGl {
        /// An api key string which you may obtain on the provider web service page.
        api_key: String,
    },
    /// <https://kutt.it> provider, can be self hosted
    Kutt {
        /// An api key string which you may obtain on the provider web service page.
        api_key: String,
        /// The api host, defaults to '<https://kutt.it>'
        host: Option<String>,
    },
    /// <https://hec.su> provider
    ///
    /// Notes:
    ///
    /// * Limited to 3000 API requests per day
    HecSu,
    /// <http://hmm.rs> provider
    HmmRs,
    /// <https://is.gd> provider
    IsGd,
    /// <http://nowlinks.net> provider
    NowLinks,
    /// <http://phx.co.in> provider
    ///
    /// Notes:
    ///
    /// * After some time the service will display ads
    /// * Instead of redirecting, a preview page will be displayed
    /// * Currently unstable
    PhxCoIn,
    /// <http://psbe.co> provider
    PsbeCo,
    /// <http://s.coop> provider
    SCoop,
    /// <http://rlu.ru> provider
    ///
    /// Notes:
    ///
    /// * If you send a lot of requests from one IP, it can be
    ///   blocked. If you plan to add more then 100 URLs in one hour, please let
    ///   the technical support know. Otherwise your IP can be blocked
    ///   unexpectedly. Prior added URLs can be deleted.
    Rlu,
    /// <http://sirbz.com> provider
    ///
    /// Notes:
    ///
    /// * By default, you are limited to 250 requests per 15 minutes.
    SirBz,
    /// <http://tinyurl.com> provider
    ///
    /// Notes:
    ///
    /// * This service does not provide any API.
    /// * The implementation result depends on the service result web page.
    TinyUrl,
    /// <http://tiny.ph> provider
    TinyPh,
    /// <http://tny.im> provider
    TnyIm,
    /// <http://url-shortener.io> provider
    UrlShortenerIo,
    /// <https://v.gd> provider
    VGd,
}

impl Provider {
    /// Converts the Provider variant into its domain name equivilant
    pub fn to_name(&self) -> &str {
        match *self {
            Provider::Abv8 => "abv8.me",
            Provider::BamBz => "bam.bz",
            Provider::BitLy { .. } => "bitly.com",
            Provider::BitUrl => "biturl.top",
            Provider::Bmeo => "bmeo.org",
            Provider::FifoCc => "fifo.cc",
            Provider::GooGl { .. } => "goo.gl",
            Provider::HmmRs => "hmm.rs",
            Provider::HecSu => "hec.su",
            Provider::IsGd => "is.gd",
            Provider::Kutt { ref host, .. } => host
                .as_ref()
                .map(|h| h.rsplit("//").next().unwrap())
                .unwrap_or("kutt.it"),
            Provider::NowLinks => "nowlinks.net",
            Provider::PhxCoIn => "phx.co.in",
            Provider::PsbeCo => "psbe.co",
            Provider::SCoop => "s.coop",
            Provider::SirBz => "sirbz.com",
            Provider::Rlu => "rlu.ru",
            Provider::TinyUrl => "tinyurl.com",
            Provider::TinyPh => "tiny.ph",
            Provider::TnyIm => "tny.im",
            Provider::UrlShortenerIo => "url-shortener.io",
            Provider::VGd => "v.gd",
        }
    }
}

parse_noop!(abv8_parse);
request!(abv8_req, req::Method::Get, "http://abv8.me/?url={}");

parse_json_tag!(bambz_parse, "url", "");
request!(
    bambz_req,
    req::Method::Post,
    "https://bam.bz/api/short",
    "target={}",
    req::ContentType::FormUrlEncoded
);

parse_noop!(bitly_parse);
fn bitly_req(url: &str, key: &str) -> req::Request {
    let encoded_url = form_urlencoded::byte_serialize(url.as_bytes()).collect::<String>();
    let address = format!(
        "https://api-ssl.bitly.com/v3/shorten?access_token={}&longUrl={}&format=txt",
        key, encoded_url
    );

    req::Request {
        url: address,
        body: None,
        content_type: None,
        user_agent: None,
        headers: None,
        method: req::Method::Get,
    }
}

parse_json_tag!(bmeo_parse, "short", "");
request!(bmeo_req, req::Method::Get, "http://bmeo.org/api.php?url={}");

parse_json_tag!(fifocc_parse, "shortner", "http://fifo.cc/");
request!(
    fifocc_req,
    req::Method::Get,
    "https://fifo.cc/api/v2?url={}"
);

parse_json_tag!(googl_parse, "id", "");
fn googl_req(url: &str, key: &str) -> req::Request {
    req::Request {
        url: format!("https://www.googleapis.com/urlshortener/v1/url?key={}", key),
        body: Some(format!(r#"{{"longUrl": "{}"}}"#, url)),
        content_type: Some(req::ContentType::Json),
        user_agent: None,
        headers: None,
        method: req::Method::Post,
    }
}

parse_json_tag!(hmmrs_parse, "shortUrl", "");
fn hmmrs_req(url: &str) -> req::Request {
    req::Request {
        url: "http:/hmm.rs/x/shorten".to_owned(),
        body: Some(format!(r#"{{"url": "{}"}}"#, url)),
        content_type: Some(req::ContentType::Json),
        user_agent: Some(req::UserAgent(FAKE_USER_AGENT.to_owned())),
        headers: None,
        method: req::Method::Post,
    }
}

parse_xml_tag!(hecsu_parse, "short");
request!(
    hecsu_req,
    req::Method::Get,
    "https://hec.su/api?url={}&method=xml"
);

parse_noop!(isgd_parse);
request!(
    isgd_req,
    req::Method::Get,
    "https://is.gd/create.php?format=simple&url={}"
);

parse_json_tag!(kutt_parse, "shortUrl", "");
fn kutt_req(url: &str, api_key: &str, host: Option<&str>) -> req::Request {
    let mut headers = HeaderMap::new();
    headers.insert("X-API-Key", api_key.parse().unwrap());

    req::Request {
        url: format!("{}/api/url/submit", host.unwrap_or("https://kutt.it")),
        body: Some(format!(r#"{{"target": "{}"}}"#, url)),
        content_type: Some(req::ContentType::Json),
        user_agent: None,
        headers: Some(headers),
        method: req::Method::Post,
    }
}

parse_noop!(nowlinks_parse);
request!(
    nowlinks_req,
    req::Method::Get,
    "http://nowlinks.net/api?url={}"
);

parse_noop!(phxcoin_parse);
request!(
    phxcoin_req,
    req::Method::Get,
    "http://phx.co.in/shrink.asp?url={}"
);

parse_xml_tag!(psbeco_parse, "ShortUrl");
request!(
    psbeco_req,
    req::Method::Get,
    "http://psbe.co/API.asmx/CreateUrl?real_url={}"
);

parse_noop!(scoop_parse);
request!(
    scoop_req,
    req::Method::Get,
    "http://s.coop/devapi.php?action=shorturl&url={}&format=RETURN"
);

parse_noop!(rlu_parse);
request!(
    rlu_req,
    req::Method::Get,
    "http://rlu.ru/index.sema?a=api&link={}"
);

parse_json_tag!(sirbz_parse, "short_link", "");
request!(
    sirbz_req,
    req::Method::Post,
    "http://sirbz.com/api/shorten_url",
    "url={}",
    req::ContentType::FormUrlEncoded
);

fn tinyurl_parse(res: &str) -> Option<String> {
    res.split("data-clipboard-text=\"")
        .nth(1)
        .unwrap_or("")
        .split("\">")
        .next()
        .map(String::from)
}
request!(
    tinyurl_req,
    req::Method::Get,
    "http://tinyurl.com/create.php?url={}"
);

parse_json_tag!(tinyph_parse, "hash", "http://tiny.ph/");
request!(
    tinyph_req,
    req::Method::Post,
    "http://tiny.ph/api/url/create",
    "url={}",
    req::ContentType::FormUrlEncoded
);

parse_xml_tag!(tnyim_parse, "shorturl");
request!(
    tnyim_req,
    req::Method::Get,
    "http://tny.im/yourls-api.php?action=shorturl&url={}"
);

parse_noop!(urlshortenerio_parse);
request!(
    urlshortenerio_req,
    req::Method::Post,
    "http://url-shortener.io/shorten",
    "url_param={}",
    req::ContentType::FormUrlEncoded
);

parse_noop!(vgd_parse);
request!(
    vgd_req,
    req::Method::Get,
    "http://v.gd/create.php?format=simple&url={}"
);

parse_json_tag!(biturl_parse, "short", "");
request!(
    biturl_req,
    req::Method::Post,
    "https://api.biturl.top/short",
    "url={}",
    req::ContentType::FormUrlEncoded
);

/// Parses the response from a successful request to a provider into the
/// URL-shortened string.
pub fn parse(res: &str, provider: &Provider) -> Result<String, ProviderError> {
    match *provider {
        Provider::Abv8 => abv8_parse(res),
        Provider::BamBz => bambz_parse(res),
        Provider::BitLy { .. } => bitly_parse(res),
        Provider::BitUrl => biturl_parse(res),
        Provider::Bmeo => bmeo_parse(res),
        Provider::FifoCc => fifocc_parse(res),
        Provider::GooGl { .. } => googl_parse(res),
        Provider::HmmRs => hmmrs_parse(res),
        Provider::HecSu => hecsu_parse(res),
        Provider::IsGd => isgd_parse(res),
        Provider::Kutt { .. } => kutt_parse(res),
        Provider::NowLinks => nowlinks_parse(res),
        Provider::PhxCoIn => phxcoin_parse(res),
        Provider::PsbeCo => psbeco_parse(res),
        Provider::SCoop => scoop_parse(res),
        Provider::SirBz => sirbz_parse(res),
        Provider::Rlu => rlu_parse(res),
        Provider::TinyUrl => tinyurl_parse(res),
        Provider::TinyPh => tinyph_parse(res),
        Provider::TnyIm => tnyim_parse(res),
        Provider::UrlShortenerIo => urlshortenerio_parse(res),
        Provider::VGd => vgd_parse(res),
    }
    .ok_or(ProviderError::Deserialize)
}

/// Performs a request to the short link provider.
/// Returns the request object which can be used for performing requests.
///
/// # Example
///
/// ```rust,no_run
/// extern crate urlshortener;
///
/// use urlshortener::providers::{Provider, self};
///
/// fn main() {
///     let long_url = "https://google.com";
///     let key = "MY_API_KEY";
///     let req = providers::request(long_url, &Provider::GooGl { api_key: key.to_owned() });
///     println!("A request object for shortening URL via GooGl: {:?}", req);
/// }
/// ```
pub fn request(url: &str, provider: &Provider) -> req::Request {
    match *provider {
        Provider::Abv8 => abv8_req(url),
        Provider::BamBz => bambz_req(url),
        Provider::BitLy { ref token } => bitly_req(url, token),
        Provider::BitUrl => biturl_req(url),
        Provider::Bmeo => bmeo_req(url),
        Provider::FifoCc => fifocc_req(url),
        Provider::GooGl { ref api_key } => googl_req(url, api_key),
        Provider::HmmRs => hmmrs_req(url),
        Provider::HecSu => hecsu_req(url),
        Provider::IsGd => isgd_req(url),
        Provider::Kutt {
            ref api_key,
            ref host,
        } => kutt_req(url, api_key, host.as_ref().map(|h| &**h)),
        Provider::NowLinks => nowlinks_req(url),
        Provider::PhxCoIn => phxcoin_req(url),
        Provider::PsbeCo => psbeco_req(url),
        Provider::SCoop => scoop_req(url),
        Provider::SirBz => sirbz_req(url),
        Provider::Rlu => rlu_req(url),
        Provider::TinyUrl => tinyurl_req(url),
        Provider::TinyPh => tinyph_req(url),
        Provider::TnyIm => tnyim_req(url),
        Provider::UrlShortenerIo => urlshortenerio_req(url),
        Provider::VGd => vgd_req(url),
    }
}

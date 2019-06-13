//! Library service providers implementation.

use request;
use url::form_urlencoded;
use url::percent_encoding::{utf8_percent_encode, QUERY_ENCODE_SET};

/// A user agent for faking weird services.
const FAKE_USER_AGENT: &str =
    "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:58.0) Gecko/20100101 Firefox/58.0";

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
    Provider::BnGy,
    Provider::VGd,
    Provider::BamBz,
    Provider::TinyPh,
    Provider::FifoCc,
    Provider::SCoop,
    Provider::Bmeo,
    Provider::UrlShortenerIo,
    Provider::HmmRs,
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

macro_rules! parse {
    ($name:ident) => {
        fn $name(res: &str) -> Option<String> {
            Some(res.to_owned())
        }
    };
}

macro_rules! request {
    ($name:ident, $method:expr, $req_url:expr) => {
        fn $name(url: &str) -> request::Request {
            let url = form_urlencoded::byte_serialize(url.as_bytes()).collect::<String>();
            request::Request {
                url: format!($req_url, url),
                body: None,
                content_type: None,
                user_agent: None,
                method: $method,
            }
        }
    };

    (B, $name:ident, $method:expr, $req_url:expr, $body:expr) => {
        fn $name(url: &str) -> request::Request {
            request::Request {
                url: $req_url.to_owned(),
                body: Some(format!($body, url)),
                content_type: None,
                user_agent: None,
                method: $method,
            }
        }
    };

    ($name:ident, $method:expr, $req_url:expr, $body:expr, $content_type:expr) => {
        fn $name(url: &str) -> request::Request {
            request::Request {
                url: $req_url.to_owned(),
                body: Some(format!($body, url)),
                content_type: Some($content_type),
                user_agent: None,
                method: $method,
            }
        }
    };
}

/// Used to specify which provider to use to generate a short URL.
#[derive(Clone, Debug)]
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
    /// https://bit.ly provider
    BitLy { token: String },
    /// http://bmeo.org provider
    Bmeo,
    /// https://bn.gy provider
    BnGy,
    /// http://fifo.cc provider
    FifoCc,
    /// https://goo.gl provider of Google
    GooGl { api_key: String },
    /// https://hec.su provider
    ///
    /// Notes:
    ///
    /// * Limited to 3000 API requests per day
    HecSu,
    /// http://hmm.rs provider
    HmmRs,
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
            Provider::BitLy { .. } => "bitly.com",
            Provider::Bmeo => "bmeo.org",
            Provider::BnGy => "bn.gy",
            Provider::FifoCc => "fifo.cc",
            Provider::GooGl { .. } => "goo.gl",
            Provider::HmmRs => "hmm.rs",
            Provider::HecSu => "hec.su",
            Provider::IsGd => "is.gd",
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

parse!(abv8_parse);
request!(abv8_req, request::Method::Get, "http://abv8.me/?url={}");

parse_json_tag!(bambz_parse, "url", "");
request!(
    bambz_req,
    request::Method::Post,
    "https://bam.bz/api/short",
    "target={}",
    request::ContentType::FormUrlEncoded
);

parse!(bitly_parse);
fn bitly_req(url: &str, key: &str) -> request::Request {
    let encoded_url = utf8_percent_encode(url, QUERY_ENCODE_SET).collect::<String>();
    let address = format!(
        "https://api-ssl.bitly.com/v3/shorten?access_token={}&longUrl={}&format=txt",
        key, encoded_url
    );

    request::Request {
        url: address,
        body: None,
        content_type: None,
        user_agent: None,
        method: request::Method::Get,
    }
}

parse_json_tag!(bmeo_parse, "short", "");
request!(
    bmeo_req,
    request::Method::Get,
    "http://bmeo.org/api.php?url={}"
);

parse_xml_tag!(bngy_parse, "ShortenedUrl");
request!(
    bngy_req,
    request::Method::Get,
    "https://bn.gy/API.asmx/CreateUrl?real_url={}"
);

parse_json_tag!(fifocc_parse, "shortner", "http://fifo.cc/");
request!(
    fifocc_req,
    request::Method::Get,
    "https://fifo.cc/api/v2?url={}"
);

parse_json_tag!(googl_parse, "id", "");
fn googl_req(url: &str, key: &str) -> request::Request {
    request::Request {
        url: format!("https://www.googleapis.com/urlshortener/v1/url?key={}", key),
        body: Some(format!(r#"{{"longUrl": "{}"}}"#, url)),
        content_type: Some(request::ContentType::Json),
        user_agent: None,
        method: request::Method::Post,
    }
}

parse_json_tag!(hmmrs_parse, "shortUrl", "");
fn hmmrs_req(url: &str) -> request::Request {
    request::Request {
        url: "http:/hmm.rs/x/shorten".to_owned(),
        body: Some(format!(r#"{{"url": "{}"}}"#, url)),
        content_type: Some(request::ContentType::Json),
        user_agent: Some(request::UserAgent(FAKE_USER_AGENT.to_owned())),
        method: request::Method::Post,
    }
}

parse_xml_tag!(hecsu_parse, "short");
request!(
    hecsu_req,
    request::Method::Get,
    "https://hec.su/api?url={}&method=xml"
);

parse!(isgd_parse);
request!(
    isgd_req,
    request::Method::Get,
    "https://is.gd/create.php?format=simple&url={}"
);

parse!(nowlinks_parse);
request!(
    nowlinks_req,
    request::Method::Get,
    "http://nowlinks.net/api?url={}"
);

parse!(phxcoin_parse);
request!(
    phxcoin_req,
    request::Method::Get,
    "http://phx.co.in/shrink.asp?url={}"
);

parse_xml_tag!(psbeco_parse, "ShortUrl");
request!(
    psbeco_req,
    request::Method::Get,
    "http://psbe.co/API.asmx/CreateUrl?real_url={}"
);

parse!(scoop_parse);
request!(
    scoop_req,
    request::Method::Get,
    "http://s.coop/devapi.php?action=shorturl&url={}&format=RETURN"
);

parse!(rlu_parse);
request!(
    rlu_req,
    request::Method::Get,
    "http://rlu.ru/index.sema?a=api&link={}"
);

parse_json_tag!(sirbz_parse, "short_link", "");
request!(
    sirbz_req,
    request::Method::Post,
    "http://sirbz.com/api/shorten_url",
    "url={}",
    request::ContentType::FormUrlEncoded
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
    request::Method::Get,
    "http://tinyurl.com/create.php?url={}"
);

parse_json_tag!(tinyph_parse, "hash", "http://tiny.ph/");
request!(
    tinyph_req,
    request::Method::Post,
    "http://tiny.ph/api/url/create",
    "url={}",
    request::ContentType::FormUrlEncoded
);

parse_xml_tag!(tnyim_parse, "shorturl");
request!(
    tnyim_req,
    request::Method::Get,
    "http://tny.im/yourls-api.php?action=shorturl&url={}"
);

parse!(urlshortenerio_parse);
request!(
    urlshortenerio_req,
    request::Method::Post,
    "http://url-shortener.io/shorten",
    "url_param={}",
    request::ContentType::FormUrlEncoded
);

parse!(vgd_parse);
request!(
    vgd_req,
    request::Method::Get,
    "http://is.gd/create.php?format=simple&url={}"
);

/// Parses the response from a successful request to a provider into the
/// URL-shortened string.
pub fn parse(res: &str, provider: &Provider) -> Option<String> {
    match *provider {
        Provider::Abv8 => abv8_parse(res),
        Provider::BamBz => bambz_parse(res),
        Provider::BitLy { .. } => bitly_parse(res),
        Provider::Bmeo => bmeo_parse(res),
        Provider::BnGy => bngy_parse(res),
        Provider::FifoCc => fifocc_parse(res),
        Provider::GooGl { .. } => googl_parse(res),
        Provider::HmmRs => hmmrs_parse(res),
        Provider::HecSu => hecsu_parse(res),
        Provider::IsGd => isgd_parse(res),
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
}

/// Performs a request to the short link provider.
/// Returns the request object which can be used for performing requests.
///
/// # Example
///
/// ```no_run
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
pub fn request(url: &str, provider: &Provider) -> request::Request {
    match *provider {
        Provider::Abv8 => abv8_req(url),
        Provider::BamBz => bambz_req(url),
        Provider::BitLy { token: ref key } => bitly_req(url, &key),
        Provider::Bmeo => bmeo_req(url),
        Provider::BnGy => bngy_req(url),
        Provider::FifoCc => fifocc_req(url),
        Provider::GooGl { api_key: ref key } => googl_req(url, &key),
        Provider::HmmRs => hmmrs_req(url),
        Provider::HecSu => hecsu_req(url),
        Provider::IsGd => isgd_req(url),
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

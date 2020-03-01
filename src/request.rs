#[cfg(feature = "client")]
use reqwest::{
    blocking::{Client, Response},
    header::{self, HeaderMap},
};

const CONTENT_JSON: &str = "application/json";
const CONTENT_FORM_URL_ENCODED: &str = "application/x-www-form-urlencoded";

/// An HTTP method abstraction
#[derive(Debug, Copy, Clone)]
pub enum Method {
    /// `Get` HTTP method should be used.
    Get,
    /// `POST` HTTP method should be used.
    Post,
}

/// An HTTP content type abstraction
#[derive(Debug, Copy, Clone)]
pub enum ContentType {
    /// The url encoded form data header should be used.
    FormUrlEncoded,
    /// The json header should be used.
    Json,
}

/// An HTTP user agent abstraction
#[derive(Debug, Clone)]
pub struct UserAgent(pub String);

/// An abstraction for basic http request.
#[derive(Debug, Clone)]
pub struct Request {
    /// The URL the request must be sent to.
    pub url: String,
    /// The request body.
    pub body: Option<String>,
    /// The content type.
    pub content_type: Option<ContentType>,
    /// The user agent.
    pub user_agent: Option<UserAgent>,
    /// Request headers.
    pub headers: Option<HeaderMap>,
    /// The HTTP method.
    pub method: Method,
}

#[cfg(feature = "client")]
impl Request {
    /// Sends the request and returns the response.
    pub fn execute(&self, client: &Client) -> Result<Response, reqwest::Error> {
        let mut builder = match self.method {
            Method::Get => client.get(&self.url),
            Method::Post => client.post(&self.url),
        };

        if let Some(agent) = self.user_agent.clone() {
            builder = builder.header(header::USER_AGENT, agent.0);
        }

        if let Some(headers) = self.headers.clone() {
            builder = builder.headers(headers);
        }

        if let Some(content_type) = self.content_type {
            builder = match content_type {
                ContentType::Json => builder.header(header::CONTENT_TYPE, CONTENT_JSON),
                ContentType::FormUrlEncoded => {
                    builder.header(header::CONTENT_TYPE, CONTENT_FORM_URL_ENCODED)
                }
            };
        }

        if let Some(body) = self.body.clone() {
            builder = builder.body(body);
        }

        builder.send()
    }
}

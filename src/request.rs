#[cfg(feature = "client")]
use reqwest::{header, Client, Response};

const CONTENT_JSON: &'static str = "application/json";
const CONTENT_FORM_URL_ENCODED: &'static str = "application/x-www-form-urlencoded";

/// An HTTP method abstraction
#[derive(Debug, Copy, Clone)]
pub enum Method {
    Get,
    Post,
}

/// An HTTP content type abstraction
#[derive(Debug, Copy, Clone)]
pub enum ContentType {
    FormUrlEncoded,
    Json,
}

/// An HTTP user agent abstraction
#[derive(Debug, Clone)]
pub struct UserAgent(pub String);

/// An abstraction for basic http request.
#[derive(Debug, Clone)]
pub struct Request {
    pub url: String,
    pub body: Option<String>,
    pub content_type: Option<ContentType>,
    pub user_agent: Option<UserAgent>,
    pub method: Method,
}

#[cfg(feature = "client")]
impl Request {
    pub fn execute(&self, client: &Client) -> Option<Response> {
        let mut builder = match self.method {
            Method::Get => client.get(&self.url),
            Method::Post => client.post(&self.url),
        };

        if let Some(agent) = self.user_agent.clone() {
            builder = builder.header(header::USER_AGENT, agent.0);
        }

        if let Some(content_type) = self.content_type.clone() {
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

        builder.send().ok()
    }
}

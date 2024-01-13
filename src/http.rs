const CONTENT_TYPE: &str = "Content-Type";

#[derive(Debug, Clone)]
pub(crate) struct Client {
    api_key: String,
    #[cfg(feature = "reqwest")]
    client: reqwest::Client,
}

impl Client {
    pub(crate) fn new(api_key: &str) -> Self {
        Self {
            api_key: api_key.to_owned(),
            client: reqwest::Client::new(),
        }
    }
}

#[cfg(feature = "reqwest")]
impl Client {
    pub(crate) async fn perform(&self, r: Request) -> Result<reqwest::Response, reqwest::Error> {
        match r.method {
            Method::Post => {
                let mut request = self
                    .client
                    .post(&r.url)
                    .bearer_auth(&self.api_key)
                    .header(CONTENT_TYPE, "application/json");

                if let Some(body) = r.body {
                    request = request.body(body);
                }

                request.send().await
            }
            Method::Get => {
                self.client
                    .get(&r.url)
                    .bearer_auth(&self.api_key)
                    .header(CONTENT_TYPE, "application/json")
                    .send()
                    .await
            }
            Method::Delete => {
                self.client
                    .delete(&r.url)
                    .bearer_auth(&self.api_key)
                    .header(CONTENT_TYPE, "application/json")
                    .send()
                    .await
            }
        }
    }
}

pub(crate) struct Request {
    method: Method,
    url: String,
    body: Option<String>,
}

impl Request {
    pub(crate) fn new(method: Method, url: &str, body: Option<String>) -> Self {
        Self {
            method,
            url: url.to_owned(),
            body,
        }
    }
}

pub(crate) enum Method {
    Post,
    Get,
    Delete,
}

pub mod api_keys;
pub mod batch;
pub mod domains;
pub mod emails;
mod http;
mod utils;

const DEFAULT_BASE_URL: &str = "https://api.resend.com";

async fn parse_response(res: reqwest::Response) -> Result<String, Error> {
    if res.status() != 200 && res.status() != 201 {
        return Err(Error::Resend(
            serde_json::from_str(&res.text().await.unwrap()).unwrap(),
        ));
    }

    res.text().await.map_err(Error::Client)
}

#[derive(Debug, Clone)]
pub struct Client {
    base_url: String,
    client: http::Client,
}

impl Client {
    pub fn new(api_key: &str) -> Self {
        Self {
            base_url: DEFAULT_BASE_URL.to_owned(),
            client: http::Client::new(api_key),
        }
    }
}

impl Client {
    async fn perform(&self, r: http::Request) -> Result<reqwest::Response, reqwest::Error> {
        self.client.perform(r).await
    }
}

#[derive(Debug)]
pub enum Error {
    Resend(ResendErrorResponse),
    JSON(serde_json::Error),
    #[cfg(feature = "reqwest")]
    Client(reqwest::Error),
    Internal,
}

#[derive(Debug, Clone, Default, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ResendErrorResponse {
    name: String,
    message: String,
}

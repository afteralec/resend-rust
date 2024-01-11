use std::fmt;

pub mod api_keys;
pub mod batch;
pub mod domains;
pub mod emails;
mod http;

pub use emails::{Attachment, Tag};

const DEFAULT_BASE_URL: &str = "https://api.resend.com";

async fn parse_response(res: reqwest::Response) -> Result<String, Error> {
    if res.status() != 200 && res.status() != 201 {
        return Err(Error::Resend(
            serde_json::from_str(&res.text().await.unwrap()).unwrap(),
        ));
    }

    res.text().await.map_err(Error::Client)
}

pub struct Client {
    client: http::Client,
}

impl Client {
    pub fn new(api_key: &str) -> Self {
        Self {
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

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

#[derive(Debug, Clone, Default, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ResendErrorResponse {
    name: String,
    status_code: u16,
    message: String,
}

#[cfg(test)]
mod test {
    use super::*;
    use dotenvy::dotenv;

    async fn test_send_email() {
        dotenv().ok();

        let api_key = std::env::var("RESEND_API_KEY").unwrap();
        let test_email_to = std::env::var("TEST_EMAIL_TO").unwrap();
        let test_email_from = std::env::var("TEST_EMAIL_FROM").unwrap();
        let c = Client::new(&api_key);

        let r = emails::SendEmailRequest::builder()
            .to(&[test_email_to])
            .from(&test_email_from)
            .subject("Test Email!")
            .text("Test email!")
            .build();

        emails::send(&c, r).await.unwrap();
    }
}

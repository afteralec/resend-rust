use std::fmt;

mod domains;
mod emails;
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

    pub async fn send_email(
        &self,
        r: emails::SendEmailRequest,
    ) -> Result<emails::SendEmailResponse, Error> {
        let request_json = serde_json::to_string(&r).map_err(Error::JSON)?;

        let url = format!("{}/emails", DEFAULT_BASE_URL);
        let request = http::Request::new(http::Method::Post, &url, &request_json);

        let response =
            parse_response(self.client.perform(request).await.map_err(Error::Client)?).await?;
        serde_json::from_str(&response).map_err(Error::JSON)
    }

    pub async fn get_email(&self, email_id: &str) -> Result<emails::Email, Error> {
        let url = format!("{}/emails/{}", DEFAULT_BASE_URL, email_id);
        let request = http::Request::new(http::Method::Get, &url, "");

        let response =
            parse_response(self.client.perform(request).await.map_err(Error::Client)?).await?;
        serde_json::from_str(&response).map_err(Error::JSON)
    }

    // TODO: Return an error if the input is longer than 100
    pub async fn batch_send_email(
        &self,
        r: &[emails::SendEmailRequest],
    ) -> Result<emails::BatchSendEmailResponse, Error> {
        let request_json = serde_json::to_string(&r).map_err(Error::JSON)?;

        let url = format!("{}/emails", DEFAULT_BASE_URL);
        let request = http::Request::new(http::Method::Post, &url, &request_json);

        let response =
            parse_response(self.client.perform(request).await.map_err(Error::Client)?).await?;
        serde_json::from_str(&response).map_err(Error::JSON)
    }

    pub async fn delete_domain(
        &self,
        r: domains::DeleteRequest,
    ) -> Result<domains::DeleteResponse, Error> {
        let url = format!("{}/domains/{}", DEFAULT_BASE_URL, r.domain_id);
        let request = http::Request::new(http::Method::Delete, &url, "");

        let response =
            parse_response(self.client.perform(request).await.map_err(Error::Client)?).await?;
        serde_json::from_str(&response).map_err(Error::JSON)
    }
}

#[derive(Debug)]
pub enum Error {
    Resend(ResendErrorResponse),
    // TODO: Implement this to wrap the underlying client error
    JSON(serde_json::Error),
    #[cfg(feature = "reqwest")]
    Client(reqwest::Error),
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

    #[tokio::test]
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

        c.send_email(r).await.unwrap();
    }
}

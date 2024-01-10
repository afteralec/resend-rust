use serde_derive::{Deserialize, Serialize};

mod emails;
mod http;

const CONTENT_TYPE: &str = "Content-Type";
const DEFAULT_BASE_URL: &str = "https://api.resend.com";

pub struct Client {
    api_key: String,
    client: reqwest::Client,
}

impl Client {
    pub fn new(api_key: &str) -> Self {
        Self {
            api_key: api_key.to_owned(),
            client: reqwest::Client::new(),
        }
    }

    pub async fn send_email(
        &self,
        r: emails::SendEmailRequest,
    ) -> anyhow::Result<emails::SendEmailResponse> {
        let request_json = serde_json::to_string(&r)?;

        let url = format!("{}/emails", DEFAULT_BASE_URL);
        let res = self
            .client
            .post(&url)
            .bearer_auth(&self.api_key)
            .header(CONTENT_TYPE, "application/json")
            .body(request_json)
            .send()
            .await?;

        let response_json = &res.text().await.unwrap();
        let deserialized: emails::SendEmailResponse = serde_json::from_str(response_json).unwrap();
        Ok(deserialized)
    }

    pub async fn get_email(&self, email_id: &str) -> anyhow::Result<emails::Email> {
        let url = format!("{}/emails/{}", DEFAULT_BASE_URL, email_id);
        let res = self
            .client
            .get(&url)
            .bearer_auth(&self.api_key)
            .send()
            .await?;

        let response_json = &res.text().await.unwrap();
        let deserialized: emails::Email = serde_json::from_str(response_json).unwrap();
        Ok(deserialized)
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
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

        c.send_email(r).await.unwrap();
    }
}

mod emails;
mod http;

pub use emails::{Attachment, Tag};

const DEFAULT_BASE_URL: &str = "https://api.resend.com";

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
    ) -> anyhow::Result<emails::SendEmailResponse> {
        let request_json = serde_json::to_string(&r)?;

        let url = format!("{}/emails", DEFAULT_BASE_URL);
        let request = http::Request::new(http::Method::Post, &url, &request_json);

        let response_json = self.client.perform(request).await?;
        let response: emails::SendEmailResponse = serde_json::from_str(&response_json).unwrap();
        Ok(response)
    }

    pub async fn get_email(&self, email_id: &str) -> anyhow::Result<emails::Email> {
        let url = format!("{}/emails/{}", DEFAULT_BASE_URL, email_id);
        let request = http::Request::new(http::Method::Get, &url, "");

        let response_json = self.client.perform(request).await?;
        let email: emails::Email = serde_json::from_str(&response_json).unwrap();
        Ok(email)
    }
}

#[derive(Debug, Clone, Default, serde_derive::Serialize, serde_derive::Deserialize)]
struct ResendErrorResponse {
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

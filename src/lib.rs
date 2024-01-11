mod emails;
mod http;

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

    #[cfg(feature = "serde")]
    pub async fn send_email(
        &self,
        r: emails::SendEmailRequest,
    ) -> anyhow::Result<emails::SendEmailResponse> {
        let response_json = self.send_email_internal(r).await?;
        let deserialized: emails::SendEmailResponse = serde_json::from_str(&response_json).unwrap();
        Ok(deserialized)
    }

    #[cfg(not(feature = "serde"))]
    pub async fn send_email(&self, r: emails::SendEmailRequest) -> anyhow::Result<String> {
        let response_json = self.send_email_interna(r).await?;
        Ok(response_json)
    }

    async fn send_email_internal(&self, r: emails::SendEmailRequest) -> anyhow::Result<String> {
        #[cfg(feature = "serde")]
        let request_json = serde_json::to_string(&r)?;
        #[cfg(not(feature = "serde"))]
        let request_json = r.to_string();

        let url = format!("{}/emails", DEFAULT_BASE_URL);
        let request = http::Request::new(http::Method::Post, &url, &request_json);

        let response_json = self.client.perform(request).await?;
        Ok(response_json)
    }

    pub async fn get_email(&self, email_id: &str) -> anyhow::Result<emails::Email> {
        let url = format!("{}/emails/{}", DEFAULT_BASE_URL, email_id);
        let request = http::Request::new(http::Method::Get, &url, "");

        let response_json = self.client.perform(request).await?;
        let deserialized: emails::Email = serde_json::from_str(&response_json).unwrap();
        Ok(deserialized)
    }
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
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

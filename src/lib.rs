use serde_derive::{Deserialize, Serialize};

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

    pub async fn send_email(&self, r: SendEmailRequest) -> anyhow::Result<reqwest::Response> {
        let request_json = serde_json::to_string(&r)?;

        println!("{:?}", request_json);

        let url = format!("{}/email", DEFAULT_BASE_URL);
        let res = self
            .client
            .post(&url)
            .bearer_auth(&self.api_key)
            .header(CONTENT_TYPE, "application/json")
            .body(request_json)
            .send()
            .await?;

        Ok(res)
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResendErrorResponse {
    name: String,
    status_code: u16,
    message: String,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct Attachment {
    content: Vec<u8>,
    #[serde(skip_serializing_if = "String::is_empty")]
    filename: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    path: String,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct Tag {
    #[serde(skip_serializing_if = "String::is_empty")]
    name: String,
    value: String,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct SendEmailRequest {
    #[serde(skip_serializing_if = "String::is_empty")]
    from: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    to: Vec<String>,
    #[serde(skip_serializing_if = "String::is_empty")]
    subject: String,

    bcc: Vec<String>,
    cc: Vec<String>,
    reply_to: Vec<String>,

    html: String,
    text: String,

    headers: hashbrown::HashMap<String, String>,
    attachments: Vec<Attachment>,

    tags: Vec<Tag>,
}

impl SendEmailRequest {
    pub fn builder() -> SendEmailRequestBuilder {
        SendEmailRequestBuilder::default()
    }
}

#[derive(Debug, Clone, Default)]
pub struct SendEmailRequestBuilder {
    from: String,
    to: Vec<String>,
    subject: String,

    bcc: Vec<String>,
    cc: Vec<String>,
    reply_to: Vec<String>,

    html: String,
    text: String,

    headers: hashbrown::HashMap<String, String>,
    attachments: Vec<Attachment>,

    tags: Vec<Tag>,
}

impl SendEmailRequestBuilder {
    pub fn build(self) -> SendEmailRequest {
        SendEmailRequest {
            from: self.from,
            to: self.to,
            subject: self.subject,
            bcc: self.bcc,
            cc: self.cc,
            reply_to: self.reply_to,
            html: self.html,
            text: self.text,
            headers: self.headers,
            attachments: self.attachments,
            tags: self.tags,
        }
    }

    pub fn from(mut self, from: &str) -> Self {
        self.from = from.to_owned();
        self
    }

    pub fn to(mut self, to: &[String]) -> Self {
        self.to = Vec::from(to);
        self
    }

    pub fn subject(mut self, subject: &str) -> Self {
        self.subject = subject.to_owned();
        self
    }

    pub fn bcc(mut self, bcc: &[String]) -> Self {
        self.bcc = Vec::from(bcc);
        self
    }

    pub fn cc(mut self, cc: &[String]) -> Self {
        self.cc = Vec::from(cc);
        self
    }

    pub fn reply_to(mut self, reply_to: &[String]) -> Self {
        self.reply_to = Vec::from(reply_to);
        self
    }

    pub fn html(mut self, html: &str) -> Self {
        self.html = html.to_owned();
        self
    }

    pub fn text(mut self, text: &str) -> Self {
        self.text = text.to_owned();
        self
    }

    pub fn headers(mut self, headers: hashbrown::HashMap<String, String>) -> Self {
        self.headers = headers;
        self
    }

    pub fn add_header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(key.to_owned(), value.to_owned());
        self
    }

    pub fn attachments(mut self, attachments: &[Attachment]) -> Self {
        self.attachments = Vec::from(attachments);
        self
    }

    pub fn tags(mut self, tags: &[Tag]) -> Self {
        self.tags = Vec::from(tags);
        self
    }
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

        let r = SendEmailRequest::builder()
            .to(&[test_email_to])
            .from(&test_email_from)
            .subject("Test Email!")
            .text("Test email!")
            .build();

        let res = c.send_email(r).await.unwrap();
        println!("{:?}", res.text().await)
    }
}

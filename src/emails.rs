use crate::{http, parse_response, Client, Error, DEFAULT_BASE_URL};

#[derive(serde_derive::Deserialize)]
pub struct SendEmailResponse {
    pub id: String,
}

#[derive(serde_derive::Deserialize)]
pub struct BatchSendEmailResponse {
    pub data: Vec<SendEmailResponse>,
}

#[derive(serde_derive::Deserialize)]
pub struct Email {
    pub id: String,
    // TODO: Type this - it's typed as 'object' in Resend's JSON
    pub object: String,
    pub to: Vec<String>,
    pub from: String,
    pub created_at: String,
    pub subject: String,
    pub html: String,
    pub text: String,
    pub bcc: Vec<String>,
    pub cc: Vec<String>,
    pub reply_to: Vec<String>,
    pub last_event: String,
}

#[derive(Debug, Clone, Default, serde_derive::Serialize)]
pub struct Attachment {
    pub content: Vec<u8>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "String::is_empty"))]
    pub filename: String,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "String::is_empty"))]
    pub path: String,
}

#[derive(Debug, Clone, Default, serde_derive::Serialize)]
pub struct Tag {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "String::is_empty"))]
    pub name: String,
    pub value: String,
}

impl Tag {
    pub fn new(name: &str, value: &str) -> Self {
        Self {
            name: name.to_owned(),
            value: value.to_owned(),
        }
    }
}

#[derive(Debug, Clone, Default, serde_derive::Serialize)]
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

pub async fn send(client: &Client, r: SendEmailRequest) -> Result<SendEmailResponse, Error> {
    let request_json = serde_json::to_string(&r).map_err(Error::JSON)?;

    let url = format!("{}/emails", DEFAULT_BASE_URL);
    let request = http::Request::new(http::Method::Post, &url, Some(request_json));

    let response = parse_response(client.perform(request).await.map_err(Error::Client)?).await?;
    serde_json::from_str(&response).map_err(Error::JSON)
}

pub async fn get(client: &Client, email_id: &str) -> Result<Email, Error> {
    let url = format!("{}/emails/{}", DEFAULT_BASE_URL, email_id);
    let request = http::Request::new(http::Method::Get, &url, None);

    let response = parse_response(client.perform(request).await.map_err(Error::Client)?).await?;
    serde_json::from_str(&response).map_err(Error::JSON)
}

use std::fmt;

#[cfg_attr(feature = "serde", derive(serde_derive::Deserialize))]
pub struct SendEmailResponse {
    pub id: String,
}

#[cfg_attr(feature = "serde", derive(serde_derive::Deserialize))]
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

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde_derive::Serialize))]
pub struct Attachment {
    content: Vec<u8>,
    #[serde(skip_serializing_if = "String::is_empty")]
    filename: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    path: String,
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde_derive::Serialize))]
pub struct Tag {
    #[serde(skip_serializing_if = "String::is_empty")]
    name: String,
    value: String,
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde_derive::Serialize))]
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

impl fmt::Display for SendEmailRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{\"from\": \"{}\", \"to\": \"{:?}\", \"subject\": \"{}\", \"bcc\": \"{:?}\", \"cc\": \"{:?}\", \"reply_to\": \"{:?}\", \"html\": \"{}\", \"text\": \"{}\", \"headers\": \"{:?}\", \"attachments\": \"{:?}\"}}",
            &self.from, &self.to, &self.subject, &self.bcc, &self.cc, &self.reply_to, &self.text, &self.html, &self.headers, &self.attachments
        )
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

    #[test]
    fn test_display() {
        dotenv().ok();

        let test_email_to = std::env::var("TEST_EMAIL_TO").unwrap();
        let test_email_from = std::env::var("TEST_EMAIL_FROM").unwrap();

        let r = SendEmailRequest::builder()
            .to(&[test_email_to])
            .from(&test_email_from)
            .subject("Test Email!")
            .text("Test email!")
            .build();

        let strung = format!("{}", r);

        println!("{}", strung);
    }
}

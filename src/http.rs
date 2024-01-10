pub struct Client {
    api_key: String,
    #[cfg(feature = "reqwest")]
    client: reqwest::Client,
}

impl Client {
    pub fn new(api_key: &str) -> Self {
        Self {
            api_key: api_key.to_owned(),
            client: reqwest::Client::new(),
        }
    }
}

pub struct Request {
    method: String,
}

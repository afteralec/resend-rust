use crate::{emails, http, parse_response, utils, Client, Error};

#[derive(serde_derive::Deserialize)]
pub struct BatchSendResponse {
    pub data: Vec<emails::SendEmailResponse>,
}

pub async fn send(
    client: &Client,
    r: &[emails::SendEmailRequest],
) -> Result<BatchSendResponse, Error> {
    let request_json = serde_json::to_string(&r).map_err(Error::JSON)?;

    let url = utils::url::emails::base(&client.base_url);
    let request = http::Request::new(http::Method::Post, &url, Some(request_json.to_string()));

    let response = parse_response(client.perform(request).await.map_err(Error::Client)?).await?;
    serde_json::from_str(&response).map_err(Error::JSON)
}

use crate::{http, parse_response, utils, Client, Error};

#[derive(serde_derive::Serialize)]
pub struct AddRequest {
    pub name: String,
    // TODO: Validate this region on the way in
    pub region: String,
}

#[derive(serde_derive::Deserialize)]
pub struct Domain {
    pub id: String,
    pub name: String,
    pub created_at: String,
    pub status: String,
    pub records: Option<Vec<Record>>,
    pub region: Option<String>,
    pub dns_provider: Option<String>,
}

#[derive(serde_derive::Deserialize)]
pub struct Record {
    pub record: String,
    pub name: String,
    pub r#type: String,
    pub ttl: String,
    pub status: Option<String>,
    pub value: String,
    pub priority: Option<usize>,
}

#[derive(serde_derive::Deserialize)]
pub struct VerifyResponse {
    pub object: String,
    pub id: String,
}

#[derive(serde_derive::Deserialize)]
pub struct DeleteResponse {
    // TODO: Find a way to type this
    pub object: String,
    pub id: String,
    pub deleted: bool,
}

pub async fn add(client: &Client, r: AddRequest) -> Result<Domain, Error> {
    let request_json = serde_json::to_string(&r).map_err(Error::JSON)?;

    let url = utils::url::domains::base(&client.base_url);
    let request = http::Request::new(http::Method::Post, &url, Some(request_json.to_string()));

    let response = parse_response(client.perform(request).await.map_err(Error::Client)?).await?;
    serde_json::from_str(&response).map_err(Error::JSON)
}

pub async fn get(client: &Client, domain_id: &str) -> Result<Domain, Error> {
    let url = utils::url::domains::with_id(&client.base_url, domain_id);
    let request = http::Request::new(http::Method::Get, &url, None);

    let response = parse_response(client.perform(request).await.map_err(Error::Client)?).await?;
    serde_json::from_str(&response).map_err(Error::JSON)
}

pub async fn verify(client: &Client, domain_id: &str) -> Result<VerifyResponse, Error> {
    let url = utils::url::domains::with_id(&client.base_url, domain_id);
    let request = http::Request::new(http::Method::Post, &url, None);

    let response = parse_response(client.perform(request).await.map_err(Error::Client)?).await?;
    serde_json::from_str(&response).map_err(Error::JSON)
}

#[derive(serde_derive::Deserialize)]
pub struct ListDomainsResponse {
    pub data: Vec<Domain>,
}

pub async fn list(client: &Client) -> Result<ListDomainsResponse, Error> {
    let url = utils::url::domains::base(&client.base_url);
    let request = http::Request::new(http::Method::Get, &url, None);

    let response = parse_response(client.perform(request).await.map_err(Error::Client)?).await?;
    serde_json::from_str(&response).map_err(Error::JSON)
}

pub async fn delete(client: &Client, domain_id: &str) -> Result<DeleteResponse, Error> {
    let url = utils::url::domains::with_id(&client.base_url, domain_id);
    let request = http::Request::new(http::Method::Delete, &url, None);

    let response = parse_response(client.perform(request).await.map_err(Error::Client)?).await?;
    serde_json::from_str(&response).map_err(Error::JSON)
}

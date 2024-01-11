use crate::{http, parse_response, utils, Client, Error};

const FULL_ACCESS: &str = "full_access";
const SENDING_ACCESS: &str = "sending_access";

#[derive(serde_derive::Serialize)]
pub struct CreateRequest {
    pub name: String,
    pub permission: Permission,
    pub domain_id: String,
}

#[derive(serde_derive::Serialize)]
pub enum Permission {
    #[serde(rename(serialize = "full_access"))]
    FullAccess,
    #[serde(rename(serialize = "sending_access"))]
    SendingAccess,
}

impl std::fmt::Display for Permission {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FullAccess => {
                write!(f, "{}", FULL_ACCESS)
            }
            Self::SendingAccess => {
                write!(f, "{}", SENDING_ACCESS)
            }
        }
    }
}

#[derive(serde_derive::Deserialize)]
pub struct CreateResponse {
    pub id: String,
    pub token: String,
}

#[derive(serde_derive::Deserialize)]
pub struct APIKey {
    pub id: String,
    pub name: String,
    pub created_at: String,
}

pub async fn create(client: &Client, r: CreateRequest) -> Result<CreateResponse, Error> {
    let request_json = serde_json::to_string(&r).map_err(Error::JSON)?;

    let url = utils::url::api_keys::base(&client.base_url);
    let request = http::Request::new(http::Method::Post, &url, Some(request_json.to_string()));

    let response = parse_response(client.perform(request).await.map_err(Error::Client)?).await?;
    serde_json::from_str(&response).map_err(Error::JSON)
}

#[derive(serde_derive::Deserialize)]
pub struct ListResponse {
    pub data: Vec<APIKey>,
}

pub async fn list(client: &Client) -> Result<ListResponse, Error> {
    let url = utils::url::api_keys::base(&client.base_url);
    let request = http::Request::new(http::Method::Get, &url, None);

    let response = parse_response(client.perform(request).await.map_err(Error::Client)?).await?;
    serde_json::from_str(&response).map_err(Error::JSON)
}

pub async fn delete(client: &Client, api_key_id: &str) -> Result<(), Error> {
    let url = utils::url::api_keys::with_id(&client.base_url, api_key_id);
    let request = http::Request::new(http::Method::Delete, &url, None);

    parse_response(client.perform(request).await.map_err(Error::Client)?).await?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_serialize_create_request() {
        let expected = "{\"permission\":\"full_access\",\"domain_id\":\"test-domain-id\"}";
        let json = serde_json::to_string(&CreateRequest {
            name: "test-api-key".to_owned(),
            permission: Permission::FullAccess,
            domain_id: "test-domain-id".to_owned(),
        })
        .unwrap();

        assert_eq!(expected, json);
    }
}

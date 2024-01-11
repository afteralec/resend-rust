const FULL_ACCESS: &str = "full_access";
const SENDING_ACCESS: &str = "sending_access";

#[derive(serde_derive::Serialize)]
pub struct CreateRequest {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_serialize_create_request() {
        let expected = "{\"permission\":\"full_access\",\"domain_id\":\"test-domain-id\"}";
        let json = serde_json::to_string(&CreateRequest {
            permission: Permission::FullAccess,
            domain_id: "test-domain-id".to_owned(),
        })
        .unwrap();

        assert_eq!(expected, json);
    }
}

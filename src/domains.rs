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
    pub records: Vec<Record>,
    pub region: String,
    pub dns_provider: String,
}

#[derive(serde_derive::Deserialize)]
pub struct Record {
    pub record: String,
    pub name: String,
    pub r#type: String,
    pub ttl: String,
    pub status: String,
    pub value: String,
    pub priority: usize,
}

#[derive(serde_derive::Deserialize)]
pub struct VerifyResponse {
    pub object: String,
    pub id: String,
}

#[derive(serde_derive::Serialize)]
pub struct DeleteRequest {
    pub domain_id: String,
}

#[derive(serde_derive::Deserialize)]
pub struct DeleteResponse {
    // TODO: Find a way to type this
    pub object: String,
    pub id: String,
    pub deleted: bool,
}

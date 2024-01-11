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

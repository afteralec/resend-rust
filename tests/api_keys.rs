use dotenvy::dotenv;
use resend::Client;

// TODO: This can't be tested on a free plan
#[tokio::test]
async fn test_add_list_and_delete_api_keys() -> anyhow::Result<()> {
    dotenv().ok();

    let api_key = std::env::var("RESEND_CLIENT_API_KEY").unwrap();

    let c = Client::new(&api_key);

    let domains = resend::domains::list(&c).await.unwrap();
    let domain = domains.data.first().unwrap();

    let api_keys = resend::api_keys::list(&c).await.unwrap().data;
    let canon_len = api_keys.len();

    let r = resend::api_keys::CreateRequest {
        name: "test-api-key".to_owned(),
        domain_id: domain.id.clone(),
        permission: resend::api_keys::Permission::SendingAccess,
    };

    let api_key = resend::api_keys::create(&c, r).await.unwrap();
    let api_keys = resend::api_keys::list(&c).await.unwrap().data;

    assert_eq!(canon_len + 1, api_keys.len());

    let retrieved_api_key = api_keys.first().unwrap();
    assert_eq!(api_key.id, retrieved_api_key.id);

    resend::api_keys::delete(&c, &api_key.id).await.unwrap();

    let api_keys = resend::api_keys::list(&c).await.unwrap().data;
    assert_eq!(canon_len, api_keys.len());

    Ok(())
}

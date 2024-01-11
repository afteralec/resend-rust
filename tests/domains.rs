use dotenvy::dotenv;
use resend_rust::Client;

// TODO: This can't be tested on a free plan
#[tokio::test]
#[ignore]
async fn test_add_get_list_and_delete_domains() -> anyhow::Result<()> {
    dotenv().ok();

    let api_key = std::env::var("RESEND_CLIENT_API_KEY").unwrap();

    let c = Client::new(&api_key);

    let r = resend_rust::domains::AddRequest {
        name: "test-resend-client.com".to_string(),
        region: "us-east-1".to_string(),
    };

    let domain = resend_rust::domains::add(&c, r).await.unwrap();
    let get_domain = resend_rust::domains::get(&c, &domain.id).await.unwrap();

    assert_eq!(domain.id, get_domain.id);

    resend_rust::domains::delete(&c, &domain.id).await.unwrap();

    Ok(())
}

use dotenvy::dotenv;
use resend_rust::Client;

#[tokio::test]
async fn test_send_and_get_email() -> anyhow::Result<()> {
    dotenv().ok();

    let api_key = std::env::var("RESEND_CLIENT_API_KEY").unwrap();
    let test_email_to = std::env::var("TEST_EMAIL_TO").unwrap();
    let test_email_from = std::env::var("TEST_EMAIL_FROM").unwrap();

    let c = Client::new(&api_key);

    let r = resend_rust::emails::SendEmailRequest::builder()
        .to(&[test_email_to.clone()])
        .from(&test_email_from)
        .subject("Test Email!")
        .text("Test email!")
        .build();

    let email = resend_rust::emails::send(&c, r).await.unwrap();
    let email = resend_rust::emails::get(&c, &email.id).await.unwrap();

    assert_eq!(email.from, test_email_from);
    for to in email.to {
        assert_eq!(to, test_email_to);
    }
    assert_eq!(email.text, "Test email!");
    assert_eq!(email.subject, "Test Email!");

    Ok(())
}

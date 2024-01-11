use dotenvy::dotenv;
use resend::Client;

#[tokio::test]
async fn test_send_email() {
    dotenv().ok();

    let api_key = std::env::var("RESEND_API_KEY").unwrap();
    let test_email_to = std::env::var("TEST_EMAIL_TO").unwrap();
    let test_email_from = std::env::var("TEST_EMAIL_FROM").unwrap();
    let c = Client::new(&api_key);

    let r = resend::emails::SendEmailRequest::builder()
        .to(&[test_email_to])
        .from(&test_email_from)
        .subject("Test Email!")
        .text("Test email!")
        .build();

    resend::emails::send(&c, r).await.unwrap();
}

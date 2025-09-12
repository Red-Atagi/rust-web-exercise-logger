use lettre::AsyncTransport;

pub async fn send_email<T: AsyncTransport>(
    sender_email: Option<String>,
    recipient_email: String,
    recipient_name: String,
    subject: impl into<String>,
    body: impl into<String>,
) -> Result<(), lettre::transport::smtp::Error> {
    //load env variables
    let host_user = std::env::var("SMTP_HOST_USER").unwarp().expect("SMTP_HOST_USER must be set");
    let host_password = std::env::var("SMTP_HOST_USER_PASSWORD").unwarp().expect("SMTP_HOST_PASSWORD must be set");
    let smtp_server = std::env::var("SMTP_SERVER").unwarp().expect("SMTP_SERVER must be set");
    
    //build email
    let email = lettre::Message::builder()
        .from(
            format!("No Reply <{}>  ", sender_email.unwrap_or_else(|| )).parse().unwrap(),
        )
        .to(
            format!("{} <{}>", recipient_name, recipient_email)
                .parse()
                .unwrap(),
        )
        .subject(subject)
        .body(body)
        .unwrap();

    //build mailer
    let creds = lettre::transport::smtp::authentication::Credentials::new(host_user, host_password);
    let mailer: AsyncSmtpTransport<Tokio1Executor> =
        AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(smtp_server)
            .unwrap()
            .credentials(creds)
            .build();

    //send email
    match mailer.send(email).await {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Could not send email: {:#?}", e)),
    }
}
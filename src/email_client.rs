use crate::domain::subscriber_email::SubscriberEmail;
use lettre::{transport::smtp::authentication::Credentials, Message, SmtpTransport, Transport};

pub struct EmailClient {
    smtp_host: String,
    from_addr: String,
    password: String,
}

impl EmailClient {
    pub fn new(smtp_host: String, from_addr: String, password: String) -> EmailClient {
        Self {
            smtp_host,
            from_addr,
            password,
        }
    }

    pub async fn send_email(
        &self,
        recipient: SubscriberEmail,
        subject: &str,
        _html_content: &str,
        _text_content: &str,
    ) -> Result<(), String> {
        let email = Message::builder()
            .from(self.from_addr.parse().unwrap())
            .to(recipient.as_ref().parse().unwrap())
            .subject(subject)
            .body("Rust test!".to_string())
            .expect("could not build email");

        let credentials = Credentials::new(self.from_addr.clone(), "smtp_password".to_string());

        let mailer = SmtpTransport::relay(self.smtp_host.as_str())
            .unwrap()
            .credentials(credentials)
            .build();

        // mailer.send(&email)
        // assert!(mailer.send(&email).is_ok());
        match mailer.send(&email) {
            Ok(_) => Ok(()),
            Err(_) => Err(format!("failed to send email for: {}", recipient.as_ref())),
        }

        // Ok(())
    }

    pub fn smtp_host(&self) -> &str {
        self.smtp_host.as_ref()
    }

    pub fn password(&self) -> &str {
        self.password.as_ref()
    }
}

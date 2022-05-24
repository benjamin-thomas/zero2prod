use std::time::Duration;

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

        let credentials = Credentials::new(self.from_addr.clone(), self.password().to_string());

        let mailer = SmtpTransport::starttls_relay(self.smtp_host.as_str())
            .unwrap()
            .credentials(credentials)
            .timeout(Some(Duration::from_millis(5000)))
            .build();

        // FIXME: don't know how to return errors more generically
        let res = mailer.send(&email);
        // println!("res:{:?}", &res);
        match res {
            Ok(_) => {
                println!("Successfully sent email to: {}", recipient.as_ref());
                Ok(())
            }
            Err(err) => Err(format!(
                "failed to send email for: {}\nerr:{}",
                recipient.as_ref(),
                err,
            )),
        }
    }

    pub fn smtp_host(&self) -> &str {
        self.smtp_host.as_ref()
    }

    pub fn password(&self) -> &str {
        self.password.as_ref()
    }
}

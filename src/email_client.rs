use crate::domain::subscriber_email::SubscriberEmail;

pub struct EmailClient {
    pub smtp_host: String,
    pub smtp_sender: SubscriberEmail,
}

impl EmailClient {
    pub fn new(smtp_host: String, smtp_sender: String) -> Result<Self, String> {
        let smtp_sender = SubscriberEmail::parse(smtp_sender)?;
        Ok(Self {
            smtp_host,
            smtp_sender,
        })
    }

    pub async fn send_email(
        &self,
        recipient: SubscriberEmail,
        subject: &str,
        html_content: &str,
        text_content: &str,
    ) -> Result<(), String> {
        todo!()
    }
}

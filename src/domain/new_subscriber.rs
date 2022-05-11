use crate::domain::subscriber_email::SubscriberEmail;
use crate::domain::subscriber_name::SubscriberName;

pub(crate) struct NewSubscriber {
    pub(crate) email: SubscriberEmail,
    pub(crate) name: SubscriberName,
}

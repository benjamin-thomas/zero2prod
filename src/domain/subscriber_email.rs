#[derive(Debug)]
pub(crate) struct SubscriberEmail(String);

impl SubscriberEmail {
    pub(crate) fn parse(s: String) -> Result<SubscriberEmail, String> {
        if validator::validate_email(&s) {
            Ok(Self(s))
        } else {
            Err(format!("Not a valid email: '{}'", s))
        }
    }
}

impl AsRef<str> for SubscriberEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::SubscriberEmail;
    use claim::{assert_err, assert_ok};

    #[test]
    fn empty_is_rejected() {
        assert_err!(SubscriberEmail::parse("".to_string()));
    }

    #[test]
    fn with_not_at_sign_is_rejected() {
        assert_err!(SubscriberEmail::parse("user.example.com".to_string()));
    }

    #[test]
    fn with_missing_first_part_is_rejected() {
        assert_err!(SubscriberEmail::parse("@example.com".to_string()));
    }

    #[test]
    fn confirms_a_valid_email() {
        assert_ok!(SubscriberEmail::parse("user@example.com".to_string()));
    }
}

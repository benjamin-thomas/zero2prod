use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub(crate) struct SubscriberName(String);

impl SubscriberName {
    pub(crate) fn parse(s: String) -> Result<SubscriberName, String> {
        let is_empty = s.trim().is_empty();
        let is_too_long = s.graphemes(true).count() > 255;
        let forbidden = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let contains_forbidden = s.chars().any(|c| forbidden.contains(&c));

        if is_empty || is_too_long || contains_forbidden {
            Err(format!("Rejected subscriber name: '{}'", s))
        } else {
            Ok(Self(s))
        }
    }
}

impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::subscriber_name::SubscriberName;
    use claim::{assert_err, assert_ok};

    #[test]
    fn length_255_is_ok() {
        assert_ok!(SubscriberName::parse("ë".repeat(255)));
    }

    #[test]
    fn length_256_is_too_long() {
        assert_err!(SubscriberName::parse("ë".repeat(256)));
    }

    #[test]
    fn reject_empty() {
        assert_err!(SubscriberName::parse("".to_string()));
        assert_err!(SubscriberName::parse(" ".to_string()));
    }

    #[test]
    fn reject_forbidden() {
        for name in &['/', '(', ')', '"', '<', '>', '\\', '{', '}'] {
            assert_err!(SubscriberName::parse(name.to_string()));
        }
    }

    #[test]
    fn valid_name() {
        assert_ok!(SubscriberName::parse("John".to_string()));
    }
}

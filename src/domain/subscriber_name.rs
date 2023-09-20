use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Clone)]
pub struct SubscriberName(String);

impl SubscriberName {
    const FORBIDDEN_CHARS: [char; 9] = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];

    pub fn parse(value: String) -> Result<Self, String> {
        let is_empty_or_whitespace = value.trim().is_empty();
        let is_too_long = value.graphemes(true).count() > 256;
        let contains_forbidden_chars = value
            .chars()
            .any(|g| SubscriberName::FORBIDDEN_CHARS.contains(&g));
        if is_empty_or_whitespace || is_too_long || contains_forbidden_chars {
            Err(format!("{value} is not a valid subscriber name."))
        } else {
            Ok(Self(value))
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
    use crate::domain::SubscriberName;
    use claims::{assert_err, assert_ok};

    #[test]
    fn a_256_grapheme_name_is_valid() {
        let name = "ę".repeat(256);
        assert_ok!(SubscriberName::parse(name));
    }

    #[test]
    fn a_name_longer_than_257_graphemes_is_rejected() {
        let name = "č".repeat(257);
        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    fn whitespace_only_names_are_rejected() {
        let name = " \n\t\r\n";
        assert_err!(SubscriberName::parse(name.to_owned()));
    }

    #[test]
    fn empty_string_is_rejected() {
        let name = "";
        assert_err!(SubscriberName::parse(name.to_owned()));
    }

    #[test]
    fn names_containing_invalid_characters_are_rejected() {
        for c in SubscriberName::FORBIDDEN_CHARS.iter() {
            let name = format!("my name{}", c);
            assert_err!(SubscriberName::parse(name));
        }
    }

    #[test]
    fn a_normal_name_is_parsed_successfully() {
        let name = "Adrien Wald".to_owned();
        let subscriber_name = SubscriberName::parse(name.clone());
        assert_ok!(subscriber_name.clone());
        assert_eq!(subscriber_name.unwrap().as_ref(), &name);
    }
}

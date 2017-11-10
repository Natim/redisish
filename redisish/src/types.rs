pub const DEFAULT_CHANNEL: &'static str = "default";

pub type Channel = String;
pub type Value = String;

#[derive(Debug, PartialEq)]
pub enum Message {
    Retrieve(Channel),
    Push(Channel, Value),
    Invalid(String),
}


impl From<String> for Message {
    fn from(raw: String) -> Self {
        let content = raw.trim();
        let mut parts = content.splitn(3, ' ');

        match (parts.next(), parts.next(), parts.next()) {
            (Some("RETRIEVE"), None, None) => {
                Message::Retrieve(String::from(DEFAULT_CHANNEL))
            },
            (Some("RETRIEVE"), Some(channel), None) => {
                Message::Retrieve(String::from(channel))
            },
            (Some("PUSH"), Some(value), None) => {
                Message::Push(String::from(DEFAULT_CHANNEL), String::from(value))
            },
            (Some("PUSH"), Some(channel), Some(value)) => {
                Message::Push(String::from(channel), String::from(value))
            },
            (_, _, _) => Message::Invalid(String::from(content))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_retrieve_without_channel() {
        assert_eq!(Message::from(String::from("RETRIEVE\n")),
                   Message::Retrieve(String::from(DEFAULT_CHANNEL)))
    }
    
    #[test]
    fn test_retrieve_with_channel() {
        assert_eq!(Message::from(String::from("RETRIEVE mago\n")),
                   Message::Retrieve(String::from("mago")))
    }

    #[test]
    fn test_push_without_channel() {
        assert_eq!(Message::from(String::from("PUSH mago\n")),
                   Message::Push(String::from(DEFAULT_CHANNEL), String::from("mago")))
    }

    #[test]
    fn test_push_with_channel() {
        assert_eq!(Message::from(String::from("PUSH names mago\n")),
                   Message::Push(String::from("names"), String::from("mago")))
    }

    #[test]
    fn test_invalid() {
        assert_eq!(Message::from(String::from("GET\n")),
                   Message::Invalid(String::from("GET")))
    }
}

pub const DEFAULT_CHANNEL: &'static str = "default";

pub type Channel = String;
pub type Value = String;

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

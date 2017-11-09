pub type Channel = String;
pub type Value = String;

pub enum Message {
    Retrieve(Channel),
    Push(Channel, Value),
    Invalid(String),
}

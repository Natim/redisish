pub enum Message {
    Retrieve,
    Push(String),
    Invalid(String),
}

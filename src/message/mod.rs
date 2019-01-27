// Include the `message` module, which is generated from message.proto.
pub mod message {
    include!(concat!(env!("OUT_DIR"), "/de.pollux.message.rs"));
}

pub fn create_chat_message(timestamp : i64, content : String) -> message::ChatMessage {
    let mut chat_message = message::ChatMessage::default();
    chat_message.timestamp = timestamp;
    chat_message.content = content;
    chat_message
}
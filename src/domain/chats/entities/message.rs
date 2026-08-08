use std::time::SystemTime;

use crate::domain::chats::enums::message_sender::MessageSender;

///N:1 with chats
pub struct Message {
    pub content: String,
    pub sender: MessageSender,
    //todo: is that the best way to use dates?
    pub sent_time: SystemTime,
}

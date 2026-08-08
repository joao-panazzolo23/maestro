use crate::domain::chats::entities::message::Message;

///1:N to messages
pub struct Chat {
    pub messages: Vec<Message>,
}

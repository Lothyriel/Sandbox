use crate::models::receipt::{MessageData, MessageReceipt};

impl MessageReceipt {
    pub fn should_answer(&self) -> Option<&str> {
        match &self.body.message_data {
            Some(MessageData::ExtendedTextMessage {
                extended_text_message_data,
            }) => {
                let number = self
                    .body
                    .instance_data
                    .wid
                    .strip_suffix("@c.us")
                    .expect("wid was expected to contain @c.us suffix");

                if extended_text_message_data.text.contains(number) {
                    Some(&extended_text_message_data.text)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

use serde::{Deserialize, Serialize};

pub mod receipt;

#[derive(Deserialize, Debug)]
pub struct ReceiptDeletedResponse {
    pub result: bool,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MessageSentResponse {
    pub id_message: String,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SendMessageRequest {
    pub chat_id: String,
    pub message: String,
    pub quoted_message_id: String,
}

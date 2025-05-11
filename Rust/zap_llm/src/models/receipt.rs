use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MessageReceipt {
    pub receipt_id: u64,
    pub body: Body,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Body {
    pub type_webhook: String,
    pub instance_data: InstanceData,
    pub timestamp: u64,
    pub id_message: String,
    pub sender_data: SenderData,
    pub message_data: Option<MessageData>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InstanceData {
    pub id_instance: u64,
    pub wid: String,
    pub type_instance: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SenderData {
    pub chat_id: String,
    pub chat_name: String,
    pub sender: String,
    pub sender_name: String,
    pub sender_contact_name: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "typeMessage")]
#[allow(clippy::enum_variant_names)]
pub enum MessageData {
    #[serde(rename_all = "camelCase")]
    StickerMessage { file_message_data: FileMessageData },
    #[serde(rename_all = "camelCase")]
    ImageMessage { file_message_data: FileMessageData },
    #[serde(rename_all = "camelCase")]
    ExtendedTextMessage {
        extended_text_message_data: ExtendedTextMessageData,
    },
    #[serde(rename_all = "camelCase")]
    QuotedMessage {
        quoted_message: Value,
        extended_text_message_data: QuotedMessageData,
    },
    #[serde(other)]
    Other,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FileMessageData {
    pub download_url: String,
    pub caption: String,
    pub file_name: String,
    pub jpeg_thumbnail: String,
    pub is_animated: bool,
    pub mime_type: String,
    pub forwarding_score: u8,
    pub is_forwarded: bool,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExtendedTextMessageData {
    pub text: String,
    pub description: String,
    pub title: String,
    pub preview_type: String,
    pub jpeg_thumbnail: String,
    pub forwarding_score: i32,
    pub is_forwarded: bool,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct QuotedMessageData {
    pub text: String,
    pub stanza_id: String,
    pub participant: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct QuotedMessage {
    pub stanza_id: String,
    pub participant: String,
    pub type_message: String,
    pub text_message: String,
    pub extended_text_message: ExtendedTextMessageData,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sticker_message_deserialization() {
        let json_data = r#"
        {
            "typeMessage": "stickerMessage",
            "fileMessageData": {
                "downloadUrl": "a.webp",
                "caption": "",
                "fileName": "a.webp",
                "jpegThumbnail": "",
                "isAnimated": false,
                "mimeType": "image/webp",
                "forwardingScore": 0,
                "isForwarded": false
            }
        }"#;

        let msg: MessageData = serde_json::from_str(json_data).unwrap();

        assert!(matches!(
            msg,
            MessageData::StickerMessage {
                file_message_data: _
            }
        ))
    }

    #[test]
    fn test_extended_text_message_deserialization() {
        let json_data = r#"
        {
            "typeMessage": "extendedTextMessage",
            "extendedTextMessageData": {
                "text": "só tenho essas de frifas",
                "description": "",
                "title": "",
                "previewType": "None",
                "jpegThumbnail": "",
                "forwardingScore": 0,
                "isForwarded": false
            }
        }"#;

        let msg: MessageData = serde_json::from_str(json_data).unwrap();

        assert!(matches!(
            msg,
            MessageData::ExtendedTextMessage {
                extended_text_message_data: _
            }
        ))
    }
}

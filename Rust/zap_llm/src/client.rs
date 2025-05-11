use std::{fs::File, io::Write};

use anyhow::Result;

use crate::models::{
    receipt::MessageReceipt, MessageSentResponse, ReceiptDeletedResponse, SendMessageRequest,
};

fn get_url(endpoint_name: &str, path_id: Option<&str>) -> String {
    let api_url = std::env::var("API_URL").expect("Missing env var API_URL");

    let id_instance = std::env::var("ID_INSTANCE").expect("Missing env var ID_INSTANCE");

    let api_token_instance =
        std::env::var("API_TOKEN_INSTANCE").expect("Missing env var API_TOKEN_INSTANCE");

    let path_id = path_id.unwrap_or_default();
    format!("{api_url}/waInstance{id_instance}/{endpoint_name}/{api_token_instance}/{path_id}")
}

#[derive(Clone)]
pub struct GreenApiClient {
    client: reqwest::Client,
}

impl GreenApiClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    pub async fn get_notification(&self) -> Result<Option<MessageReceipt>> {
        let url = get_url("receiveNotification", None);

        let response = self.client.get(url).send().await?;

        let text = response.text().await?;

        log_request(&text)?;

        let message = serde_json::from_str(&text)?;

        Ok(message)
    }

    pub async fn delete_notification(&self, receipt_id: u64) -> Result<ReceiptDeletedResponse> {
        let url = get_url("deleteNotification", Some(&receipt_id.to_string()));

        let response = self.client.delete(url).send().await?;

        let text = response.text().await?;

        log_request(&text)?;

        let message = serde_json::from_str(&text)?;

        Ok(message)
    }

    pub async fn quote_answer_message(
        &self,
        msg: SendMessageRequest,
    ) -> Result<MessageSentResponse> {
        let url = get_url("sendMessage", None);

        let response = self.client.post(url).json(&msg).send().await?;

        let text = response.text().await?;

        log_request(&text)?;

        let message = serde_json::from_str(&text)?;

        Ok(message)
    }
}

fn log_request(text: &String) -> Result<()> {
    let now = chrono::Utc::now().timestamp();

    std::fs::create_dir_all("logs/")?;

    File::create(format!("logs/{}.json", now))?.write_all(text.as_bytes())?;

    Ok(())
}

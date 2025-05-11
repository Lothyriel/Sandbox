mod client;
mod models;
mod utils;

use anyhow::Result;
use client::GreenApiClient;
use models::{receipt::MessageReceipt, SendMessageRequest};
use ollama_rs::{generation::completion::request::GenerationRequest, Ollama};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    if let Err(e) = start().await {
        eprintln!("Error: {e}");
    }
}

async fn start() -> Result<()> {
    println!("Started processing...");

    loop {
        if !process_message().await? {
            println!("No messages found...");
        }
    }
}

async fn process_message() -> Result<bool> {
    let client = client::GreenApiClient::new();

    let message = match client.get_notification().await? {
        Some(m) => m,
        None => return Ok(false),
    };

    println!("Sender: {}", message.body.sender_data.sender_name);
    println!("Message: {:?}", message.body.message_data);

    clear_msg(&client, message.receipt_id);

    handle_answer(message, &client).await?;

    Ok(true)
}

async fn handle_answer(message: MessageReceipt, client: &GreenApiClient) -> Result<()> {
    let text = match message.should_answer() {
        Some(t) => t,
        None => return Ok(()),
    };

    println!("Sending message to model...");

    let model_response = get_model_response(text.to_string()).await?;

    let msg = SendMessageRequest {
        chat_id: message.body.sender_data.chat_id,
        quoted_message_id: message.body.id_message,
        message: model_response,
    };

    client.quote_answer_message(msg).await?;

    Ok(())
}

async fn get_model_response(prompt: String) -> Result<String> {
    let model = "llama2:latest".to_string();

    let url = std::env::var("OLLAMA_URL").expect("Missing OLLAMA_URL env var");

    let ollama = Ollama::new(url, 11434);

    let res = ollama
        .generate(GenerationRequest::new(model, prompt))
        .await?;

    Ok(res.response)
}

fn clear_msg(client: &GreenApiClient, message_id: u64) {
    let c = client.clone();

    let task = async move {
        let deletion = c.delete_notification(message_id).await;

        let deletion = match deletion {
            Ok(d) => d,
            Err(e) => {
                println!("Error deleting message {message_id} | {e}");
                return;
            }
        };

        if deletion.result {
            println!("Message {} deleted from queue", message_id);
        } else {
            println!("Error deleting message: {}", message_id)
        }
    };

    tokio::spawn(task);
}

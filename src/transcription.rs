use reqwest::multipart::{Form, Part};
use reqwest::Client;
use serde_json::Value;
use std::env;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

pub async fn get_transcription(file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let api_key = env::var("OPENAI_API_KEY").unwrap();
    let client = Client::new();

    let mut file = File::open(file_path).await?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).await?;

    let file_name = std::path::Path::new(file_path)
        .file_name()
        .ok_or("Failed to extract file name from the path")?
        .to_str()
        .ok_or("Failed to convert the file name to a string")?;

    let file_part = Part::bytes(buffer).file_name(file_name.to_string());

    let form = Form::new()
        .text("model", "whisper-1")
        .part("file", file_part)
        .text("initial_prompt", "Una conversación entre un doctor y un paciente durante una cita médica")
        .text("language", "es");

    let res = client
        .post("https://api.openai.com/v1/audio/transcriptions")
        .header("Authorization", format!("Bearer {}", api_key))
        .multipart(form)
        .send()
        .await?
        .text()
        .await?;

    let parsed: Value = serde_json::from_str(&res)?;

    if let Some(Value::String(text)) = parsed.get("text") {
        Ok(text.clone())
    } else {
        Err("text field not found or is not a string".into())
    }
}

use openai_rust::chat::{ChatArguments, Message};
use openai_rust::futures_util::StreamExt;  // for the `.next()` method on streams
use std::io::{self, Write};  // for the .flush() method

pub async fn start_chat(dialog: String, system_prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = openai_rust::Client::new(&std::env::var("OPENAI_API_KEY").unwrap());
    let prompt = format!("summarize the following dialogue:\n{}", dialog);
    let args = ChatArguments::new("gpt-4", vec![
        Message {
            role: "system".to_owned(),
            content: system_prompt.to_owned(),
        },
        Message {
            role: "user".to_owned(),
            content: prompt,
        },
    ]);
    let mut res = client.create_chat_stream(args).await?;
    let mut chat_output = String::new();


    while let Some(events) = res.next().await {
        for event in events? {
            print!("{}", event);
            io::stdout().flush()?;  // Ensure output is displayed immediately
            chat_output.push_str(&event.to_string());
        }
    }

    Ok(chat_output)
}

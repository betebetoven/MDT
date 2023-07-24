mod transcription;
mod chat;
mod prompts;
mod ffmpeg;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "C:\\Users\\alber\\OneDrive\\Escritorio\\cys\\wisachindeploymenttest\\test4.mp3";
    
    let file_chunks = ffmpeg::split_audio(file_path).await?;

    let mut all_transcripts = Vec::new();
    println!("Starting the transcription process...");
    for chunk_path in file_chunks {
        println!("Transcribing chunk: {}", chunk_path);
        let dialog = transcription::get_transcription(&chunk_path).await?;
        all_transcripts.push(dialog);
    }
    println!("All chunks have been transcribed!");

    let full_dialog = all_transcripts.join("\n");

    // Define the system prompt
    println!("Starting the chat...");
    chat::start_chat(full_dialog, prompts::SYSTEM_PROMPT).await
}

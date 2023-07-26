use std::io::{ Result };
use std::path::Path;
use std::str::FromStr;
use tokio::fs;
use tokio::io::AsyncWriteExt as _;
use actix_cors::Cors;
use actix_web::{ HttpServer,
                 App,
                 HttpResponse,
                 HttpRequest,
                 web,
                 http::header::CONTENT_LENGTH };
use actix_multipart::{ Multipart };
use futures_util::{ TryStreamExt as _ };
use mime::{ Mime };
use uuid::Uuid;
mod transcription;
mod chat;
mod prompts;
mod ffmpeg;
//////
use openai_rust::chat::{ChatArguments, Message};
use openai_rust::futures_util::StreamExt;  // for the `.next()` method on streams
use std::io::{self, Write};


#[actix_web::main]
async fn main() -> Result<()> {
    if !Path::new("./upload").exists() {
        fs::create_dir("./upload").await?;
    }

    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .route("/", web::get().to(root))
            .route("/upload", web::post().to(upload))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

async fn root() -> String {
    "Server is up and running.".to_string()
}

async fn upload(mut payload: Multipart, req: HttpRequest) -> HttpResponse {
    
    let content_length: usize = match req.headers().get(CONTENT_LENGTH) {
        Some(header_value) => header_value.to_str().unwrap_or("0").parse().unwrap(),
        None => "0".parse().unwrap(),
    };

    let max_file_count: usize = 3;
    let max_file_size: usize = 50_000_000; // approximately 50 MB

    let audio_mpeg = mime::Mime::from_str("audio/mpeg").unwrap();
    let audio_wav = mime::Mime::from_str("audio/wav").unwrap();
    let audio_flac = mime::Mime::from_str("audio/flac").unwrap();
    let audio_3gpp = mime::Mime::from_str("audio/3gpp").unwrap();
    let audio_aac = mime::Mime::from_str("audio/aac").unwrap();
    let audio_m4a = mime::Mime::from_str("audio/x-m4a").unwrap();


    let legal_filetypes: [Mime; 6] = [ audio_mpeg,audio_wav, audio_flac, audio_3gpp, audio_aac, audio_m4a];


    let mut current_count: usize = 0;
    let dir: &str = "./upload/";

    if content_length > max_file_size { return HttpResponse::BadRequest().into(); }
    let mut chat_output = String::new(); 
    loop {
        if current_count == max_file_count { break; }
        if let Ok(Some(mut field)) = payload.try_next().await {
            let filetype: Option<&Mime> = field.content_type();
            if filetype.is_none() { continue; }
            if !legal_filetypes.contains(&filetype.unwrap()) { continue; }
            if field.name() != "avatar" { continue; }

            let destination: String = format!(
                "{}{}-{}",
                dir,
                Uuid::new_v4(),
                field.content_disposition().get_filename().unwrap()
            );

            let mut saved_file: fs::File = fs::File::create(&destination).await.unwrap();
            while let Ok(Some(chunk)) = field.try_next().await {
                let _ = saved_file.write_all(&chunk).await.unwrap();
            }
             //////COMIENZA TRANSCRIPCION
             println!("Starting the transcription process...");

             let mut all_transcripts = Vec::new();
 
             if content_length <= 25_000_000 {
                 // If file size is less than or equal to 25MB, transcribe directly.
                 println!("Transcribing only chunk: {}", destination);
                 match transcription::get_transcription(&destination).await {
                     Ok(dialog) => {
                         all_transcripts.push(dialog);
                         
                     },
                     Err(_) => return HttpResponse::InternalServerError().body("Failed to transcribe audio"),
                 };
                 let destination_clone = destination.clone();
                    tokio::spawn(async move {
                        if fs::remove_file(&destination_clone).await.is_ok() {
                            println!("File {} was removed successfully", &destination_clone);
                        }
                    });
             } else {
                 // If file size is larger than 25MB, split and transcribe chunks.
                 let file_chunks = match ffmpeg::split_audio(&destination).await {
                     Ok(chunks) => chunks,
                     Err(_) => return HttpResponse::InternalServerError().body("Failed to split audio"),
                 };
 
                 for chunk_path in file_chunks {
                     println!("Transcribing chunk: {}", chunk_path);
                     match transcription::get_transcription(&chunk_path).await {
                         Ok(dialog) => all_transcripts.push(dialog),
                         Err(_) => return HttpResponse::InternalServerError().body("Failed to transcribe audio"),
                     };
                     let chunk_path_clone = chunk_path.clone();
                     tokio::spawn(async move {
                         if fs::remove_file(&chunk_path_clone).await.is_ok() {
                             println!("File {} was removed successfully", &chunk_path_clone);
                         }
                     });
                 }
                 println!("All chunks have been transcribed!");
                 let destination_clone = destination.clone();
        tokio::spawn(async move {
            if fs::remove_file(&destination_clone).await.is_ok() {
                println!("File {} was removed successfully", &destination_clone);
            }
        });
                 
             }
 
             let full_dialog = all_transcripts.join("\n");
 
             // Define the system prompt
             println!("Starting the chat...");
             /*match chat::start_chat(full_dialog, prompts::SYSTEM_PROMPT).await {
                Ok(responses) => {
                    for response in responses {
                        println!("{}", response);
                    }
                    
                },
                Err(_) => return HttpResponse::InternalServerError().body("Failed to start chat"),
            };*/
            
            println!("Starting the chat...");

                let client = openai_rust::Client::new(&std::env::var("OPENAI_API_KEY").unwrap());
                let prompt = format!("summarize the following dialogue:\n{}", full_dialog);
                let args = ChatArguments::new("gpt-4", vec![
                    Message {
                        role: "system".to_owned(),
                        content: prompts::SYSTEM_PROMPT.to_owned(),
                    },
                    Message {
                        role: "user".to_owned(),
                        content: prompt,
                    },
                ]);
                let mut res = match client.create_chat_stream(args).await {
                    Ok(res) => res,
                    Err(_) => return HttpResponse::InternalServerError().body("Failed to start chat"),
                };
                
                while let Some(events) = res.next().await {
                    for event in match events {
                        Ok(event) => event,
                        Err(_) => return HttpResponse::InternalServerError().body("Failed during chat"),
                    } {
                        print!("{}", event);
                        io::stdout().flush().unwrap();  // Ensure output is displayed immediately
                        let event_str = event.to_string(); 
                        chat_output.push_str(&event_str);
                    }
                }
            
 
             current_count += 1;
         } else { break; }
     }
 
     HttpResponse::Ok().body(chat_output) 
 }
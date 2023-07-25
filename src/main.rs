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
use mime::{ Mime, IMAGE_PNG, IMAGE_JPEG, IMAGE_GIF };
use mime::MPEG;
use uuid::Uuid;
use image::{ DynamicImage, imageops::FilterType };

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
    .bind(("127.0.0.1", 8080))?
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


    let legal_filetypes: [Mime; 9] = [IMAGE_PNG, IMAGE_JPEG, IMAGE_GIF, audio_mpeg,audio_wav, audio_flac, audio_3gpp, audio_aac, audio_m4a];


    let mut current_count: usize = 0;
    let dir: &str = "./upload/";

    if content_length > max_file_size { return HttpResponse::BadRequest().into(); }

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

            current_count += 1;
        } else { break; }
    }

    HttpResponse::Ok().into()
}
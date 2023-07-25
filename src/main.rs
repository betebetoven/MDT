use std::io::{ Result };
use std::path::Path;
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
    // 1. limit file size             done
    // 2. limit file count            done
    // 3. limit file type             done
    // 4. check if correct field      done
    // 5. convert to *gif             done
    // 6. save under random name      done
    let content_length: usize = match req.headers().get(CONTENT_LENGTH) {
        Some(header_value) => header_value.to_str().unwrap_or("0").parse().unwrap(),
        None => "0".parse().unwrap(),
    };

    let max_file_count: usize = 3;
    let max_file_size: usize = 10_000_000; // approximately 10 MB
    let legal_filetypes: [Mime; 3] = [IMAGE_PNG, IMAGE_JPEG, IMAGE_GIF];
    let mut current_count: usize = 0;
    let dir: &str = "./upload/";

    if content_length > max_file_size { return HttpResponse::BadRequest().into(); }
    print!("si lelga aqui");
    loop {
        if current_count == max_file_count { break; }
        if let Ok(Some(mut field)) = payload.try_next().await {
            let filetype: Option<&Mime> = field.content_type();
            if filetype.is_none() { continue; }
            if !legal_filetypes.contains(&filetype.unwrap()) { continue; }
            if field.name() != "avatar" { continue; }

             println!("content_length: {:#?}", content_length);
             println!("{}. picture:", current_count);
             println!("name {}", field.name()); // &str
             //println!("headers {}", field.headers());
             //println!("content type {}", field.content_type()); // &Mime
             //println!("content type is mime::IMAGE_PNG {}", field.content_type() == &IMAGE_PNG);

            // In a multipart/form-data body, the HTTP Content-Disposition general header is a header that can be used on the subpart of a multipart body to give information about the field it applies to. The subpart is delimited by the boundary defined in the Content-Type header. Used on the body itself, Content-Disposition has no effect.
             println!("content disposition {}", field.content_disposition()); // &ContentDisposition

             println!("filename {}", field.content_disposition().get_filename().unwrap()); // Option<&str>
            
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

            web::block(move || async move {
                let uploaded_img: DynamicImage = image::open(&destination).unwrap();
                let _ = fs::remove_file(&destination).await.unwrap();
                uploaded_img
                    .resize_exact(200, 200, FilterType::Gaussian)
                    .save(format!("{}{}.gif", dir, Uuid::new_v4().to_string())).unwrap();
            }).await.unwrap().await;

        } else { break; }
        current_count += 1;
    }

    HttpResponse::Ok().into()
}

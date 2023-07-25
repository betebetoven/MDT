use std::process::Command;
use std::error::Error;
use tokio::fs;

pub async fn split_audio(file_path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let path = std::path::Path::new(file_path);

    let file_stem = path
        .file_stem()
        .ok_or("Failed to extract file stem from the path")?
        .to_str()
        .ok_or("Failed to convert the file stem to a string")?;

    let file_extension = path
        .extension()
        .ok_or("Failed to extract file extension from the path")?
        .to_str()
        .ok_or("Failed to convert the file extension to a string")?;

    /*let command = format!(
        "ffmpeg -i {} -f segment -segment_time 600 -c copy {}_%03d.{}",
        file_path, file_stem, file_extension
    );*/

    let output = Command::new("ffmpeg")
    .arg("-i")
    .arg(file_path)
    .arg("-f")
    .arg("segment")
    .arg("-segment_time")
    .arg("600")
    .arg("-c")
    .arg("copy")
    .arg(format!("{}_%03d.{}", file_stem, file_extension))
    .output()?;


    if !output.status.success() {
        return Err(format!(
            "ffmpeg error: {}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }

    let mut output_files = vec![];

    let mut dir = fs::read_dir("./").await?;
    while let Some(res) = dir.next_entry().await? {
        let path = res.path();
        if path.is_file() && path.extension() == Some(std::ffi::OsStr::new(file_extension)) {
            if let Some(stem_str) = path.file_stem().and_then(|s| s.to_str()) {
                if stem_str.starts_with(file_stem) {
                    if let Some(path_str) = path.to_str() {
                        output_files.push(path_str.to_string());
                    }
                }
            }
        }
    }


    Ok(output_files)
}

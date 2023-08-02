use reqwest::blocking::multipart::{Form, Part};
use reqwest::blocking::Client;
use std::fs::{self, File};
use std::io::Read;
use std::path::Path;

use clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
    /// The path to the directory with gpx files
    path: std::path::PathBuf,
    /// Access token
    access_token: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let upload_url = "https://www.strava.com/api/v3/uploads";

    let args = Cli::parse();
    let client = Client::new();

    let entries = fs::read_dir(args.path)?;
    for entry in entries {
        let entry = entry?;
        let file_path = entry.path();
        let file_name = file_path
            .file_name()
            .and_then(|s| s.to_str())
            .map(|s| s.to_string())
            .expect("filename");

        let mut file_content = Vec::new();
        let mut file = File::open(file_path.clone())?;
        file.read_to_end(&mut file_content)?;

        let form = Form::new().text("data_type", "gpx").part(
            "file",
            Part::bytes(file_content).file_name(file_name.clone()),
        );

        let request_builder = client
            .post(upload_url)
            .multipart(form)
            .bearer_auth(&args.access_token);
        let response = request_builder.send()?;

        if response.status().is_success() {
            let done_folder = Path::new("./uploaded");
            fs::create_dir_all(done_folder)?;
            let new_file_path = done_folder.join(file_name.clone());
            fs::rename(file_path, &new_file_path)?;

            println!("→ parsed {:?}", file_name);
        } else {
            if response.status() == 429 {
                println!("→ rate limit has been hit - continue in 15 minutes")
            } else {
                println!("→ {:?}", response.text()?);
            }

            return Ok(());
        }
    }

    Ok(())
}

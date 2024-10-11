use std::{
    env, fs::File, io::{Cursor, Read}, path::PathBuf
};

use flate2::bufread::GzDecoder;
use reqwest::{header::{HeaderMap, HeaderValue, USER_AGENT}, Client};
use tar::Archive;
use serde_json::Value;

use crate::Result;

pub async fn upgrade() -> Result<()> {
    println!("upgrade");

    let latest_version = {
        let client = Client::new();

        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static("Rune CLI (https://rune.sh)"));

        let response = client
            .get("https://api.github.com/repos/rune-runtime/rune/tags")
            .headers(headers)
            .send()
            .await?;

        let json = response.text().await?;
        let value: Value = serde_json::from_str(&json)?;
        value[0]["name"].as_str().unwrap().to_string()
    };
    let platform = format!("{}-{}", env::consts::OS, env::consts::ARCH);
    let tarball_name = format!("rune-cli-{latest_version}-{platform}.tar.gz");
    let tarball_url = format!("https://github.com/rune-runtime/rune/releases/download/{latest_version}/{tarball_name}");

    let tmp_path = download_bin(
        &latest_version,
        &tarball_url,
    ).await?;

    let mut tar_gz = File::open(tmp_path.clone())?;
    let mut tar_bytes = Vec::new();
    tar_gz.read_to_end(&mut tar_bytes)?;
    let tar = GzDecoder::new(&tar_bytes[..]);
    let mut archive = Archive::new(&tar_bytes[..]);
    archive.unpack(".")?;

    let new_bin = tmp_path.join("rune-cli");
    self_replace::self_replace(new_bin)?;

    Ok(())
}

async fn download_bin(version: &str, url: &str) -> Result<PathBuf> {
    let response = reqwest::get(url).await?;

    let tmp_dir = tempfile::Builder::new()
        .prefix(&format!(".update-{version}"))
        .tempdir_in(::std::env::current_dir()?)?;

    let tmp_path = tmp_dir.path().to_path_buf();

    let mut file = std::fs::File::open(tmp_dir)?;
    let mut content = Cursor::new(response.bytes().await?);
    std::io::copy(&mut content, &mut file)?;
    Ok(tmp_path)
}

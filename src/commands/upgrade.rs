use std::{
    fs::File,
    io::{Cursor, Read},
    path::PathBuf,
};

use flate2::bufread::GzDecoder;
use tar::Archive;

use crate::Result;

pub async fn upgrade() -> Result<()> {
    // TODO: Get latest version & compare to current
    let latest_version = "";
    let platform = "";
    let bin_name = format!("rune-{latest_version}-{platform}");

    let tmp_path = download_bin(
        latest_version,
        &format!("https://rune.sh/releases/{bin_name}"),
    )
    .await?;

    let mut tar_gz = File::open(tmp_path.clone())?;
    let mut tar_bytes = Vec::new();
    tar_gz.read_to_end(&mut tar_bytes)?;
    let tar = GzDecoder::new(&tar_bytes[..]);
    let mut archive = Archive::new(&tar_bytes[..]);
    archive.unpack(".")?;

    let new_bin = tmp_path.join(bin_name);
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

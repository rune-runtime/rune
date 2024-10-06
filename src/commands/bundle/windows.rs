use std::{fs, path::{Path, PathBuf}};

type Package = msi::Package<fs::File>;

use crate::settings::Settings;

pub fn bundle_project(settings: &Settings) -> crate::Result<()> {
    let msi_bundle_name = format!("{}.msi", settings.bundle_name);
    let msi_path = settings
        .build_output_dir
        .join("bundle/windows")
        .join(&msi_bundle_name);

    if msi_path.exists() {
        fs::remove_dir_all(&msi_path)?;
    }

    let mut package = new_empty_package(&msi_path)?;

    Ok(())
}

fn new_empty_package(msi_path: &Path) -> crate::Result<Package> {
    if let Some(parent) = msi_path.parent() {
        fs::create_dir_all(parent)?;
    }
    let msi_file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(msi_path)?;
    let package = msi::Package::create(msi::PackageType::Installer, msi_file)?;
    Ok(package)
}

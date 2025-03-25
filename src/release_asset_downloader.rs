use reqwest::header::HeaderValue;
use std::path::Path;

use crate::{
    Args, release_inspector::InspectReleaseResponseSuccess, reqwest_builder,
    responses::ResponseError,
};

pub fn download_release_assets(
    Args {
        personal_access_token,
        output_directory,
        ..
    }: &Args,
    release: &InspectReleaseResponseSuccess,
) -> anyhow::Result<()> {
    let path = if let Some(output_directory) = output_directory {
        Path::new(output_directory).to_path_buf()
    } else {
        std::env::current_dir()?.join(&release.tag_name)
    };

    if path.is_file() {
        Err(anyhow::anyhow!("Output directory must be a directory"))?;
    }
    if !path.exists() {
        std::fs::create_dir_all(&path)?;
    }

    let mut header_map = reqwest::header::HeaderMap::new();
    header_map.append("Accept", HeaderValue::from_str("application/octet-stream")?);

    let client = reqwest_builder::get_builder(personal_access_token)?
        .default_headers(header_map)
        .build()?;

    for asset in &release.assets {
        let response = client.get(&asset.url).send()?;

        let payload = match response.status() {
            reqwest::StatusCode::OK => response.bytes()?,
            reqwest::StatusCode::UNAUTHORIZED => {
                let response = response.json::<ResponseError>()?;
                dbg!(response);
                Err(anyhow::anyhow!(
                    "Unauthorized, most likely this repo is private. Please either provide a personal access token or check that the token is valid."
                ))?
            }
            _ => Err(anyhow::anyhow!("Failed to get asset"))?,
        };

        match std::fs::write(path.join(&asset.asset_name), payload) {
            Ok(_) => println!("✓ Successfully written to disk: {}", asset.asset_name),
            Err(e) => println!("✗ Failed to write file: {}", e),
        }
    }

    Ok(())
}

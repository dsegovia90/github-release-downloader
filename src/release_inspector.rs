use crate::{Args, github_response_error::ResponseError, reqwest_builder};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Asset {
    #[serde(rename = "name")]
    pub asset_name: String,
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct InspectReleaseResponseSuccess {
    pub tag_name: String,
    pub name: String,
    pub assets: Vec<Asset>,
}

pub fn fetch_release(
    Args {
        repo_owner,
        repo_name,
        personal_access_token,
        ..
    }: &Args,
) -> anyhow::Result<InspectReleaseResponseSuccess> {
    let url = format!("https://api.github.com/repos/{repo_owner}/{repo_name}/releases/latest",);
    let client = reqwest_builder::get_builder(personal_access_token)?.build();
    let response = client?.get(&url).send()?;

    let release = match response.status() {
        reqwest::StatusCode::OK => response
            .json::<InspectReleaseResponseSuccess>()
            .map_err(|e| anyhow::anyhow!(e))?,
        reqwest::StatusCode::NOT_FOUND => {
            let response = response.json::<ResponseError>()?;
            dbg!(response);
            Err(anyhow::anyhow!(
                "Repository not found, check repo_owner and repo_name."
            ))?
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            let response = response.json::<ResponseError>()?;
            dbg!(response);
            Err(anyhow::anyhow!(
                "Unauthorized, most likely this repo is private. Please either provide a personal access token or check that the token is valid."
            ))?
        }
        _ => Err(anyhow::anyhow!(
            "Unauthorized, most likely this repo is private. Please either provide a personal access token or check that the token is valid."
        ))?,
    };

    println!(
        "Found release: {} ({}) with {} assets",
        release.name,
        release.tag_name,
        release.assets.len()
    );

    Ok(release)
}

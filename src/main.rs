use clap::Parser;
use secrecy::SecretString;
mod github_response_error;
mod release_asset_downloader;
mod release_inspector;
mod reqwest_builder;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
    /// Repository owner
    pub repo_owner: String,
    /// Repository name
    pub repo_name: String,
    /// Optional Github Personal Access Token, required for private repositories. Found at https://github.com/settings/personal-access-tokens,
    /// the only permission required is `Read-only` under `Contents` for the repo you want to download assets from.
    pub personal_access_token: Option<SecretString>,
    /// Optional output directory, defaults to current directory
    #[arg(short, long)]
    pub output_directory: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let cli = Args::parse();

    println!(
        "Downloading assets from latest release of {}/{}...",
        cli.repo_owner, cli.repo_name
    );

    let release = release_inspector::fetch_release(&cli)?;
    release_asset_downloader::download_release_assets(&cli, &release)?;

    Ok(())
}

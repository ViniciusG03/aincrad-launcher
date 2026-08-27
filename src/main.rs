use aincrad_launcher::minecraft_http_fetcher::ReqwestMinecraftHttpFetcher;
use aincrad_launcher::minecraft_version_manifest::parse_minecraft_version_manifest;
use reqwest::blocking::Client;
use std::time::Duration;

const MINECRAFT_MANIFEST_URL: &str =
    "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let http_client = Client::builder().timeout(Duration::from_secs(10)).build()?;

    let minecraft_http_fetcher = ReqwestMinecraftHttpFetcher::new(http_client);

    let manifest_json = minecraft_http_fetcher.fetch_text(MINECRAFT_MANIFEST_URL)?;

    let manifest = parse_minecraft_version_manifest(&manifest_json)?;
    for minecraft_version in &manifest.versions {
        println!(
            "{} {}",
            minecraft_version.id, minecraft_version.version_type
        );
    }
    Ok(())
}

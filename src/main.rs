use aincrad_launcher::minecraft_version_manifest::parse_minecraft_version_manifest;
use aincrad_launcher::minecraft_version_manifest_fetcher::ReqwestMinecraftManifestTextFetcher;
use reqwest::blocking::Client;
use std::time::Duration;

const MINECRAFT_MANIFEST_URL: &str =
    "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let http_client = Client::builder().timeout(Duration::from_secs(10)).build()?;

    let manifest_text_fetcher = ReqwestMinecraftManifestTextFetcher::new(http_client);

    let manifest_json = manifest_text_fetcher.fetch_manifest_text(MINECRAFT_MANIFEST_URL)?;

    let manifest = parse_minecraft_version_manifest(&manifest_json)?;
    for minecraft_version in &manifest.versions {
        println!(
            "{} {}",
            minecraft_version.id, minecraft_version.version_type
        );
    }
    Ok(())
}

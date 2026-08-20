use serde::Deserialize;

#[derive(Deserialize)]
struct LatestMinecraftVersions {
    #[serde(rename = "release")]
    release_id: String,
    #[serde(rename = "snapshot")]
    snapshot_id: String,
}

#[derive(Deserialize)]
struct MinecraftVersionManifest {
    latest: LatestMinecraftVersions,
}

fn parse_minecraft_version_manifest(
    manifest_json: &str,
) -> Result<MinecraftVersionManifest, serde_json::Error> {
    serde_json::from_str(manifest_json)
}

fn main() {
    let manifest_json: &str = r#"
    {
        "latest": {
            "release": "26.2",
            "snapshot": "26.3-snapshot-9"
        },
        "versions": []
    }
    "#;

    let manifest_result = parse_minecraft_version_manifest(manifest_json);

    let manifest = match manifest_result {
        Ok(parsed_manifest) => parsed_manifest,
        Err(parse_error) => {
            eprintln!(
                "Failed to parse Minecraft version manifest from JSON `{}`: {}",
                manifest_json, parse_error
            );
            return;
        }
    };

    println!("Latest release: {}", manifest.latest.release_id);
    println!("Latest snapshot: {}", manifest.latest.snapshot_id);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_minecraft_manifest_versions_from_valid_json() {
        let manifest_json = r#"
        {
            "latest": {
                "release": "26.2",
                "snapshot": "26.3-snapshot-9"
            },
            "versions": []
        }
        "#;

        let manifest = parse_minecraft_version_manifest(manifest_json)
            .expect("valid Minecraft version manifest JSON should parse");

        assert_eq!(manifest.latest.release_id, "26.2");
        assert_eq!(manifest.latest.snapshot_id, "26.3-snapshot-9");
    }

    #[test]
    fn rejects_minecraft_manifest_without_snapshot() {
        let manifest_json = r#"
        {
            "latest": {
                "release": "26.2"
            },
            "versions": []
        }
        "#;

        let parse_manifest = parse_minecraft_version_manifest(manifest_json);

        assert!(parse_manifest.is_err());
    }
}

use serde::Deserialize;

#[derive(Deserialize)]
pub struct MinecraftVersionMetadata {
    pub id: String,
    #[serde(rename = "type")]
    pub version_type: String,
    pub downloads: MinecraftVersionDownloads,
}

#[derive(Deserialize)]
pub struct MinecraftVersionDownloads {
    pub client: MinecraftClientDownload,
}

#[derive(Deserialize)]
pub struct MinecraftClientDownload {
    pub sha1: String,
    pub size: u64,
    pub url: String,
}

/// Parses Minecraft metadata JSON without performing network I/O.
///
/// # Example
///
/// ```text
/// let parse_result = parse_minecraft_version_metadata(metadata_json);
/// ```
pub fn parse_minecraft_version_metadata(
    metadata_json: &str,
) -> Result<MinecraftVersionMetadata, serde_json::Error> {
    serde_json::from_str(metadata_json)
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_MINECRAFT_VERSION_METADATA_JSON: &str = r#"
        {
            "id": "26.2",
            "type": "release",
            "mainClass": "net.minecraft.client.main.Main",
            "downloads": {
                "client": {
                    "sha1": "0123456789abcdef0123456789abcdef01234567",
                    "size": 123456,
                    "url": "https://example.invalid/client.jar"
                }
            },
            "libraries": [],
            "assetIndex": {
                "id": "34",
                "sha1": "abcdef0123456789abcdef0123456789abcdef01",
                "size": 1234,
                "totalSize": 5678,
                "url": "https://example.invalid/assets.json"
            }
        }
        "#;

    const MINECRAFT_VERSION_METADATA_WITHOUT_ID_JSON: &str = r#"
        {
            "type": "release",
            "downloads": {
                "client": {
                    "sha1": "0123456789abcdef0123456789abcdef01234567",
                    "size": 123456,
                    "url": "https://example.invalid/client.jar"
                }
            },
            "libraries": []
        }
        "#;

    #[test]
    fn parses_minecraft_version_metadata_from_valid_json() {
        let minecraft_version_metadata =
            parse_minecraft_version_metadata(VALID_MINECRAFT_VERSION_METADATA_JSON)
                .expect("valid Minecraft metadata JSON should parse");

        assert_eq!(minecraft_version_metadata.id, "26.2");
        assert_eq!(minecraft_version_metadata.version_type, "release");
        assert_eq!(
            minecraft_version_metadata.downloads.client.sha1,
            "0123456789abcdef0123456789abcdef01234567"
        );
        assert_eq!(minecraft_version_metadata.downloads.client.size, 123456_u64);
        assert_eq!(
            minecraft_version_metadata.downloads.client.url,
            "https://example.invalid/client.jar"
        );
    }

    #[test]
    fn rejects_minecraft_version_metadata_without_id() {
        let metadata_result =
            parse_minecraft_version_metadata(MINECRAFT_VERSION_METADATA_WITHOUT_ID_JSON);

        let parse_error = metadata_result
            .err()
            .expect("metadata version without id should fail to parse");

        assert!(
            parse_error.to_string().contains("missing field `id`"),
            "expected missing id error, got: {parse_error}"
        );
    }

    #[test]
    fn rejects_minecraft_version_metadata_without_client_download() {
        let metadata_json = r#"
        {
            "id": "26.2",
            "type": "release",
            "mainClass": "net.minecraft.client.main.Main",
            "downloads": {},
            "libraries": [],
            "assetIndex": {
                "id": "34",
                "sha1": "abcdef0123456789abcdef0123456789abcdef01",
                "size": 1234,
                "totalSize": 5678,
                "url": "https://example.invalid/assets.json"
            }
        }
        "#;

        let minecraft_version_metadata = parse_minecraft_version_metadata(metadata_json);

        let parse_error = minecraft_version_metadata
            .err()
            .expect("metadata version without client should fail to parse");

        assert!(
            parse_error.to_string().contains("missing field `client`"),
            "expected missing client error, got: {parse_error}"
        );
    }
}

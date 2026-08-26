use serde::Deserialize;

#[derive(Deserialize)]
pub struct LatestMinecraftVersions {
    #[serde(rename = "release")]
    pub release_id: String,
    #[serde(rename = "snapshot")]
    pub snapshot_id: String,
}

#[derive(Deserialize)]
pub struct MinecraftVersionManifest {
    pub latest: LatestMinecraftVersions,
    pub versions: Vec<MinecraftVersionSummary>,
}

impl MinecraftVersionManifest {
    /// Finds the Minecraft version summary with the requested identifier.
    ///
    /// Returns `None` when the manifest does not contain an exact match.
    ///
    /// # Example
    ///
    /// ```text
    /// let version_summary = manifest
    ///     .find_minecraft_version_summary_by_id("26.2");
    /// ```
    pub fn find_minecraft_version_summary_by_id(
        &self,
        requested_version_id: &str,
    ) -> Option<&MinecraftVersionSummary> {
        self.versions
            .iter()
            .find(|version| version.id == requested_version_id)
    }
}

#[derive(Deserialize)]
pub struct MinecraftVersionSummary {
    pub id: String,

    #[serde(rename = "type")]
    pub version_type: String,

    #[serde(rename = "url")]
    pub metadata_url: String,

    #[serde(rename = "sha1")]
    pub metadata_sha1: String,
}

/// Parses Minecraft version manifest JSON without performing network I/O.
///
/// # Example
///
/// ```text
/// let parse_result = parse_minecraft_version_manifest(manifest_json);
/// ```
pub fn parse_minecraft_version_manifest(
    manifest_json: &str,
) -> Result<MinecraftVersionManifest, serde_json::Error> {
    serde_json::from_str(manifest_json)
}

#[cfg(test)]
mod tests {
    use super::*;

    const MANIFEST_WITH_TWO_VERSIONS_JSON: &str = r#"
        {
            "latest": {
                "release": "26.2",
                "snapshot": "26.3-snapshot-9"
            },
            "versions": [
                {
                    "id": "26.2",
                    "type": "release",
                    "url": "https://piston-meta.mojang.com/v1/packages/f9e9e12c8b96ea9e04b7f87e36583a6ba0e2e9e6/26.2.json",
                    "sha1": "f9e9e12c8b96ea9e04b7f87e36583a6ba0e2e9e6"
                },
                {
                    "id": "26.3-snapshot-1",
                    "type": "snapshot",
                    "url": "https://piston-meta.mojang.com/v1/packages/aa0814284a3845e2b6eaccdf7d357c189c9c895b/26.3-snapshot-1.json",
                    "sha1": "aa0814284a3845e2b6eaccdf7d357c189c9c895b"
                }
            ]
        }
        "#;

    #[test]
    fn rejects_minecraft_manifest_without_snapshot() {
        let manifest_json = r#"
        {
            "latest": {
                "release": "26.2"
            },
            "versions": [
                {
                    "id": "26.2",
                    "type": "release",
                    "url": "https://example.invalid/26.2.json",
                    "sha1": "46fb31ca7e74aea93545df8f1aa14b9d670097d3"
                }
            ]
        }
        "#;

        let parse_result = parse_minecraft_version_manifest(manifest_json);

        assert!(parse_result.is_err());
    }

    #[test]
    fn parses_minecraft_manifest_versions_from_valid_json() {
        let manifest_json = r#"
        {
            "latest": {
                "release": "26.2",
                "snapshot": "26.3-snapshot-9"
            },
            "versions": [
                {
                    "id": "26.2",
                    "type": "release",
                    "url": "https://example.invalid/26.2.json",
                    "sha1": "46fb31ca7e74aea93545df8f1aa14b9d670097d3"
                }
            ]
        }
        "#;

        let manifest = parse_minecraft_version_manifest(manifest_json)
            .expect("valid Minecraft version manifest JSON should parse");

        assert_eq!(manifest.latest.release_id, "26.2");
        assert_eq!(manifest.latest.snapshot_id, "26.3-snapshot-9");

        assert_eq!(manifest.versions.len(), 1);

        let minecraft_version = &manifest.versions[0];

        assert_eq!(minecraft_version.id, "26.2");
        assert_eq!(minecraft_version.version_type, "release");
        assert_eq!(
            minecraft_version.metadata_url,
            "https://example.invalid/26.2.json"
        );
        assert_eq!(
            minecraft_version.metadata_sha1,
            "46fb31ca7e74aea93545df8f1aa14b9d670097d3"
        );
    }

    #[test]
    fn rejects_minecraft_version_manifest_without_versions() {
        let manifest_json = r#"
        {
            "latest": {
                "release": "26.2",
                "snapshot": "26.3-snapshot-9"
            }
        }
        "#;

        let manifest_result = parse_minecraft_version_manifest(manifest_json);

        assert!(manifest_result.is_err());
    }

    #[test]
    fn rejects_minecraft_version_without_metadata_sha1() {
        let manifest_json = r#"
        {
            "latest": {
                "release": "26.2",
                "snapshot": "26.3-snapshot-9"
            },
            "versions": [
                {
                    "id": "26.2",
                    "type": "release",
                    "url": "https://example.invalid/26.2.json"
                }
            ]
        }
        "#;

        let manifest_result = parse_minecraft_version_manifest(manifest_json);

        let parse_error = manifest_result
            .err()
            .expect("manifest version without metadata sha1 should fail to parse");

        assert!(
            parse_error.to_string().contains("missing field `sha1`"),
            "expected missing sha1 error, got: {parse_error}"
        );
    }

    #[test]
    fn finds_minecraft_version_summary_by_existing_id() {
        let manifest_result = parse_minecraft_version_manifest(MANIFEST_WITH_TWO_VERSIONS_JSON);

        let manifest = manifest_result.expect("valid test manifest JSON should parse");

        let minecraft_version = manifest
            .find_minecraft_version_summary_by_id("26.3-snapshot-1")
            .expect("Minecraft version 26.3-snapshot-1 should exist in the test manifest");

        assert_eq!(minecraft_version.id, "26.3-snapshot-1");
    }

    #[test]
    fn returns_none_for_unknown_minecraft_version_id() {
        let manifest_result = parse_minecraft_version_manifest(MANIFEST_WITH_TWO_VERSIONS_JSON);

        let manifest = manifest_result.expect("valid test manifest JSON should parse");

        let search_result = manifest.find_minecraft_version_summary_by_id("unknown-version");

        assert!(search_result.is_none());
    }
}

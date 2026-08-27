use sha1::{Digest, Sha1};
use std::fmt::Write;

/// Calculates the lowercase hexadecimal SHA-1 digest of Minecraft version metadata bytes.
///
/// # Example
///
/// ```
/// use aincrad_launcher::minecraft_version_metadata_sha1::calculate_minecraft_version_metadata_sha1;
///
/// let metadata_bytes: &[u8] = br#"{"id":"26.2","type":"release"}"#;
/// let metadata_sha1 = calculate_minecraft_version_metadata_sha1(metadata_bytes);
///
/// assert_eq!(metadata_sha1, "faa956574db97931bc0612fb62191e32acc8db85");
/// ```
pub fn calculate_minecraft_version_metadata_sha1(metadata_bytes: &[u8]) -> String {
    let metadata_digest = Sha1::digest(metadata_bytes);
    let mut metadata_sha1 = String::with_capacity(40);

    for digest_byte in metadata_digest {
        write!(&mut metadata_sha1, "{digest_byte:02x}")
            .expect("formatting a SHA-1 byte as lowercase hexadecimal should succeed");
    }

    metadata_sha1
}

/// Checks whether Minecraft version metadata bytes match an expected SHA-1 digest.
///
/// The expected digest must use the lowercase hexadecimal representation published
/// in the Minecraft version manifest.
///
/// # Example
///
/// ```
/// use aincrad_launcher::minecraft_version_metadata_sha1::matches_minecraft_version_metadata_sha1;
///
/// let metadata_bytes: &[u8] = br#"{"id":"26.2","type":"release"}"#;
/// let expected_metadata_sha1 = "faa956574db97931bc0612fb62191e32acc8db85";
///
/// assert!(matches_minecraft_version_metadata_sha1(
///     metadata_bytes,
///     expected_metadata_sha1,
/// ));
/// ```
pub fn matches_minecraft_version_metadata_sha1(
    metadata_bytes: &[u8],
    expected_metadata_sha1: &str,
) -> bool {
    calculate_minecraft_version_metadata_sha1(metadata_bytes) == expected_metadata_sha1
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_METADATA_BYTES: &[u8] = br#"{"id":"26.2","type":"release"}"#;

    #[test]
    fn calculates_minecraft_version_metadata_sha1() {
        let metadata_sha1 = calculate_minecraft_version_metadata_sha1(SAMPLE_METADATA_BYTES);
        assert_eq!(metadata_sha1, "faa956574db97931bc0612fb62191e32acc8db85");
    }

    #[test]
    fn returns_true_when_minecraft_version_metadata_sha1_matches() {
        let sha1 = "faa956574db97931bc0612fb62191e32acc8db85";
        let result = matches_minecraft_version_metadata_sha1(SAMPLE_METADATA_BYTES, sha1);
        assert!(result);
    }

    #[test]
    fn returns_false_when_minecraft_version_metadata_sha1_does_not_match() {
        let metadata_bytes: &[u8] = br#"{"id":"26.3","type":"release"}"#;
        let sha1: &str = "faa956574db97931bc0612fb62191e32acc8db85";
        let result = matches_minecraft_version_metadata_sha1(metadata_bytes, sha1);
        assert!(!result);
    }
}

use reqwest::blocking::{Client, Response};

pub struct ReqwestMinecraftHttpFetcher {
    http_client: Client,
}

impl ReqwestMinecraftHttpFetcher {
    fn fetch_successful_response(&self, resource_url: &str) -> Result<Response, reqwest::Error> {
        self.http_client
            .get(resource_url)
            .send()?
            .error_for_status()
    }

    /// Fetches the Minecraft HTTP resource response body as text.
    ///
    /// Returns an error when the request fails, the server returns an unsuccessful
    /// HTTP status, or the response body cannot be read as text.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use aincrad_launcher::minecraft_http_fetcher::ReqwestMinecraftHttpFetcher;
    /// use reqwest::blocking::Client;
    ///
    /// let http_client = Client::new();
    /// let minecraft_http_fetcher = ReqwestMinecraftHttpFetcher::new(http_client);
    ///
    /// let manifest_json = minecraft_http_fetcher.fetch_text(
    ///     "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json",
    /// )?;
    ///
    /// assert!(!manifest_json.is_empty());
    /// # Ok::<(), reqwest::Error>(())
    /// ```
    pub fn fetch_text(&self, resource_url: &str) -> Result<String, reqwest::Error> {
        let response = self.fetch_successful_response(resource_url)?;

        let response_text = response.text()?;
        Ok(response_text)
    }

    /// Creates a Minecraft HTTP fetcher that owns the provided HTTP client.
    ///
    /// The client can be configured with timeouts, proxy settings, or default headers
    /// before being passed to the fetcher.
    ///
    /// # Example
    ///
    /// ```
    /// use aincrad_launcher::minecraft_http_fetcher::ReqwestMinecraftHttpFetcher;
    /// use reqwest::blocking::Client;
    ///
    /// let http_client = Client::new();
    /// let minecraft_http_fetcher = ReqwestMinecraftHttpFetcher::new(http_client);
    /// ```
    pub fn new(http_client: Client) -> Self {
        Self { http_client }
    }

    /// Fetches the Minecraft HTTP resource response body as bytes.
    ///
    /// Returns an error when the request fails, the server returns an unsuccessful
    /// HTTP status, or the response body cannot be read as bytes.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use aincrad_launcher::minecraft_http_fetcher::ReqwestMinecraftHttpFetcher;
    /// use reqwest::blocking::Client;
    ///
    /// let http_client = Client::new();
    /// let minecraft_http_fetcher = ReqwestMinecraftHttpFetcher::new(http_client);
    ///
    /// let resource_bytes = minecraft_http_fetcher.fetch_bytes(
    ///     "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json",
    /// )?;
    ///
    /// assert!(!resource_bytes.is_empty());
    /// # Ok::<(), reqwest::Error>(())
    /// ```
    pub fn fetch_bytes(&self, resource_url: &str) -> Result<Vec<u8>, reqwest::Error> {
        let response = self.fetch_successful_response(resource_url)?;

        let response_bytes = response.bytes()?;
        Ok(response_bytes.to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_MANIFEST_JSON: &str = r#"
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

    const SAMPLE_RESOURCE_BYTES: &[u8] = &[0x00, 0xff, 0x10, 0x80];

    #[test]
    fn fetches_minecraft_manifest_text_from_successful_http_response() {
        let mut server = mockito::Server::new();
        let manifest_mock = server
            .mock("GET", "/version_manifest_v2.json")
            .with_status(200)
            .with_body(SAMPLE_MANIFEST_JSON)
            .create();
        let http_client = Client::new();
        let minecraft_http_fetcher = ReqwestMinecraftHttpFetcher::new(http_client);
        let manifest_url = format!("{}/version_manifest_v2.json", server.url());
        let manifest_json = minecraft_http_fetcher
            .fetch_text(&manifest_url)
            .expect("fake Minecraft request should return its response body.");
        assert_eq!(manifest_json, SAMPLE_MANIFEST_JSON);
        manifest_mock.assert();
    }

    #[test]
    fn returns_error_for_unsuccessful_minecraft_manifest_response() {
        let mut server = mockito::Server::new();
        let manifest_mock = server
            .mock("GET", "/version_manifest_v2.json")
            .with_status(500)
            .create();
        let http_client = Client::new();
        let minecraft_http_fetcher = ReqwestMinecraftHttpFetcher::new(http_client);
        let manifest_url = format!("{}/version_manifest_v2.json", server.url());
        let manifest_fetch_result = minecraft_http_fetcher.fetch_text(&manifest_url);
        let fetch_error = manifest_fetch_result
            .expect_err("fake Minecraft request should return internal server error!");
        assert_eq!(
            fetch_error.status(),
            Some(reqwest::StatusCode::INTERNAL_SERVER_ERROR)
        );
        manifest_mock.assert();
    }

    #[test]
    fn fetches_minecraft_resource_bytes_from_successful_http_response() {
        let mut server = mockito::Server::new();
        let resource_mock = server
            .mock("GET", "/resource.bin")
            .with_status(200)
            .with_body(SAMPLE_RESOURCE_BYTES)
            .create();
        let http_client = Client::new();
        let minecraft_http_fetcher = ReqwestMinecraftHttpFetcher::new(http_client);
        let resource_url = format!("{}/resource.bin", server.url());
        let resource_bytes = minecraft_http_fetcher
            .fetch_bytes(&resource_url)
            .expect("fake Minecraft request should return its response bytes");
        assert_eq!(resource_bytes, SAMPLE_RESOURCE_BYTES);
        resource_mock.assert();
    }
}

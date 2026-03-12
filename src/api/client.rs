use anyhow::{Context, Result};
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT};
use serde::de::DeserializeOwned;

const TBA_BASE_URL: &str = "https://www.thebluealliance.com/api/v3";

pub struct ApiClient {
   client: Client,
   api_key: String,
}

impl ApiClient {
   pub fn new(api_key: String) -> Result<Self> {
      let mut headers = HeaderMap::new();
      headers.insert(ACCEPT, HeaderValue::from_static("application/json"));

      let client = Client::builder()
         .default_headers(headers)
         .timeout(std::time::Duration::from_secs(30))
         .build()
         .context("Failed to create HTTP client")?;

      Ok(ApiClient { client, api_key })
   }

   pub fn get<T: DeserializeOwned>(
      &self,
      endpoint: &str,
      if_none_match: Option<&str>,
   ) -> Result<(T, Option<String>)> {
      let url = format!("{}{}", TBA_BASE_URL, endpoint);

      let mut request_builder = self.client.get(&url);

      // Only add auth header if API key is not empty
      if !self.api_key.is_empty() {
         request_builder = request_builder.header("X-TBA-Auth-Key", self.api_key.trim());
      }

      if let Some(etag) = if_none_match {
         request_builder = request_builder.header("If-None-Match", etag);
      }

      let response = request_builder.send().context("Failed to send request")?;

      // Handle 304 Not Modified
      if response.status() == 304 {
         anyhow::bail!("Not modified (304)");
      }

      // Get ETag from response
      let etag = response
         .headers()
         .get("ETag")
         .and_then(|v| v.to_str().ok())
         .map(|s| s.to_string());

      // Check for errors
      let status = response.status();
      if !status.is_success() {
         let error_text = response.text().unwrap_or_default();
         anyhow::bail!("API request failed with status {}: {}", status, error_text);
      }

      let data: T = response.json().context("Failed to parse JSON response")?;

      Ok((data, etag))
   }
}

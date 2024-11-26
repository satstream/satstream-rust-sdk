/* 
 * Satstream API
 *
 * Satstream API
 *
 * OpenAPI spec version: 1.0
 * Contact: team@satstream.io
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestsAnalyzePsbtRequest {
  /// The base64-encoded PSBT to analyze
  #[serde(rename = "psbt")]
  psbt: Option<String>
}

impl RequestsAnalyzePsbtRequest {
  pub fn new() -> RequestsAnalyzePsbtRequest {
    RequestsAnalyzePsbtRequest {
      psbt: None
    }
  }

  pub fn set_psbt(&mut self, psbt: String) {
    self.psbt = Some(psbt);
  }

  pub fn with_psbt(mut self, psbt: String) -> RequestsAnalyzePsbtRequest {
    self.psbt = Some(psbt);
    self
  }

  pub fn psbt(&self) -> Option<&String> {
    self.psbt.as_ref()
  }

  pub fn reset_psbt(&mut self) {
    self.psbt = None;
  }

}




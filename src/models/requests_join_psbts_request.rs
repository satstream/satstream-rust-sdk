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
pub struct RequestsJoinPsbtsRequest {
  /// Array of base64-encoded PSBTs to join
  #[serde(rename = "psbts")]
  psbts: Option<Vec<String>>
}

impl RequestsJoinPsbtsRequest {
  pub fn new() -> RequestsJoinPsbtsRequest {
    RequestsJoinPsbtsRequest {
      psbts: None
    }
  }

  pub fn set_psbts(&mut self, psbts: Vec<String>) {
    self.psbts = Some(psbts);
  }

  pub fn with_psbts(mut self, psbts: Vec<String>) -> RequestsJoinPsbtsRequest {
    self.psbts = Some(psbts);
    self
  }

  pub fn psbts(&self) -> Option<&Vec<String>> {
    self.psbts.as_ref()
  }

  pub fn reset_psbts(&mut self) {
    self.psbts = None;
  }

}




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
pub struct RequestsGetRawMempoolRequest {
  /// Optional: If verbose=false, returns txids with mempool sequence number
  #[serde(rename = "mempool_sequence")]
  mempool_sequence: Option<bool>,
  /// Optional: True for detailed information, false for just txids
  #[serde(rename = "verbose")]
  verbose: Option<bool>
}

impl RequestsGetRawMempoolRequest {
  pub fn new() -> RequestsGetRawMempoolRequest {
    RequestsGetRawMempoolRequest {
      mempool_sequence: None,
      verbose: None
    }
  }

  pub fn set_mempool_sequence(&mut self, mempool_sequence: bool) {
    self.mempool_sequence = Some(mempool_sequence);
  }

  pub fn with_mempool_sequence(mut self, mempool_sequence: bool) -> RequestsGetRawMempoolRequest {
    self.mempool_sequence = Some(mempool_sequence);
    self
  }

  pub fn mempool_sequence(&self) -> Option<&bool> {
    self.mempool_sequence.as_ref()
  }

  pub fn reset_mempool_sequence(&mut self) {
    self.mempool_sequence = None;
  }

  pub fn set_verbose(&mut self, verbose: bool) {
    self.verbose = Some(verbose);
  }

  pub fn with_verbose(mut self, verbose: bool) -> RequestsGetRawMempoolRequest {
    self.verbose = Some(verbose);
    self
  }

  pub fn verbose(&self) -> Option<&bool> {
    self.verbose.as_ref()
  }

  pub fn reset_verbose(&mut self) {
    self.verbose = None;
  }

}




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
pub struct RequestsGetMempoolDescendantsRequest {
  /// Required: The transaction id (must be in mempool)
  #[serde(rename = "txid")]
  txid: Option<String>,
  /// Optional: True for detailed information, false for just txids
  #[serde(rename = "verbose")]
  verbose: Option<bool>
}

impl RequestsGetMempoolDescendantsRequest {
  pub fn new() -> RequestsGetMempoolDescendantsRequest {
    RequestsGetMempoolDescendantsRequest {
      txid: None,
      verbose: None
    }
  }

  pub fn set_txid(&mut self, txid: String) {
    self.txid = Some(txid);
  }

  pub fn with_txid(mut self, txid: String) -> RequestsGetMempoolDescendantsRequest {
    self.txid = Some(txid);
    self
  }

  pub fn txid(&self) -> Option<&String> {
    self.txid.as_ref()
  }

  pub fn reset_txid(&mut self) {
    self.txid = None;
  }

  pub fn set_verbose(&mut self, verbose: bool) {
    self.verbose = Some(verbose);
  }

  pub fn with_verbose(mut self, verbose: bool) -> RequestsGetMempoolDescendantsRequest {
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




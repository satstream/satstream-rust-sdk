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
pub struct MempoolAncestorsData {
  /// Only if verbose is true
  #[serde(rename = "detailed")]
  detailed: Option<::std::collections::HashMap<String, ::models::MempoolEntry>>,
  /// Only if verbose is false
  #[serde(rename = "txids")]
  txids: Option<Vec<String>>
}

impl MempoolAncestorsData {
  pub fn new() -> MempoolAncestorsData {
    MempoolAncestorsData {
      detailed: None,
      txids: None
    }
  }

  pub fn set_detailed(&mut self, detailed: ::std::collections::HashMap<String, ::models::MempoolEntry>) {
    self.detailed = Some(detailed);
  }

  pub fn with_detailed(mut self, detailed: ::std::collections::HashMap<String, ::models::MempoolEntry>) -> MempoolAncestorsData {
    self.detailed = Some(detailed);
    self
  }

  pub fn detailed(&self) -> Option<&::std::collections::HashMap<String, ::models::MempoolEntry>> {
    self.detailed.as_ref()
  }

  pub fn reset_detailed(&mut self) {
    self.detailed = None;
  }

  pub fn set_txids(&mut self, txids: Vec<String>) {
    self.txids = Some(txids);
  }

  pub fn with_txids(mut self, txids: Vec<String>) -> MempoolAncestorsData {
    self.txids = Some(txids);
    self
  }

  pub fn txids(&self) -> Option<&Vec<String>> {
    self.txids.as_ref()
  }

  pub fn reset_txids(&mut self) {
    self.txids = None;
  }

}



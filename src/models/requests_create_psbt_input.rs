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
pub struct RequestsCreatePsbtInput {
  /// The sequence number to use for the input
  #[serde(rename = "sequence")]
  sequence: Option<i32>,
  /// The transaction id of the output to spend
  #[serde(rename = "txid")]
  txid: Option<String>,
  /// The output index number (vout) of the output to spend
  #[serde(rename = "vout")]
  vout: Option<i32>
}

impl RequestsCreatePsbtInput {
  pub fn new() -> RequestsCreatePsbtInput {
    RequestsCreatePsbtInput {
      sequence: None,
      txid: None,
      vout: None
    }
  }

  pub fn set_sequence(&mut self, sequence: i32) {
    self.sequence = Some(sequence);
  }

  pub fn with_sequence(mut self, sequence: i32) -> RequestsCreatePsbtInput {
    self.sequence = Some(sequence);
    self
  }

  pub fn sequence(&self) -> Option<&i32> {
    self.sequence.as_ref()
  }

  pub fn reset_sequence(&mut self) {
    self.sequence = None;
  }

  pub fn set_txid(&mut self, txid: String) {
    self.txid = Some(txid);
  }

  pub fn with_txid(mut self, txid: String) -> RequestsCreatePsbtInput {
    self.txid = Some(txid);
    self
  }

  pub fn txid(&self) -> Option<&String> {
    self.txid.as_ref()
  }

  pub fn reset_txid(&mut self) {
    self.txid = None;
  }

  pub fn set_vout(&mut self, vout: i32) {
    self.vout = Some(vout);
  }

  pub fn with_vout(mut self, vout: i32) -> RequestsCreatePsbtInput {
    self.vout = Some(vout);
    self
  }

  pub fn vout(&self) -> Option<&i32> {
    self.vout.as_ref()
  }

  pub fn reset_vout(&mut self) {
    self.vout = None;
  }

}



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
pub struct BlockVin3 {
  #[serde(rename = "coinbase")]
  coinbase: Option<String>,
  #[serde(rename = "prevout")]
  prevout: Option<::models::BlockVin3Prevout>,
  #[serde(rename = "scriptSig")]
  script_sig: Option<::models::ScriptSig>,
  #[serde(rename = "sequence")]
  sequence: Option<i32>,
  #[serde(rename = "txid")]
  txid: Option<String>,
  #[serde(rename = "txinwitness")]
  txinwitness: Option<Vec<String>>,
  #[serde(rename = "vout")]
  vout: Option<i32>
}

impl BlockVin3 {
  pub fn new() -> BlockVin3 {
    BlockVin3 {
      coinbase: None,
      prevout: None,
      script_sig: None,
      sequence: None,
      txid: None,
      txinwitness: None,
      vout: None
    }
  }

  pub fn set_coinbase(&mut self, coinbase: String) {
    self.coinbase = Some(coinbase);
  }

  pub fn with_coinbase(mut self, coinbase: String) -> BlockVin3 {
    self.coinbase = Some(coinbase);
    self
  }

  pub fn coinbase(&self) -> Option<&String> {
    self.coinbase.as_ref()
  }

  pub fn reset_coinbase(&mut self) {
    self.coinbase = None;
  }

  pub fn set_prevout(&mut self, prevout: ::models::BlockVin3Prevout) {
    self.prevout = Some(prevout);
  }

  pub fn with_prevout(mut self, prevout: ::models::BlockVin3Prevout) -> BlockVin3 {
    self.prevout = Some(prevout);
    self
  }

  pub fn prevout(&self) -> Option<&::models::BlockVin3Prevout> {
    self.prevout.as_ref()
  }

  pub fn reset_prevout(&mut self) {
    self.prevout = None;
  }

  pub fn set_script_sig(&mut self, script_sig: ::models::ScriptSig) {
    self.script_sig = Some(script_sig);
  }

  pub fn with_script_sig(mut self, script_sig: ::models::ScriptSig) -> BlockVin3 {
    self.script_sig = Some(script_sig);
    self
  }

  pub fn script_sig(&self) -> Option<&::models::ScriptSig> {
    self.script_sig.as_ref()
  }

  pub fn reset_script_sig(&mut self) {
    self.script_sig = None;
  }

  pub fn set_sequence(&mut self, sequence: i32) {
    self.sequence = Some(sequence);
  }

  pub fn with_sequence(mut self, sequence: i32) -> BlockVin3 {
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

  pub fn with_txid(mut self, txid: String) -> BlockVin3 {
    self.txid = Some(txid);
    self
  }

  pub fn txid(&self) -> Option<&String> {
    self.txid.as_ref()
  }

  pub fn reset_txid(&mut self) {
    self.txid = None;
  }

  pub fn set_txinwitness(&mut self, txinwitness: Vec<String>) {
    self.txinwitness = Some(txinwitness);
  }

  pub fn with_txinwitness(mut self, txinwitness: Vec<String>) -> BlockVin3 {
    self.txinwitness = Some(txinwitness);
    self
  }

  pub fn txinwitness(&self) -> Option<&Vec<String>> {
    self.txinwitness.as_ref()
  }

  pub fn reset_txinwitness(&mut self) {
    self.txinwitness = None;
  }

  pub fn set_vout(&mut self, vout: i32) {
    self.vout = Some(vout);
  }

  pub fn with_vout(mut self, vout: i32) -> BlockVin3 {
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




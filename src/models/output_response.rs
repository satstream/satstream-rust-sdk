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
pub struct OutputResponse {
  #[serde(rename = "address")]
  address: Option<String>,
  #[serde(rename = "indexed")]
  indexed: Option<bool>,
  #[serde(rename = "inscriptions")]
  inscriptions: Option<Vec<String>>,
  #[serde(rename = "outpoint")]
  outpoint: Option<String>,
  #[serde(rename = "runes")]
  runes: Option<::std::collections::HashMap<String, String>>,
  #[serde(rename = "sat_ranges")]
  sat_ranges: Option<Vec<Vec<i32>>>,
  #[serde(rename = "script_pubkey")]
  script_pubkey: Option<String>,
  #[serde(rename = "spent")]
  spent: Option<bool>,
  #[serde(rename = "transaction")]
  transaction: Option<String>,
  #[serde(rename = "value")]
  value: Option<i32>
}

impl OutputResponse {
  pub fn new() -> OutputResponse {
    OutputResponse {
      address: None,
      indexed: None,
      inscriptions: None,
      outpoint: None,
      runes: None,
      sat_ranges: None,
      script_pubkey: None,
      spent: None,
      transaction: None,
      value: None
    }
  }

  pub fn set_address(&mut self, address: String) {
    self.address = Some(address);
  }

  pub fn with_address(mut self, address: String) -> OutputResponse {
    self.address = Some(address);
    self
  }

  pub fn address(&self) -> Option<&String> {
    self.address.as_ref()
  }

  pub fn reset_address(&mut self) {
    self.address = None;
  }

  pub fn set_indexed(&mut self, indexed: bool) {
    self.indexed = Some(indexed);
  }

  pub fn with_indexed(mut self, indexed: bool) -> OutputResponse {
    self.indexed = Some(indexed);
    self
  }

  pub fn indexed(&self) -> Option<&bool> {
    self.indexed.as_ref()
  }

  pub fn reset_indexed(&mut self) {
    self.indexed = None;
  }

  pub fn set_inscriptions(&mut self, inscriptions: Vec<String>) {
    self.inscriptions = Some(inscriptions);
  }

  pub fn with_inscriptions(mut self, inscriptions: Vec<String>) -> OutputResponse {
    self.inscriptions = Some(inscriptions);
    self
  }

  pub fn inscriptions(&self) -> Option<&Vec<String>> {
    self.inscriptions.as_ref()
  }

  pub fn reset_inscriptions(&mut self) {
    self.inscriptions = None;
  }

  pub fn set_outpoint(&mut self, outpoint: String) {
    self.outpoint = Some(outpoint);
  }

  pub fn with_outpoint(mut self, outpoint: String) -> OutputResponse {
    self.outpoint = Some(outpoint);
    self
  }

  pub fn outpoint(&self) -> Option<&String> {
    self.outpoint.as_ref()
  }

  pub fn reset_outpoint(&mut self) {
    self.outpoint = None;
  }

  pub fn set_runes(&mut self, runes: ::std::collections::HashMap<String, String>) {
    self.runes = Some(runes);
  }

  pub fn with_runes(mut self, runes: ::std::collections::HashMap<String, String>) -> OutputResponse {
    self.runes = Some(runes);
    self
  }

  pub fn runes(&self) -> Option<&::std::collections::HashMap<String, String>> {
    self.runes.as_ref()
  }

  pub fn reset_runes(&mut self) {
    self.runes = None;
  }

  pub fn set_sat_ranges(&mut self, sat_ranges: Vec<Vec<i32>>) {
    self.sat_ranges = Some(sat_ranges);
  }

  pub fn with_sat_ranges(mut self, sat_ranges: Vec<Vec<i32>>) -> OutputResponse {
    self.sat_ranges = Some(sat_ranges);
    self
  }

  pub fn sat_ranges(&self) -> Option<&Vec<Vec<i32>>> {
    self.sat_ranges.as_ref()
  }

  pub fn reset_sat_ranges(&mut self) {
    self.sat_ranges = None;
  }

  pub fn set_script_pubkey(&mut self, script_pubkey: String) {
    self.script_pubkey = Some(script_pubkey);
  }

  pub fn with_script_pubkey(mut self, script_pubkey: String) -> OutputResponse {
    self.script_pubkey = Some(script_pubkey);
    self
  }

  pub fn script_pubkey(&self) -> Option<&String> {
    self.script_pubkey.as_ref()
  }

  pub fn reset_script_pubkey(&mut self) {
    self.script_pubkey = None;
  }

  pub fn set_spent(&mut self, spent: bool) {
    self.spent = Some(spent);
  }

  pub fn with_spent(mut self, spent: bool) -> OutputResponse {
    self.spent = Some(spent);
    self
  }

  pub fn spent(&self) -> Option<&bool> {
    self.spent.as_ref()
  }

  pub fn reset_spent(&mut self) {
    self.spent = None;
  }

  pub fn set_transaction(&mut self, transaction: String) {
    self.transaction = Some(transaction);
  }

  pub fn with_transaction(mut self, transaction: String) -> OutputResponse {
    self.transaction = Some(transaction);
    self
  }

  pub fn transaction(&self) -> Option<&String> {
    self.transaction.as_ref()
  }

  pub fn reset_transaction(&mut self) {
    self.transaction = None;
  }

  pub fn set_value(&mut self, value: i32) {
    self.value = Some(value);
  }

  pub fn with_value(mut self, value: i32) -> OutputResponse {
    self.value = Some(value);
    self
  }

  pub fn value(&self) -> Option<&i32> {
    self.value.as_ref()
  }

  pub fn reset_value(&mut self) {
    self.value = None;
  }

}




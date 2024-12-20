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
pub struct PsbtWitnessUtxo {
  /// The value in BTC
  #[serde(rename = "amount")]
  amount: Option<f32>,
  #[serde(rename = "scriptPubKey")]
  script_pub_key: Option<::models::PsbtWitnessUtxoScriptPubKey>
}

impl PsbtWitnessUtxo {
  pub fn new() -> PsbtWitnessUtxo {
    PsbtWitnessUtxo {
      amount: None,
      script_pub_key: None
    }
  }

  pub fn set_amount(&mut self, amount: f32) {
    self.amount = Some(amount);
  }

  pub fn with_amount(mut self, amount: f32) -> PsbtWitnessUtxo {
    self.amount = Some(amount);
    self
  }

  pub fn amount(&self) -> Option<&f32> {
    self.amount.as_ref()
  }

  pub fn reset_amount(&mut self) {
    self.amount = None;
  }

  pub fn set_script_pub_key(&mut self, script_pub_key: ::models::PsbtWitnessUtxoScriptPubKey) {
    self.script_pub_key = Some(script_pub_key);
  }

  pub fn with_script_pub_key(mut self, script_pub_key: ::models::PsbtWitnessUtxoScriptPubKey) -> PsbtWitnessUtxo {
    self.script_pub_key = Some(script_pub_key);
    self
  }

  pub fn script_pub_key(&self) -> Option<&::models::PsbtWitnessUtxoScriptPubKey> {
    self.script_pub_key.as_ref()
  }

  pub fn reset_script_pub_key(&mut self) {
    self.script_pub_key = None;
  }

}




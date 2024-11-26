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
pub struct GithubComSatstreamSsUtilsBitcoinCliUtxoBlockInfo {
  /// Coinbase subsidy amount of this block
  #[serde(rename = "coinbase")]
  coinbase: Option<f32>,
  /// Total amount of new outputs created by this block
  #[serde(rename = "new_outputs_ex_coinbase")]
  new_outputs_ex_coinbase: Option<f32>,
  /// Total amount of all prevouts spent in this block
  #[serde(rename = "prevout_spent")]
  prevout_spent: Option<f32>,
  /// Total amount of unspendable outputs created
  #[serde(rename = "unspendable")]
  unspendable: Option<f32>,
  #[serde(rename = "unspendables")]
  unspendables: Option<::models::GithubComSatstreamSsutilsBitcoincliUtxoBlockInfoUnspendables>
}

impl GithubComSatstreamSsUtilsBitcoinCliUtxoBlockInfo {
  pub fn new() -> GithubComSatstreamSsUtilsBitcoinCliUtxoBlockInfo {
    GithubComSatstreamSsUtilsBitcoinCliUtxoBlockInfo {
      coinbase: None,
      new_outputs_ex_coinbase: None,
      prevout_spent: None,
      unspendable: None,
      unspendables: None
    }
  }

  pub fn set_coinbase(&mut self, coinbase: f32) {
    self.coinbase = Some(coinbase);
  }

  pub fn with_coinbase(mut self, coinbase: f32) -> GithubComSatstreamSsUtilsBitcoinCliUtxoBlockInfo {
    self.coinbase = Some(coinbase);
    self
  }

  pub fn coinbase(&self) -> Option<&f32> {
    self.coinbase.as_ref()
  }

  pub fn reset_coinbase(&mut self) {
    self.coinbase = None;
  }

  pub fn set_new_outputs_ex_coinbase(&mut self, new_outputs_ex_coinbase: f32) {
    self.new_outputs_ex_coinbase = Some(new_outputs_ex_coinbase);
  }

  pub fn with_new_outputs_ex_coinbase(mut self, new_outputs_ex_coinbase: f32) -> GithubComSatstreamSsUtilsBitcoinCliUtxoBlockInfo {
    self.new_outputs_ex_coinbase = Some(new_outputs_ex_coinbase);
    self
  }

  pub fn new_outputs_ex_coinbase(&self) -> Option<&f32> {
    self.new_outputs_ex_coinbase.as_ref()
  }

  pub fn reset_new_outputs_ex_coinbase(&mut self) {
    self.new_outputs_ex_coinbase = None;
  }

  pub fn set_prevout_spent(&mut self, prevout_spent: f32) {
    self.prevout_spent = Some(prevout_spent);
  }

  pub fn with_prevout_spent(mut self, prevout_spent: f32) -> GithubComSatstreamSsUtilsBitcoinCliUtxoBlockInfo {
    self.prevout_spent = Some(prevout_spent);
    self
  }

  pub fn prevout_spent(&self) -> Option<&f32> {
    self.prevout_spent.as_ref()
  }

  pub fn reset_prevout_spent(&mut self) {
    self.prevout_spent = None;
  }

  pub fn set_unspendable(&mut self, unspendable: f32) {
    self.unspendable = Some(unspendable);
  }

  pub fn with_unspendable(mut self, unspendable: f32) -> GithubComSatstreamSsUtilsBitcoinCliUtxoBlockInfo {
    self.unspendable = Some(unspendable);
    self
  }

  pub fn unspendable(&self) -> Option<&f32> {
    self.unspendable.as_ref()
  }

  pub fn reset_unspendable(&mut self) {
    self.unspendable = None;
  }

  pub fn set_unspendables(&mut self, unspendables: ::models::GithubComSatstreamSsutilsBitcoincliUtxoBlockInfoUnspendables) {
    self.unspendables = Some(unspendables);
  }

  pub fn with_unspendables(mut self, unspendables: ::models::GithubComSatstreamSsutilsBitcoincliUtxoBlockInfoUnspendables) -> GithubComSatstreamSsUtilsBitcoinCliUtxoBlockInfo {
    self.unspendables = Some(unspendables);
    self
  }

  pub fn unspendables(&self) -> Option<&::models::GithubComSatstreamSsutilsBitcoincliUtxoBlockInfoUnspendables> {
    self.unspendables.as_ref()
  }

  pub fn reset_unspendables(&mut self) {
    self.unspendables = None;
  }

}




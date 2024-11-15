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
pub struct GithubComSatstreamSsUtilsRpcUtxoRune {
  #[serde(rename = "amount")]
  amount: Option<::models::BigInt>,
  #[serde(rename = "rune_id")]
  rune_id: Option<String>
}

impl GithubComSatstreamSsUtilsRpcUtxoRune {
  pub fn new() -> GithubComSatstreamSsUtilsRpcUtxoRune {
    GithubComSatstreamSsUtilsRpcUtxoRune {
      amount: None,
      rune_id: None
    }
  }

  pub fn set_amount(&mut self, amount: ::models::BigInt) {
    self.amount = Some(amount);
  }

  pub fn with_amount(mut self, amount: ::models::BigInt) -> GithubComSatstreamSsUtilsRpcUtxoRune {
    self.amount = Some(amount);
    self
  }

  pub fn amount(&self) -> Option<&::models::BigInt> {
    self.amount.as_ref()
  }

  pub fn reset_amount(&mut self) {
    self.amount = None;
  }

  pub fn set_rune_id(&mut self, rune_id: String) {
    self.rune_id = Some(rune_id);
  }

  pub fn with_rune_id(mut self, rune_id: String) -> GithubComSatstreamSsUtilsRpcUtxoRune {
    self.rune_id = Some(rune_id);
    self
  }

  pub fn rune_id(&self) -> Option<&String> {
    self.rune_id.as_ref()
  }

  pub fn reset_rune_id(&mut self) {
    self.rune_id = None;
  }

}




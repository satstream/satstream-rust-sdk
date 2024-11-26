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
pub struct GithubComSatstreamSsUtilsBitcoinCliScriptPubKey {
  #[serde(rename = "address")]
  address: Option<String>,
  #[serde(rename = "asm")]
  asm: Option<String>,
  #[serde(rename = "desc")]
  desc: Option<String>,
  #[serde(rename = "hex")]
  hex: Option<String>,
  #[serde(rename = "type")]
  _type: Option<String>
}

impl GithubComSatstreamSsUtilsBitcoinCliScriptPubKey {
  pub fn new() -> GithubComSatstreamSsUtilsBitcoinCliScriptPubKey {
    GithubComSatstreamSsUtilsBitcoinCliScriptPubKey {
      address: None,
      asm: None,
      desc: None,
      hex: None,
      _type: None
    }
  }

  pub fn set_address(&mut self, address: String) {
    self.address = Some(address);
  }

  pub fn with_address(mut self, address: String) -> GithubComSatstreamSsUtilsBitcoinCliScriptPubKey {
    self.address = Some(address);
    self
  }

  pub fn address(&self) -> Option<&String> {
    self.address.as_ref()
  }

  pub fn reset_address(&mut self) {
    self.address = None;
  }

  pub fn set_asm(&mut self, asm: String) {
    self.asm = Some(asm);
  }

  pub fn with_asm(mut self, asm: String) -> GithubComSatstreamSsUtilsBitcoinCliScriptPubKey {
    self.asm = Some(asm);
    self
  }

  pub fn asm(&self) -> Option<&String> {
    self.asm.as_ref()
  }

  pub fn reset_asm(&mut self) {
    self.asm = None;
  }

  pub fn set_desc(&mut self, desc: String) {
    self.desc = Some(desc);
  }

  pub fn with_desc(mut self, desc: String) -> GithubComSatstreamSsUtilsBitcoinCliScriptPubKey {
    self.desc = Some(desc);
    self
  }

  pub fn desc(&self) -> Option<&String> {
    self.desc.as_ref()
  }

  pub fn reset_desc(&mut self) {
    self.desc = None;
  }

  pub fn set_hex(&mut self, hex: String) {
    self.hex = Some(hex);
  }

  pub fn with_hex(mut self, hex: String) -> GithubComSatstreamSsUtilsBitcoinCliScriptPubKey {
    self.hex = Some(hex);
    self
  }

  pub fn hex(&self) -> Option<&String> {
    self.hex.as_ref()
  }

  pub fn reset_hex(&mut self) {
    self.hex = None;
  }

  pub fn set__type(&mut self, _type: String) {
    self._type = Some(_type);
  }

  pub fn with__type(mut self, _type: String) -> GithubComSatstreamSsUtilsBitcoinCliScriptPubKey {
    self._type = Some(_type);
    self
  }

  pub fn _type(&self) -> Option<&String> {
    self._type.as_ref()
  }

  pub fn reset__type(&mut self) {
    self._type = None;
  }

}



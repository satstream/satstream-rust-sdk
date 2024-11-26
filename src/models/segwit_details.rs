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
pub struct SegwitDetails {
  /// The Bitcoin address (only if well-defined)
  #[serde(rename = "address")]
  address: Option<String>,
  /// String representation of the script public key
  #[serde(rename = "asm")]
  asm: Option<String>,
  /// Inferred descriptor for the script
  #[serde(rename = "desc")]
  desc: Option<String>,
  /// Hex string of the script public key
  #[serde(rename = "hex")]
  hex: Option<String>,
  /// P2SH address wrapping this witness script
  #[serde(rename = "p2sh-segwit")]
  p2sh_segwit: Option<String>,
  /// The type of the script public key
  #[serde(rename = "type")]
  _type: Option<String>
}

impl SegwitDetails {
  pub fn new() -> SegwitDetails {
    SegwitDetails {
      address: None,
      asm: None,
      desc: None,
      hex: None,
      p2sh_segwit: None,
      _type: None
    }
  }

  pub fn set_address(&mut self, address: String) {
    self.address = Some(address);
  }

  pub fn with_address(mut self, address: String) -> SegwitDetails {
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

  pub fn with_asm(mut self, asm: String) -> SegwitDetails {
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

  pub fn with_desc(mut self, desc: String) -> SegwitDetails {
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

  pub fn with_hex(mut self, hex: String) -> SegwitDetails {
    self.hex = Some(hex);
    self
  }

  pub fn hex(&self) -> Option<&String> {
    self.hex.as_ref()
  }

  pub fn reset_hex(&mut self) {
    self.hex = None;
  }

  pub fn set_p2sh_segwit(&mut self, p2sh_segwit: String) {
    self.p2sh_segwit = Some(p2sh_segwit);
  }

  pub fn with_p2sh_segwit(mut self, p2sh_segwit: String) -> SegwitDetails {
    self.p2sh_segwit = Some(p2sh_segwit);
    self
  }

  pub fn p2sh_segwit(&self) -> Option<&String> {
    self.p2sh_segwit.as_ref()
  }

  pub fn reset_p2sh_segwit(&mut self) {
    self.p2sh_segwit = None;
  }

  pub fn set__type(&mut self, _type: String) {
    self._type = Some(_type);
  }

  pub fn with__type(mut self, _type: String) -> SegwitDetails {
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



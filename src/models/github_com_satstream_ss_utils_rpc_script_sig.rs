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
pub struct GithubComSatstreamSsUtilsRpcScriptSig {
  #[serde(rename = "asm")]
  asm: Option<String>,
  #[serde(rename = "hex")]
  hex: Option<String>
}

impl GithubComSatstreamSsUtilsRpcScriptSig {
  pub fn new() -> GithubComSatstreamSsUtilsRpcScriptSig {
    GithubComSatstreamSsUtilsRpcScriptSig {
      asm: None,
      hex: None
    }
  }

  pub fn set_asm(&mut self, asm: String) {
    self.asm = Some(asm);
  }

  pub fn with_asm(mut self, asm: String) -> GithubComSatstreamSsUtilsRpcScriptSig {
    self.asm = Some(asm);
    self
  }

  pub fn asm(&self) -> Option<&String> {
    self.asm.as_ref()
  }

  pub fn reset_asm(&mut self) {
    self.asm = None;
  }

  pub fn set_hex(&mut self, hex: String) {
    self.hex = Some(hex);
  }

  pub fn with_hex(mut self, hex: String) -> GithubComSatstreamSsUtilsRpcScriptSig {
    self.hex = Some(hex);
    self
  }

  pub fn hex(&self) -> Option<&String> {
    self.hex.as_ref()
  }

  pub fn reset_hex(&mut self) {
    self.hex = None;
  }

}




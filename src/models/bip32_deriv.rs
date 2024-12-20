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
pub struct Bip32Deriv {
  /// The fingerprint of the master key
  #[serde(rename = "master_fingerprint")]
  master_fingerprint: Option<String>,
  /// The derivation path
  #[serde(rename = "path")]
  path: Option<String>
}

impl Bip32Deriv {
  pub fn new() -> Bip32Deriv {
    Bip32Deriv {
      master_fingerprint: None,
      path: None
    }
  }

  pub fn set_master_fingerprint(&mut self, master_fingerprint: String) {
    self.master_fingerprint = Some(master_fingerprint);
  }

  pub fn with_master_fingerprint(mut self, master_fingerprint: String) -> Bip32Deriv {
    self.master_fingerprint = Some(master_fingerprint);
    self
  }

  pub fn master_fingerprint(&self) -> Option<&String> {
    self.master_fingerprint.as_ref()
  }

  pub fn reset_master_fingerprint(&mut self) {
    self.master_fingerprint = None;
  }

  pub fn set_path(&mut self, path: String) {
    self.path = Some(path);
  }

  pub fn with_path(mut self, path: String) -> Bip32Deriv {
    self.path = Some(path);
    self
  }

  pub fn path(&self) -> Option<&String> {
    self.path.as_ref()
  }

  pub fn reset_path(&mut self) {
    self.path = None;
  }

}




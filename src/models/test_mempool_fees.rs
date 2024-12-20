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
pub struct TestMempoolFees {
  /// Transaction fee in BTC
  #[serde(rename = "base")]
  base: Option<f32>
}

impl TestMempoolFees {
  pub fn new() -> TestMempoolFees {
    TestMempoolFees {
      base: None
    }
  }

  pub fn set_base(&mut self, base: f32) {
    self.base = Some(base);
  }

  pub fn with_base(mut self, base: f32) -> TestMempoolFees {
    self.base = Some(base);
    self
  }

  pub fn base(&self) -> Option<&f32> {
    self.base.as_ref()
  }

  pub fn reset_base(&mut self) {
    self.base = None;
  }

}




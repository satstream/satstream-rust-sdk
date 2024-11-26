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
pub struct RequestsCreateRawTxOutput {
  /// The bitcoin address and amount pairs Key is the bitcoin address, value is the amount in BTC
  #[serde(rename = "address")]
  address: Option<f32>,
  /// Hex-encoded data output If present, this must be the only field in the output
  #[serde(rename = "data")]
  data: Option<String>
}

impl RequestsCreateRawTxOutput {
  pub fn new() -> RequestsCreateRawTxOutput {
    RequestsCreateRawTxOutput {
      address: None,
      data: None
    }
  }

  pub fn set_address(&mut self, address: f32) {
    self.address = Some(address);
  }

  pub fn with_address(mut self, address: f32) -> RequestsCreateRawTxOutput {
    self.address = Some(address);
    self
  }

  pub fn address(&self) -> Option<&f32> {
    self.address.as_ref()
  }

  pub fn reset_address(&mut self) {
    self.address = None;
  }

  pub fn set_data(&mut self, data: String) {
    self.data = Some(data);
  }

  pub fn with_data(mut self, data: String) -> RequestsCreateRawTxOutput {
    self.data = Some(data);
    self
  }

  pub fn data(&self) -> Option<&String> {
    self.data.as_ref()
  }

  pub fn reset_data(&mut self) {
    self.data = None;
  }

}



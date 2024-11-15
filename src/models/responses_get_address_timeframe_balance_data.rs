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
pub struct ResponsesGetAddressTimeframeBalanceData {
  #[serde(rename = "items")]
  items: Option<Vec<::models::ResponsesGetAddressTimeframeBalanceItem>>
}

impl ResponsesGetAddressTimeframeBalanceData {
  pub fn new() -> ResponsesGetAddressTimeframeBalanceData {
    ResponsesGetAddressTimeframeBalanceData {
      items: None
    }
  }

  pub fn set_items(&mut self, items: Vec<::models::ResponsesGetAddressTimeframeBalanceItem>) {
    self.items = Some(items);
  }

  pub fn with_items(mut self, items: Vec<::models::ResponsesGetAddressTimeframeBalanceItem>) -> ResponsesGetAddressTimeframeBalanceData {
    self.items = Some(items);
    self
  }

  pub fn items(&self) -> Option<&Vec<::models::ResponsesGetAddressTimeframeBalanceItem>> {
    self.items.as_ref()
  }

  pub fn reset_items(&mut self) {
    self.items = None;
  }

}




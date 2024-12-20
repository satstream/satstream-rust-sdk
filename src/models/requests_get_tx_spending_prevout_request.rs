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
pub struct RequestsGetTxSpendingPrevoutRequest {
  /// The transaction outputs to check
  #[serde(rename = "outputs")]
  outputs: Option<Vec<::models::TxSpendingPrevoutInput>>
}

impl RequestsGetTxSpendingPrevoutRequest {
  pub fn new() -> RequestsGetTxSpendingPrevoutRequest {
    RequestsGetTxSpendingPrevoutRequest {
      outputs: None
    }
  }

  pub fn set_outputs(&mut self, outputs: Vec<::models::TxSpendingPrevoutInput>) {
    self.outputs = Some(outputs);
  }

  pub fn with_outputs(mut self, outputs: Vec<::models::TxSpendingPrevoutInput>) -> RequestsGetTxSpendingPrevoutRequest {
    self.outputs = Some(outputs);
    self
  }

  pub fn outputs(&self) -> Option<&Vec<::models::TxSpendingPrevoutInput>> {
    self.outputs.as_ref()
  }

  pub fn reset_outputs(&mut self) {
    self.outputs = None;
  }

}




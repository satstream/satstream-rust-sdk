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
pub struct ResponsesEstimateRawFeeResponse {
  #[serde(rename = "code")]
  code: Option<i32>,
  #[serde(rename = "data")]
  data: Option<::models::ResponsesEstimateRawFeeResponseData>,
  #[serde(rename = "msg")]
  msg: Option<String>
}

impl ResponsesEstimateRawFeeResponse {
  pub fn new() -> ResponsesEstimateRawFeeResponse {
    ResponsesEstimateRawFeeResponse {
      code: None,
      data: None,
      msg: None
    }
  }

  pub fn set_code(&mut self, code: i32) {
    self.code = Some(code);
  }

  pub fn with_code(mut self, code: i32) -> ResponsesEstimateRawFeeResponse {
    self.code = Some(code);
    self
  }

  pub fn code(&self) -> Option<&i32> {
    self.code.as_ref()
  }

  pub fn reset_code(&mut self) {
    self.code = None;
  }

  pub fn set_data(&mut self, data: ::models::ResponsesEstimateRawFeeResponseData) {
    self.data = Some(data);
  }

  pub fn with_data(mut self, data: ::models::ResponsesEstimateRawFeeResponseData) -> ResponsesEstimateRawFeeResponse {
    self.data = Some(data);
    self
  }

  pub fn data(&self) -> Option<&::models::ResponsesEstimateRawFeeResponseData> {
    self.data.as_ref()
  }

  pub fn reset_data(&mut self) {
    self.data = None;
  }

  pub fn set_msg(&mut self, msg: String) {
    self.msg = Some(msg);
  }

  pub fn with_msg(mut self, msg: String) -> ResponsesEstimateRawFeeResponse {
    self.msg = Some(msg);
    self
  }

  pub fn msg(&self) -> Option<&String> {
    self.msg.as_ref()
  }

  pub fn reset_msg(&mut self) {
    self.msg = None;
  }

}



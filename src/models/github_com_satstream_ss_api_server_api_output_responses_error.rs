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
pub struct GithubComSatstreamSsApiServerApiOutputResponsesError {
  #[serde(rename = "code")]
  code: Option<i32>,
  #[serde(rename = "data")]
  data: Option<Value>,
  #[serde(rename = "msg")]
  msg: Option<String>
}

impl GithubComSatstreamSsApiServerApiOutputResponsesError {
  pub fn new() -> GithubComSatstreamSsApiServerApiOutputResponsesError {
    GithubComSatstreamSsApiServerApiOutputResponsesError {
      code: None,
      data: None,
      msg: None
    }
  }

  pub fn set_code(&mut self, code: i32) {
    self.code = Some(code);
  }

  pub fn with_code(mut self, code: i32) -> GithubComSatstreamSsApiServerApiOutputResponsesError {
    self.code = Some(code);
    self
  }

  pub fn code(&self) -> Option<&i32> {
    self.code.as_ref()
  }

  pub fn reset_code(&mut self) {
    self.code = None;
  }

  pub fn set_data(&mut self, data: Value) {
    self.data = Some(data);
  }

  pub fn with_data(mut self, data: Value) -> GithubComSatstreamSsApiServerApiOutputResponsesError {
    self.data = Some(data);
    self
  }

  pub fn data(&self) -> Option<&Value> {
    self.data.as_ref()
  }

  pub fn reset_data(&mut self) {
    self.data = None;
  }

  pub fn set_msg(&mut self, msg: String) {
    self.msg = Some(msg);
  }

  pub fn with_msg(mut self, msg: String) -> GithubComSatstreamSsApiServerApiOutputResponsesError {
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




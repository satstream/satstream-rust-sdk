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
pub struct ResponsesGetMempoolTxInfo {
  #[serde(rename = "code")]
  code: Option<i32>,
  #[serde(rename = "data")]
  data: Option<::models::GithubComSatstreamSsUtilsRpcBtcTx>,
  #[serde(rename = "msg")]
  msg: Option<String>
}

impl ResponsesGetMempoolTxInfo {
  pub fn new() -> ResponsesGetMempoolTxInfo {
    ResponsesGetMempoolTxInfo {
      code: None,
      data: None,
      msg: None
    }
  }

  pub fn set_code(&mut self, code: i32) {
    self.code = Some(code);
  }

  pub fn with_code(mut self, code: i32) -> ResponsesGetMempoolTxInfo {
    self.code = Some(code);
    self
  }

  pub fn code(&self) -> Option<&i32> {
    self.code.as_ref()
  }

  pub fn reset_code(&mut self) {
    self.code = None;
  }

  pub fn set_data(&mut self, data: ::models::GithubComSatstreamSsUtilsRpcBtcTx) {
    self.data = Some(data);
  }

  pub fn with_data(mut self, data: ::models::GithubComSatstreamSsUtilsRpcBtcTx) -> ResponsesGetMempoolTxInfo {
    self.data = Some(data);
    self
  }

  pub fn data(&self) -> Option<&::models::GithubComSatstreamSsUtilsRpcBtcTx> {
    self.data.as_ref()
  }

  pub fn reset_data(&mut self) {
    self.data = None;
  }

  pub fn set_msg(&mut self, msg: String) {
    self.msg = Some(msg);
  }

  pub fn with_msg(mut self, msg: String) -> ResponsesGetMempoolTxInfo {
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




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
pub struct GithubComSatstreamSsUtilsOrdServerResponsesTransactionDetails {
  #[serde(rename = "input")]
  input: Option<Vec<::models::GithubComSatstreamSsUtilsOrdServerResponsesTransactionInput>>,
  #[serde(rename = "lock_time")]
  lock_time: Option<i32>,
  #[serde(rename = "output")]
  output: Option<Vec<::models::GithubComSatstreamSsUtilsOrdServerResponsesTransactionOutput>>,
  #[serde(rename = "version")]
  version: Option<i32>
}

impl GithubComSatstreamSsUtilsOrdServerResponsesTransactionDetails {
  pub fn new() -> GithubComSatstreamSsUtilsOrdServerResponsesTransactionDetails {
    GithubComSatstreamSsUtilsOrdServerResponsesTransactionDetails {
      input: None,
      lock_time: None,
      output: None,
      version: None
    }
  }

  pub fn set_input(&mut self, input: Vec<::models::GithubComSatstreamSsUtilsOrdServerResponsesTransactionInput>) {
    self.input = Some(input);
  }

  pub fn with_input(mut self, input: Vec<::models::GithubComSatstreamSsUtilsOrdServerResponsesTransactionInput>) -> GithubComSatstreamSsUtilsOrdServerResponsesTransactionDetails {
    self.input = Some(input);
    self
  }

  pub fn input(&self) -> Option<&Vec<::models::GithubComSatstreamSsUtilsOrdServerResponsesTransactionInput>> {
    self.input.as_ref()
  }

  pub fn reset_input(&mut self) {
    self.input = None;
  }

  pub fn set_lock_time(&mut self, lock_time: i32) {
    self.lock_time = Some(lock_time);
  }

  pub fn with_lock_time(mut self, lock_time: i32) -> GithubComSatstreamSsUtilsOrdServerResponsesTransactionDetails {
    self.lock_time = Some(lock_time);
    self
  }

  pub fn lock_time(&self) -> Option<&i32> {
    self.lock_time.as_ref()
  }

  pub fn reset_lock_time(&mut self) {
    self.lock_time = None;
  }

  pub fn set_output(&mut self, output: Vec<::models::GithubComSatstreamSsUtilsOrdServerResponsesTransactionOutput>) {
    self.output = Some(output);
  }

  pub fn with_output(mut self, output: Vec<::models::GithubComSatstreamSsUtilsOrdServerResponsesTransactionOutput>) -> GithubComSatstreamSsUtilsOrdServerResponsesTransactionDetails {
    self.output = Some(output);
    self
  }

  pub fn output(&self) -> Option<&Vec<::models::GithubComSatstreamSsUtilsOrdServerResponsesTransactionOutput>> {
    self.output.as_ref()
  }

  pub fn reset_output(&mut self) {
    self.output = None;
  }

  pub fn set_version(&mut self, version: i32) {
    self.version = Some(version);
  }

  pub fn with_version(mut self, version: i32) -> GithubComSatstreamSsUtilsOrdServerResponsesTransactionDetails {
    self.version = Some(version);
    self
  }

  pub fn version(&self) -> Option<&i32> {
    self.version.as_ref()
  }

  pub fn reset_version(&mut self) {
    self.version = None;
  }

}




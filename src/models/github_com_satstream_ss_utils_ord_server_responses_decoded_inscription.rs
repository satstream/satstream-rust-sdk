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
pub struct GithubComSatstreamSsUtilsOrdServerResponsesDecodedInscription {
  #[serde(rename = "input")]
  input: Option<i32>,
  #[serde(rename = "offset")]
  offset: Option<i32>,
  #[serde(rename = "payload")]
  payload: Option<::models::GithubComSatstreamSsUtilsOrdServerResponsesInscriptionData>,
  #[serde(rename = "pushnum")]
  pushnum: Option<bool>,
  #[serde(rename = "stutter")]
  stutter: Option<bool>
}

impl GithubComSatstreamSsUtilsOrdServerResponsesDecodedInscription {
  pub fn new() -> GithubComSatstreamSsUtilsOrdServerResponsesDecodedInscription {
    GithubComSatstreamSsUtilsOrdServerResponsesDecodedInscription {
      input: None,
      offset: None,
      payload: None,
      pushnum: None,
      stutter: None
    }
  }

  pub fn set_input(&mut self, input: i32) {
    self.input = Some(input);
  }

  pub fn with_input(mut self, input: i32) -> GithubComSatstreamSsUtilsOrdServerResponsesDecodedInscription {
    self.input = Some(input);
    self
  }

  pub fn input(&self) -> Option<&i32> {
    self.input.as_ref()
  }

  pub fn reset_input(&mut self) {
    self.input = None;
  }

  pub fn set_offset(&mut self, offset: i32) {
    self.offset = Some(offset);
  }

  pub fn with_offset(mut self, offset: i32) -> GithubComSatstreamSsUtilsOrdServerResponsesDecodedInscription {
    self.offset = Some(offset);
    self
  }

  pub fn offset(&self) -> Option<&i32> {
    self.offset.as_ref()
  }

  pub fn reset_offset(&mut self) {
    self.offset = None;
  }

  pub fn set_payload(&mut self, payload: ::models::GithubComSatstreamSsUtilsOrdServerResponsesInscriptionData) {
    self.payload = Some(payload);
  }

  pub fn with_payload(mut self, payload: ::models::GithubComSatstreamSsUtilsOrdServerResponsesInscriptionData) -> GithubComSatstreamSsUtilsOrdServerResponsesDecodedInscription {
    self.payload = Some(payload);
    self
  }

  pub fn payload(&self) -> Option<&::models::GithubComSatstreamSsUtilsOrdServerResponsesInscriptionData> {
    self.payload.as_ref()
  }

  pub fn reset_payload(&mut self) {
    self.payload = None;
  }

  pub fn set_pushnum(&mut self, pushnum: bool) {
    self.pushnum = Some(pushnum);
  }

  pub fn with_pushnum(mut self, pushnum: bool) -> GithubComSatstreamSsUtilsOrdServerResponsesDecodedInscription {
    self.pushnum = Some(pushnum);
    self
  }

  pub fn pushnum(&self) -> Option<&bool> {
    self.pushnum.as_ref()
  }

  pub fn reset_pushnum(&mut self) {
    self.pushnum = None;
  }

  pub fn set_stutter(&mut self, stutter: bool) {
    self.stutter = Some(stutter);
  }

  pub fn with_stutter(mut self, stutter: bool) -> GithubComSatstreamSsUtilsOrdServerResponsesDecodedInscription {
    self.stutter = Some(stutter);
    self
  }

  pub fn stutter(&self) -> Option<&bool> {
    self.stutter.as_ref()
  }

  pub fn reset_stutter(&mut self) {
    self.stutter = None;
  }

}




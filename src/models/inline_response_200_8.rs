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
pub struct InlineResponse2008 {
  #[serde(rename = "data")]
  data: Option<::models::BlockStats>,
  #[serde(rename = "utils.ResponseEnvelope")]
  utils_response_envelope: Option<::models::UtilsResponseEnvelope>
}

impl InlineResponse2008 {
  pub fn new() -> InlineResponse2008 {
    InlineResponse2008 {
      data: None,
      utils_response_envelope: None
    }
  }

  pub fn set_data(&mut self, data: ::models::BlockStats) {
    self.data = Some(data);
  }

  pub fn with_data(mut self, data: ::models::BlockStats) -> InlineResponse2008 {
    self.data = Some(data);
    self
  }

  pub fn data(&self) -> Option<&::models::BlockStats> {
    self.data.as_ref()
  }

  pub fn reset_data(&mut self) {
    self.data = None;
  }

  pub fn set_utils_response_envelope(&mut self, utils_response_envelope: ::models::UtilsResponseEnvelope) {
    self.utils_response_envelope = Some(utils_response_envelope);
  }

  pub fn with_utils_response_envelope(mut self, utils_response_envelope: ::models::UtilsResponseEnvelope) -> InlineResponse2008 {
    self.utils_response_envelope = Some(utils_response_envelope);
    self
  }

  pub fn utils_response_envelope(&self) -> Option<&::models::UtilsResponseEnvelope> {
    self.utils_response_envelope.as_ref()
  }

  pub fn reset_utils_response_envelope(&mut self) {
    self.utils_response_envelope = None;
  }

}



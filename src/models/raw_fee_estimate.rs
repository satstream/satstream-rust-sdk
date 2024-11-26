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
pub struct RawFeeEstimate {
  #[serde(rename = "long")]
  long: Option<::models::RawFeeEstimateLong>,
  #[serde(rename = "medium")]
  medium: Option<::models::RawFeeEstimateMedium>,
  #[serde(rename = "short")]
  short: Option<::models::RawFeeEstimateShort>
}

impl RawFeeEstimate {
  pub fn new() -> RawFeeEstimate {
    RawFeeEstimate {
      long: None,
      medium: None,
      short: None
    }
  }

  pub fn set_long(&mut self, long: ::models::RawFeeEstimateLong) {
    self.long = Some(long);
  }

  pub fn with_long(mut self, long: ::models::RawFeeEstimateLong) -> RawFeeEstimate {
    self.long = Some(long);
    self
  }

  pub fn long(&self) -> Option<&::models::RawFeeEstimateLong> {
    self.long.as_ref()
  }

  pub fn reset_long(&mut self) {
    self.long = None;
  }

  pub fn set_medium(&mut self, medium: ::models::RawFeeEstimateMedium) {
    self.medium = Some(medium);
  }

  pub fn with_medium(mut self, medium: ::models::RawFeeEstimateMedium) -> RawFeeEstimate {
    self.medium = Some(medium);
    self
  }

  pub fn medium(&self) -> Option<&::models::RawFeeEstimateMedium> {
    self.medium.as_ref()
  }

  pub fn reset_medium(&mut self) {
    self.medium = None;
  }

  pub fn set_short(&mut self, short: ::models::RawFeeEstimateShort) {
    self.short = Some(short);
  }

  pub fn with_short(mut self, short: ::models::RawFeeEstimateShort) -> RawFeeEstimate {
    self.short = Some(short);
    self
  }

  pub fn short(&self) -> Option<&::models::RawFeeEstimateShort> {
    self.short.as_ref()
  }

  pub fn reset_short(&mut self) {
    self.short = None;
  }

}



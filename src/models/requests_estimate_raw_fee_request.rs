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
pub struct RequestsEstimateRawFeeRequest {
  /// Confirmation target in blocks (1 - 1008)
  #[serde(rename = "conf_target")]
  conf_target: i32,
  /// The proportion of transactions in a given feerate range that must have been confirmed within conf_target in order to consider those feerates as high enough and proceed to check lower buckets. Optional, defaults to 0.95
  #[serde(rename = "threshold")]
  threshold: Option<f32>
}

impl RequestsEstimateRawFeeRequest {
  pub fn new(conf_target: i32) -> RequestsEstimateRawFeeRequest {
    RequestsEstimateRawFeeRequest {
      conf_target: conf_target,
      threshold: None
    }
  }

  pub fn set_conf_target(&mut self, conf_target: i32) {
    self.conf_target = conf_target;
  }

  pub fn with_conf_target(mut self, conf_target: i32) -> RequestsEstimateRawFeeRequest {
    self.conf_target = conf_target;
    self
  }

  pub fn conf_target(&self) -> &i32 {
    &self.conf_target
  }


  pub fn set_threshold(&mut self, threshold: f32) {
    self.threshold = Some(threshold);
  }

  pub fn with_threshold(mut self, threshold: f32) -> RequestsEstimateRawFeeRequest {
    self.threshold = Some(threshold);
    self
  }

  pub fn threshold(&self) -> Option<&f32> {
    self.threshold.as_ref()
  }

  pub fn reset_threshold(&mut self) {
    self.threshold = None;
  }

}




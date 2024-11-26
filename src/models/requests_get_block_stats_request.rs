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
pub struct RequestsGetBlockStatsRequest {
  /// Required: The block hash (string) or height (numeric)
  #[serde(rename = "hash_or_height")]
  hash_or_height: Option<Value>,
  /// Optional: Values to plot (if empty, all values will be included)
  #[serde(rename = "stats")]
  stats: Option<Vec<String>>
}

impl RequestsGetBlockStatsRequest {
  pub fn new() -> RequestsGetBlockStatsRequest {
    RequestsGetBlockStatsRequest {
      hash_or_height: None,
      stats: None
    }
  }

  pub fn set_hash_or_height(&mut self, hash_or_height: Value) {
    self.hash_or_height = Some(hash_or_height);
  }

  pub fn with_hash_or_height(mut self, hash_or_height: Value) -> RequestsGetBlockStatsRequest {
    self.hash_or_height = Some(hash_or_height);
    self
  }

  pub fn hash_or_height(&self) -> Option<&Value> {
    self.hash_or_height.as_ref()
  }

  pub fn reset_hash_or_height(&mut self) {
    self.hash_or_height = None;
  }

  pub fn set_stats(&mut self, stats: Vec<String>) {
    self.stats = Some(stats);
  }

  pub fn with_stats(mut self, stats: Vec<String>) -> RequestsGetBlockStatsRequest {
    self.stats = Some(stats);
    self
  }

  pub fn stats(&self) -> Option<&Vec<String>> {
    self.stats.as_ref()
  }

  pub fn reset_stats(&mut self) {
    self.stats = None;
  }

}




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
pub struct GithubComSatstreamSsUtilsOrdServerResponsesSatoshiResponse {
  #[serde(rename = "block")]
  block: Option<i32>,
  #[serde(rename = "charms")]
  charms: Option<Vec<String>>,
  #[serde(rename = "cycle")]
  cycle: Option<i32>,
  #[serde(rename = "decimal")]
  decimal: Option<String>,
  #[serde(rename = "degree")]
  degree: Option<String>,
  #[serde(rename = "epoch")]
  epoch: Option<i32>,
  #[serde(rename = "inscriptions")]
  inscriptions: Option<Vec<String>>,
  #[serde(rename = "name")]
  name: Option<String>,
  #[serde(rename = "number")]
  number: Option<i32>,
  #[serde(rename = "offset")]
  offset: Option<i32>,
  #[serde(rename = "percentile")]
  percentile: Option<String>,
  #[serde(rename = "period")]
  period: Option<i32>,
  #[serde(rename = "rarity")]
  rarity: Option<String>,
  #[serde(rename = "satpoint")]
  satpoint: Option<String>,
  #[serde(rename = "timestamp")]
  timestamp: Option<i32>
}

impl GithubComSatstreamSsUtilsOrdServerResponsesSatoshiResponse {
  pub fn new() -> GithubComSatstreamSsUtilsOrdServerResponsesSatoshiResponse {
    GithubComSatstreamSsUtilsOrdServerResponsesSatoshiResponse {
      block: None,
      charms: None,
      cycle: None,
      decimal: None,
      degree: None,
      epoch: None,
      inscriptions: None,
      name: None,
      number: None,
      offset: None,
      percentile: None,
      period: None,
      rarity: None,
      satpoint: None,
      timestamp: None
    }
  }

  pub fn set_block(&mut self, block: i32) {
    self.block = Some(block);
  }

  pub fn with_block(mut self, block: i32) -> GithubComSatstreamSsUtilsOrdServerResponsesSatoshiResponse {
    self.block = Some(block);
    self
  }

  pub fn block(&self) -> Option<&i32> {
    self.block.as_ref()
  }

  pub fn reset_block(&mut self) {
    self.block = None;
  }

  pub fn set_charms(&mut self, charms: Vec<String>) {
    self.charms = Some(charms);
  }

  pub fn with_charms(mut self, charms: Vec<String>) -> GithubComSatstreamSsUtilsOrdServerResponsesSatoshiResponse {
    self.charms = Some(charms);
    self
  }

  pub fn charms(&self) -> Option<&Vec<String>> {
    self.charms.as_ref()
  }

  pub fn reset_charms(&mut self) {
    self.charms = None;
  }

  pub fn set_cycle(&mut self, cycle: i32) {
    self.cycle = Some(cycle);
  }

  pub fn with_cycle(mut self, cycle: i32) -> GithubComSatstreamSsUtilsOrdServerResponsesSatoshiResponse {
    self.cycle = Some(cycle);
    self
  }

  pub fn cycle(&self) -> Option<&i32> {
    self.cycle.as_ref()
  }

  pub fn reset_cycle(&mut self) {
    self.cycle = None;
  }

  pub fn set_decimal(&mut self, decimal: String) {
    self.decimal = Some(decimal);
  }

  pub fn with_decimal(mut self, decimal: String) -> GithubComSatstreamSsUtilsOrdServerResponsesSatoshiResponse {
    self.decimal = Some(decimal);
    self
  }

  pub fn decimal(&self) -> Option<&String> {
    self.decimal.as_ref()
  }

  pub fn reset_decimal(&mut self) {
    self.decimal = None;
  }

  pub fn set_degree(&mut self, degree: String) {
    self.degree = Some(degree);
  }

  pub fn with_degree(mut self, degree: String) -> GithubComSatstreamSsUtilsOrdServerResponsesSatoshiResponse {
    self.degree = Some(degree);
    self
  }

  pub fn degree(&self) -> Option<&String> {
    self.degree.as_ref()
  }

  pub fn reset_degree(&mut self) {
    self.degree = None;
  }

  pub fn set_epoch(&mut self, epoch: i32) {
    self.epoch = Some(epoch);
  }

  pub fn with_epoch(mut self, epoch: i32) -> GithubComSatstreamSsUtilsOrdServerResponsesSatoshiResponse {
    self.epoch = Some(epoch);
    self
  }

  pub fn epoch(&self) -> Option<&i32> {
    self.epoch.as_ref()
  }

  pub fn reset_epoch(&mut self) {
    self.epoch = None;
  }

  pub fn set_inscriptions(&mut self, inscriptions: Vec<String>) {
    self.inscriptions = Some(inscriptions);
  }

  pub fn with_inscriptions(mut self, inscriptions: Vec<String>) -> GithubComSatstreamSsUtilsOrdServerResponsesSatoshiResponse {
    self.inscriptions = Some(inscriptions);
    self
  }

  pub fn inscriptions(&self) -> Option<&Vec<String>> {
    self.inscriptions.as_ref()
  }

  pub fn reset_inscriptions(&mut self) {
    self.inscriptions = None;
  }

  pub fn set_name(&mut self, name: String) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: String) -> GithubComSatstreamSsUtilsOrdServerResponsesSatoshiResponse {
    self.name = Some(name);
    self
  }

  pub fn name(&self) -> Option<&String> {
    self.name.as_ref()
  }

  pub fn reset_name(&mut self) {
    self.name = None;
  }

  pub fn set_number(&mut self, number: i32) {
    self.number = Some(number);
  }

  pub fn with_number(mut self, number: i32) -> GithubComSatstreamSsUtilsOrdServerResponsesSatoshiResponse {
    self.number = Some(number);
    self
  }

  pub fn number(&self) -> Option<&i32> {
    self.number.as_ref()
  }

  pub fn reset_number(&mut self) {
    self.number = None;
  }

  pub fn set_offset(&mut self, offset: i32) {
    self.offset = Some(offset);
  }

  pub fn with_offset(mut self, offset: i32) -> GithubComSatstreamSsUtilsOrdServerResponsesSatoshiResponse {
    self.offset = Some(offset);
    self
  }

  pub fn offset(&self) -> Option<&i32> {
    self.offset.as_ref()
  }

  pub fn reset_offset(&mut self) {
    self.offset = None;
  }

  pub fn set_percentile(&mut self, percentile: String) {
    self.percentile = Some(percentile);
  }

  pub fn with_percentile(mut self, percentile: String) -> GithubComSatstreamSsUtilsOrdServerResponsesSatoshiResponse {
    self.percentile = Some(percentile);
    self
  }

  pub fn percentile(&self) -> Option<&String> {
    self.percentile.as_ref()
  }

  pub fn reset_percentile(&mut self) {
    self.percentile = None;
  }

  pub fn set_period(&mut self, period: i32) {
    self.period = Some(period);
  }

  pub fn with_period(mut self, period: i32) -> GithubComSatstreamSsUtilsOrdServerResponsesSatoshiResponse {
    self.period = Some(period);
    self
  }

  pub fn period(&self) -> Option<&i32> {
    self.period.as_ref()
  }

  pub fn reset_period(&mut self) {
    self.period = None;
  }

  pub fn set_rarity(&mut self, rarity: String) {
    self.rarity = Some(rarity);
  }

  pub fn with_rarity(mut self, rarity: String) -> GithubComSatstreamSsUtilsOrdServerResponsesSatoshiResponse {
    self.rarity = Some(rarity);
    self
  }

  pub fn rarity(&self) -> Option<&String> {
    self.rarity.as_ref()
  }

  pub fn reset_rarity(&mut self) {
    self.rarity = None;
  }

  pub fn set_satpoint(&mut self, satpoint: String) {
    self.satpoint = Some(satpoint);
  }

  pub fn with_satpoint(mut self, satpoint: String) -> GithubComSatstreamSsUtilsOrdServerResponsesSatoshiResponse {
    self.satpoint = Some(satpoint);
    self
  }

  pub fn satpoint(&self) -> Option<&String> {
    self.satpoint.as_ref()
  }

  pub fn reset_satpoint(&mut self) {
    self.satpoint = None;
  }

  pub fn set_timestamp(&mut self, timestamp: i32) {
    self.timestamp = Some(timestamp);
  }

  pub fn with_timestamp(mut self, timestamp: i32) -> GithubComSatstreamSsUtilsOrdServerResponsesSatoshiResponse {
    self.timestamp = Some(timestamp);
    self
  }

  pub fn timestamp(&self) -> Option<&i32> {
    self.timestamp.as_ref()
  }

  pub fn reset_timestamp(&mut self) {
    self.timestamp = None;
  }

}



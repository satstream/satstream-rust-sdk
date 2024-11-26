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
pub struct GithubComSatstreamSsUtilsOrdServerResponsesStatusResponse {
  #[serde(rename = "address_index")]
  address_index: Option<bool>,
  #[serde(rename = "blessed_inscriptions")]
  blessed_inscriptions: Option<i32>,
  #[serde(rename = "chain")]
  chain: Option<String>,
  #[serde(rename = "cursed_inscriptions")]
  cursed_inscriptions: Option<i32>,
  #[serde(rename = "height")]
  height: Option<i32>,
  #[serde(rename = "initial_sync_time")]
  initial_sync_time: Option<::models::GithubComSatstreamSsUtilsOrdServerResponsesDuration>,
  #[serde(rename = "inscriptions")]
  inscriptions: Option<i32>,
  #[serde(rename = "lost_sats")]
  lost_sats: Option<i32>,
  #[serde(rename = "minimum_rune_for_next_block")]
  minimum_rune_for_next_block: Option<String>,
  #[serde(rename = "rune_index")]
  rune_index: Option<bool>,
  #[serde(rename = "runes")]
  runes: Option<i32>,
  #[serde(rename = "sat_index")]
  sat_index: Option<bool>,
  #[serde(rename = "started")]
  started: Option<String>,
  #[serde(rename = "transaction_index")]
  transaction_index: Option<bool>,
  #[serde(rename = "unrecoverably_reorged")]
  unrecoverably_reorged: Option<bool>,
  #[serde(rename = "uptime")]
  uptime: Option<::models::GithubComSatstreamSsUtilsOrdServerResponsesDuration>
}

impl GithubComSatstreamSsUtilsOrdServerResponsesStatusResponse {
  pub fn new() -> GithubComSatstreamSsUtilsOrdServerResponsesStatusResponse {
    GithubComSatstreamSsUtilsOrdServerResponsesStatusResponse {
      address_index: None,
      blessed_inscriptions: None,
      chain: None,
      cursed_inscriptions: None,
      height: None,
      initial_sync_time: None,
      inscriptions: None,
      lost_sats: None,
      minimum_rune_for_next_block: None,
      rune_index: None,
      runes: None,
      sat_index: None,
      started: None,
      transaction_index: None,
      unrecoverably_reorged: None,
      uptime: None
    }
  }

  pub fn set_address_index(&mut self, address_index: bool) {
    self.address_index = Some(address_index);
  }

  pub fn with_address_index(mut self, address_index: bool) -> GithubComSatstreamSsUtilsOrdServerResponsesStatusResponse {
    self.address_index = Some(address_index);
    self
  }

  pub fn address_index(&self) -> Option<&bool> {
    self.address_index.as_ref()
  }

  pub fn reset_address_index(&mut self) {
    self.address_index = None;
  }

  pub fn set_blessed_inscriptions(&mut self, blessed_inscriptions: i32) {
    self.blessed_inscriptions = Some(blessed_inscriptions);
  }

  pub fn with_blessed_inscriptions(mut self, blessed_inscriptions: i32) -> GithubComSatstreamSsUtilsOrdServerResponsesStatusResponse {
    self.blessed_inscriptions = Some(blessed_inscriptions);
    self
  }

  pub fn blessed_inscriptions(&self) -> Option<&i32> {
    self.blessed_inscriptions.as_ref()
  }

  pub fn reset_blessed_inscriptions(&mut self) {
    self.blessed_inscriptions = None;
  }

  pub fn set_chain(&mut self, chain: String) {
    self.chain = Some(chain);
  }

  pub fn with_chain(mut self, chain: String) -> GithubComSatstreamSsUtilsOrdServerResponsesStatusResponse {
    self.chain = Some(chain);
    self
  }

  pub fn chain(&self) -> Option<&String> {
    self.chain.as_ref()
  }

  pub fn reset_chain(&mut self) {
    self.chain = None;
  }

  pub fn set_cursed_inscriptions(&mut self, cursed_inscriptions: i32) {
    self.cursed_inscriptions = Some(cursed_inscriptions);
  }

  pub fn with_cursed_inscriptions(mut self, cursed_inscriptions: i32) -> GithubComSatstreamSsUtilsOrdServerResponsesStatusResponse {
    self.cursed_inscriptions = Some(cursed_inscriptions);
    self
  }

  pub fn cursed_inscriptions(&self) -> Option<&i32> {
    self.cursed_inscriptions.as_ref()
  }

  pub fn reset_cursed_inscriptions(&mut self) {
    self.cursed_inscriptions = None;
  }

  pub fn set_height(&mut self, height: i32) {
    self.height = Some(height);
  }

  pub fn with_height(mut self, height: i32) -> GithubComSatstreamSsUtilsOrdServerResponsesStatusResponse {
    self.height = Some(height);
    self
  }

  pub fn height(&self) -> Option<&i32> {
    self.height.as_ref()
  }

  pub fn reset_height(&mut self) {
    self.height = None;
  }

  pub fn set_initial_sync_time(&mut self, initial_sync_time: ::models::GithubComSatstreamSsUtilsOrdServerResponsesDuration) {
    self.initial_sync_time = Some(initial_sync_time);
  }

  pub fn with_initial_sync_time(mut self, initial_sync_time: ::models::GithubComSatstreamSsUtilsOrdServerResponsesDuration) -> GithubComSatstreamSsUtilsOrdServerResponsesStatusResponse {
    self.initial_sync_time = Some(initial_sync_time);
    self
  }

  pub fn initial_sync_time(&self) -> Option<&::models::GithubComSatstreamSsUtilsOrdServerResponsesDuration> {
    self.initial_sync_time.as_ref()
  }

  pub fn reset_initial_sync_time(&mut self) {
    self.initial_sync_time = None;
  }

  pub fn set_inscriptions(&mut self, inscriptions: i32) {
    self.inscriptions = Some(inscriptions);
  }

  pub fn with_inscriptions(mut self, inscriptions: i32) -> GithubComSatstreamSsUtilsOrdServerResponsesStatusResponse {
    self.inscriptions = Some(inscriptions);
    self
  }

  pub fn inscriptions(&self) -> Option<&i32> {
    self.inscriptions.as_ref()
  }

  pub fn reset_inscriptions(&mut self) {
    self.inscriptions = None;
  }

  pub fn set_lost_sats(&mut self, lost_sats: i32) {
    self.lost_sats = Some(lost_sats);
  }

  pub fn with_lost_sats(mut self, lost_sats: i32) -> GithubComSatstreamSsUtilsOrdServerResponsesStatusResponse {
    self.lost_sats = Some(lost_sats);
    self
  }

  pub fn lost_sats(&self) -> Option<&i32> {
    self.lost_sats.as_ref()
  }

  pub fn reset_lost_sats(&mut self) {
    self.lost_sats = None;
  }

  pub fn set_minimum_rune_for_next_block(&mut self, minimum_rune_for_next_block: String) {
    self.minimum_rune_for_next_block = Some(minimum_rune_for_next_block);
  }

  pub fn with_minimum_rune_for_next_block(mut self, minimum_rune_for_next_block: String) -> GithubComSatstreamSsUtilsOrdServerResponsesStatusResponse {
    self.minimum_rune_for_next_block = Some(minimum_rune_for_next_block);
    self
  }

  pub fn minimum_rune_for_next_block(&self) -> Option<&String> {
    self.minimum_rune_for_next_block.as_ref()
  }

  pub fn reset_minimum_rune_for_next_block(&mut self) {
    self.minimum_rune_for_next_block = None;
  }

  pub fn set_rune_index(&mut self, rune_index: bool) {
    self.rune_index = Some(rune_index);
  }

  pub fn with_rune_index(mut self, rune_index: bool) -> GithubComSatstreamSsUtilsOrdServerResponsesStatusResponse {
    self.rune_index = Some(rune_index);
    self
  }

  pub fn rune_index(&self) -> Option<&bool> {
    self.rune_index.as_ref()
  }

  pub fn reset_rune_index(&mut self) {
    self.rune_index = None;
  }

  pub fn set_runes(&mut self, runes: i32) {
    self.runes = Some(runes);
  }

  pub fn with_runes(mut self, runes: i32) -> GithubComSatstreamSsUtilsOrdServerResponsesStatusResponse {
    self.runes = Some(runes);
    self
  }

  pub fn runes(&self) -> Option<&i32> {
    self.runes.as_ref()
  }

  pub fn reset_runes(&mut self) {
    self.runes = None;
  }

  pub fn set_sat_index(&mut self, sat_index: bool) {
    self.sat_index = Some(sat_index);
  }

  pub fn with_sat_index(mut self, sat_index: bool) -> GithubComSatstreamSsUtilsOrdServerResponsesStatusResponse {
    self.sat_index = Some(sat_index);
    self
  }

  pub fn sat_index(&self) -> Option<&bool> {
    self.sat_index.as_ref()
  }

  pub fn reset_sat_index(&mut self) {
    self.sat_index = None;
  }

  pub fn set_started(&mut self, started: String) {
    self.started = Some(started);
  }

  pub fn with_started(mut self, started: String) -> GithubComSatstreamSsUtilsOrdServerResponsesStatusResponse {
    self.started = Some(started);
    self
  }

  pub fn started(&self) -> Option<&String> {
    self.started.as_ref()
  }

  pub fn reset_started(&mut self) {
    self.started = None;
  }

  pub fn set_transaction_index(&mut self, transaction_index: bool) {
    self.transaction_index = Some(transaction_index);
  }

  pub fn with_transaction_index(mut self, transaction_index: bool) -> GithubComSatstreamSsUtilsOrdServerResponsesStatusResponse {
    self.transaction_index = Some(transaction_index);
    self
  }

  pub fn transaction_index(&self) -> Option<&bool> {
    self.transaction_index.as_ref()
  }

  pub fn reset_transaction_index(&mut self) {
    self.transaction_index = None;
  }

  pub fn set_unrecoverably_reorged(&mut self, unrecoverably_reorged: bool) {
    self.unrecoverably_reorged = Some(unrecoverably_reorged);
  }

  pub fn with_unrecoverably_reorged(mut self, unrecoverably_reorged: bool) -> GithubComSatstreamSsUtilsOrdServerResponsesStatusResponse {
    self.unrecoverably_reorged = Some(unrecoverably_reorged);
    self
  }

  pub fn unrecoverably_reorged(&self) -> Option<&bool> {
    self.unrecoverably_reorged.as_ref()
  }

  pub fn reset_unrecoverably_reorged(&mut self) {
    self.unrecoverably_reorged = None;
  }

  pub fn set_uptime(&mut self, uptime: ::models::GithubComSatstreamSsUtilsOrdServerResponsesDuration) {
    self.uptime = Some(uptime);
  }

  pub fn with_uptime(mut self, uptime: ::models::GithubComSatstreamSsUtilsOrdServerResponsesDuration) -> GithubComSatstreamSsUtilsOrdServerResponsesStatusResponse {
    self.uptime = Some(uptime);
    self
  }

  pub fn uptime(&self) -> Option<&::models::GithubComSatstreamSsUtilsOrdServerResponsesDuration> {
    self.uptime.as_ref()
  }

  pub fn reset_uptime(&mut self) {
    self.uptime = None;
  }

}



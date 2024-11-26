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
pub struct GithubComSatstreamSsUtilsBitcoinCliBlock1 {
  #[serde(rename = "bits")]
  bits: Option<String>,
  #[serde(rename = "chainwork")]
  chainwork: Option<String>,
  #[serde(rename = "confirmations")]
  confirmations: Option<i32>,
  #[serde(rename = "difficulty")]
  difficulty: Option<f32>,
  #[serde(rename = "hash")]
  hash: Option<String>,
  #[serde(rename = "height")]
  height: Option<i32>,
  #[serde(rename = "mediantime")]
  mediantime: Option<i32>,
  #[serde(rename = "merkleroot")]
  merkleroot: Option<String>,
  #[serde(rename = "nTx")]
  n_tx: Option<i32>,
  #[serde(rename = "nextblockhash")]
  nextblockhash: Option<String>,
  #[serde(rename = "nonce")]
  nonce: Option<i32>,
  #[serde(rename = "previousblockhash")]
  previousblockhash: Option<String>,
  #[serde(rename = "size")]
  size: Option<i32>,
  #[serde(rename = "strippedsize")]
  strippedsize: Option<i32>,
  #[serde(rename = "time")]
  time: Option<i32>,
  #[serde(rename = "tx")]
  tx: Option<Vec<String>>,
  #[serde(rename = "version")]
  version: Option<i32>,
  #[serde(rename = "versionHex")]
  version_hex: Option<String>,
  #[serde(rename = "weight")]
  weight: Option<i32>
}

impl GithubComSatstreamSsUtilsBitcoinCliBlock1 {
  pub fn new() -> GithubComSatstreamSsUtilsBitcoinCliBlock1 {
    GithubComSatstreamSsUtilsBitcoinCliBlock1 {
      bits: None,
      chainwork: None,
      confirmations: None,
      difficulty: None,
      hash: None,
      height: None,
      mediantime: None,
      merkleroot: None,
      n_tx: None,
      nextblockhash: None,
      nonce: None,
      previousblockhash: None,
      size: None,
      strippedsize: None,
      time: None,
      tx: None,
      version: None,
      version_hex: None,
      weight: None
    }
  }

  pub fn set_bits(&mut self, bits: String) {
    self.bits = Some(bits);
  }

  pub fn with_bits(mut self, bits: String) -> GithubComSatstreamSsUtilsBitcoinCliBlock1 {
    self.bits = Some(bits);
    self
  }

  pub fn bits(&self) -> Option<&String> {
    self.bits.as_ref()
  }

  pub fn reset_bits(&mut self) {
    self.bits = None;
  }

  pub fn set_chainwork(&mut self, chainwork: String) {
    self.chainwork = Some(chainwork);
  }

  pub fn with_chainwork(mut self, chainwork: String) -> GithubComSatstreamSsUtilsBitcoinCliBlock1 {
    self.chainwork = Some(chainwork);
    self
  }

  pub fn chainwork(&self) -> Option<&String> {
    self.chainwork.as_ref()
  }

  pub fn reset_chainwork(&mut self) {
    self.chainwork = None;
  }

  pub fn set_confirmations(&mut self, confirmations: i32) {
    self.confirmations = Some(confirmations);
  }

  pub fn with_confirmations(mut self, confirmations: i32) -> GithubComSatstreamSsUtilsBitcoinCliBlock1 {
    self.confirmations = Some(confirmations);
    self
  }

  pub fn confirmations(&self) -> Option<&i32> {
    self.confirmations.as_ref()
  }

  pub fn reset_confirmations(&mut self) {
    self.confirmations = None;
  }

  pub fn set_difficulty(&mut self, difficulty: f32) {
    self.difficulty = Some(difficulty);
  }

  pub fn with_difficulty(mut self, difficulty: f32) -> GithubComSatstreamSsUtilsBitcoinCliBlock1 {
    self.difficulty = Some(difficulty);
    self
  }

  pub fn difficulty(&self) -> Option<&f32> {
    self.difficulty.as_ref()
  }

  pub fn reset_difficulty(&mut self) {
    self.difficulty = None;
  }

  pub fn set_hash(&mut self, hash: String) {
    self.hash = Some(hash);
  }

  pub fn with_hash(mut self, hash: String) -> GithubComSatstreamSsUtilsBitcoinCliBlock1 {
    self.hash = Some(hash);
    self
  }

  pub fn hash(&self) -> Option<&String> {
    self.hash.as_ref()
  }

  pub fn reset_hash(&mut self) {
    self.hash = None;
  }

  pub fn set_height(&mut self, height: i32) {
    self.height = Some(height);
  }

  pub fn with_height(mut self, height: i32) -> GithubComSatstreamSsUtilsBitcoinCliBlock1 {
    self.height = Some(height);
    self
  }

  pub fn height(&self) -> Option<&i32> {
    self.height.as_ref()
  }

  pub fn reset_height(&mut self) {
    self.height = None;
  }

  pub fn set_mediantime(&mut self, mediantime: i32) {
    self.mediantime = Some(mediantime);
  }

  pub fn with_mediantime(mut self, mediantime: i32) -> GithubComSatstreamSsUtilsBitcoinCliBlock1 {
    self.mediantime = Some(mediantime);
    self
  }

  pub fn mediantime(&self) -> Option<&i32> {
    self.mediantime.as_ref()
  }

  pub fn reset_mediantime(&mut self) {
    self.mediantime = None;
  }

  pub fn set_merkleroot(&mut self, merkleroot: String) {
    self.merkleroot = Some(merkleroot);
  }

  pub fn with_merkleroot(mut self, merkleroot: String) -> GithubComSatstreamSsUtilsBitcoinCliBlock1 {
    self.merkleroot = Some(merkleroot);
    self
  }

  pub fn merkleroot(&self) -> Option<&String> {
    self.merkleroot.as_ref()
  }

  pub fn reset_merkleroot(&mut self) {
    self.merkleroot = None;
  }

  pub fn set_n_tx(&mut self, n_tx: i32) {
    self.n_tx = Some(n_tx);
  }

  pub fn with_n_tx(mut self, n_tx: i32) -> GithubComSatstreamSsUtilsBitcoinCliBlock1 {
    self.n_tx = Some(n_tx);
    self
  }

  pub fn n_tx(&self) -> Option<&i32> {
    self.n_tx.as_ref()
  }

  pub fn reset_n_tx(&mut self) {
    self.n_tx = None;
  }

  pub fn set_nextblockhash(&mut self, nextblockhash: String) {
    self.nextblockhash = Some(nextblockhash);
  }

  pub fn with_nextblockhash(mut self, nextblockhash: String) -> GithubComSatstreamSsUtilsBitcoinCliBlock1 {
    self.nextblockhash = Some(nextblockhash);
    self
  }

  pub fn nextblockhash(&self) -> Option<&String> {
    self.nextblockhash.as_ref()
  }

  pub fn reset_nextblockhash(&mut self) {
    self.nextblockhash = None;
  }

  pub fn set_nonce(&mut self, nonce: i32) {
    self.nonce = Some(nonce);
  }

  pub fn with_nonce(mut self, nonce: i32) -> GithubComSatstreamSsUtilsBitcoinCliBlock1 {
    self.nonce = Some(nonce);
    self
  }

  pub fn nonce(&self) -> Option<&i32> {
    self.nonce.as_ref()
  }

  pub fn reset_nonce(&mut self) {
    self.nonce = None;
  }

  pub fn set_previousblockhash(&mut self, previousblockhash: String) {
    self.previousblockhash = Some(previousblockhash);
  }

  pub fn with_previousblockhash(mut self, previousblockhash: String) -> GithubComSatstreamSsUtilsBitcoinCliBlock1 {
    self.previousblockhash = Some(previousblockhash);
    self
  }

  pub fn previousblockhash(&self) -> Option<&String> {
    self.previousblockhash.as_ref()
  }

  pub fn reset_previousblockhash(&mut self) {
    self.previousblockhash = None;
  }

  pub fn set_size(&mut self, size: i32) {
    self.size = Some(size);
  }

  pub fn with_size(mut self, size: i32) -> GithubComSatstreamSsUtilsBitcoinCliBlock1 {
    self.size = Some(size);
    self
  }

  pub fn size(&self) -> Option<&i32> {
    self.size.as_ref()
  }

  pub fn reset_size(&mut self) {
    self.size = None;
  }

  pub fn set_strippedsize(&mut self, strippedsize: i32) {
    self.strippedsize = Some(strippedsize);
  }

  pub fn with_strippedsize(mut self, strippedsize: i32) -> GithubComSatstreamSsUtilsBitcoinCliBlock1 {
    self.strippedsize = Some(strippedsize);
    self
  }

  pub fn strippedsize(&self) -> Option<&i32> {
    self.strippedsize.as_ref()
  }

  pub fn reset_strippedsize(&mut self) {
    self.strippedsize = None;
  }

  pub fn set_time(&mut self, time: i32) {
    self.time = Some(time);
  }

  pub fn with_time(mut self, time: i32) -> GithubComSatstreamSsUtilsBitcoinCliBlock1 {
    self.time = Some(time);
    self
  }

  pub fn time(&self) -> Option<&i32> {
    self.time.as_ref()
  }

  pub fn reset_time(&mut self) {
    self.time = None;
  }

  pub fn set_tx(&mut self, tx: Vec<String>) {
    self.tx = Some(tx);
  }

  pub fn with_tx(mut self, tx: Vec<String>) -> GithubComSatstreamSsUtilsBitcoinCliBlock1 {
    self.tx = Some(tx);
    self
  }

  pub fn tx(&self) -> Option<&Vec<String>> {
    self.tx.as_ref()
  }

  pub fn reset_tx(&mut self) {
    self.tx = None;
  }

  pub fn set_version(&mut self, version: i32) {
    self.version = Some(version);
  }

  pub fn with_version(mut self, version: i32) -> GithubComSatstreamSsUtilsBitcoinCliBlock1 {
    self.version = Some(version);
    self
  }

  pub fn version(&self) -> Option<&i32> {
    self.version.as_ref()
  }

  pub fn reset_version(&mut self) {
    self.version = None;
  }

  pub fn set_version_hex(&mut self, version_hex: String) {
    self.version_hex = Some(version_hex);
  }

  pub fn with_version_hex(mut self, version_hex: String) -> GithubComSatstreamSsUtilsBitcoinCliBlock1 {
    self.version_hex = Some(version_hex);
    self
  }

  pub fn version_hex(&self) -> Option<&String> {
    self.version_hex.as_ref()
  }

  pub fn reset_version_hex(&mut self) {
    self.version_hex = None;
  }

  pub fn set_weight(&mut self, weight: i32) {
    self.weight = Some(weight);
  }

  pub fn with_weight(mut self, weight: i32) -> GithubComSatstreamSsUtilsBitcoinCliBlock1 {
    self.weight = Some(weight);
    self
  }

  pub fn weight(&self) -> Option<&i32> {
    self.weight.as_ref()
  }

  pub fn reset_weight(&mut self) {
    self.weight = None;
  }

}



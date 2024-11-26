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
pub struct GithubComSatstreamSsUtilsOrdServerResponsesInscriptionData {
  #[serde(rename = "body")]
  body: Option<Vec<i32>>,
  #[serde(rename = "content_encoding")]
  content_encoding: Option<String>,
  #[serde(rename = "content_type")]
  content_type: Option<Vec<i32>>,
  #[serde(rename = "delegate")]
  delegate: Option<String>,
  #[serde(rename = "duplicate_field")]
  duplicate_field: Option<bool>,
  #[serde(rename = "incomplete_field")]
  incomplete_field: Option<bool>,
  #[serde(rename = "metadata")]
  metadata: Option<String>,
  #[serde(rename = "metaprotocol")]
  metaprotocol: Option<String>,
  #[serde(rename = "parents")]
  parents: Option<Vec<String>>,
  #[serde(rename = "pointer")]
  pointer: Option<String>,
  #[serde(rename = "rune")]
  rune: Option<String>,
  #[serde(rename = "unrecognized_even_field")]
  unrecognized_even_field: Option<bool>
}

impl GithubComSatstreamSsUtilsOrdServerResponsesInscriptionData {
  pub fn new() -> GithubComSatstreamSsUtilsOrdServerResponsesInscriptionData {
    GithubComSatstreamSsUtilsOrdServerResponsesInscriptionData {
      body: None,
      content_encoding: None,
      content_type: None,
      delegate: None,
      duplicate_field: None,
      incomplete_field: None,
      metadata: None,
      metaprotocol: None,
      parents: None,
      pointer: None,
      rune: None,
      unrecognized_even_field: None
    }
  }

  pub fn set_body(&mut self, body: Vec<i32>) {
    self.body = Some(body);
  }

  pub fn with_body(mut self, body: Vec<i32>) -> GithubComSatstreamSsUtilsOrdServerResponsesInscriptionData {
    self.body = Some(body);
    self
  }

  pub fn body(&self) -> Option<&Vec<i32>> {
    self.body.as_ref()
  }

  pub fn reset_body(&mut self) {
    self.body = None;
  }

  pub fn set_content_encoding(&mut self, content_encoding: String) {
    self.content_encoding = Some(content_encoding);
  }

  pub fn with_content_encoding(mut self, content_encoding: String) -> GithubComSatstreamSsUtilsOrdServerResponsesInscriptionData {
    self.content_encoding = Some(content_encoding);
    self
  }

  pub fn content_encoding(&self) -> Option<&String> {
    self.content_encoding.as_ref()
  }

  pub fn reset_content_encoding(&mut self) {
    self.content_encoding = None;
  }

  pub fn set_content_type(&mut self, content_type: Vec<i32>) {
    self.content_type = Some(content_type);
  }

  pub fn with_content_type(mut self, content_type: Vec<i32>) -> GithubComSatstreamSsUtilsOrdServerResponsesInscriptionData {
    self.content_type = Some(content_type);
    self
  }

  pub fn content_type(&self) -> Option<&Vec<i32>> {
    self.content_type.as_ref()
  }

  pub fn reset_content_type(&mut self) {
    self.content_type = None;
  }

  pub fn set_delegate(&mut self, delegate: String) {
    self.delegate = Some(delegate);
  }

  pub fn with_delegate(mut self, delegate: String) -> GithubComSatstreamSsUtilsOrdServerResponsesInscriptionData {
    self.delegate = Some(delegate);
    self
  }

  pub fn delegate(&self) -> Option<&String> {
    self.delegate.as_ref()
  }

  pub fn reset_delegate(&mut self) {
    self.delegate = None;
  }

  pub fn set_duplicate_field(&mut self, duplicate_field: bool) {
    self.duplicate_field = Some(duplicate_field);
  }

  pub fn with_duplicate_field(mut self, duplicate_field: bool) -> GithubComSatstreamSsUtilsOrdServerResponsesInscriptionData {
    self.duplicate_field = Some(duplicate_field);
    self
  }

  pub fn duplicate_field(&self) -> Option<&bool> {
    self.duplicate_field.as_ref()
  }

  pub fn reset_duplicate_field(&mut self) {
    self.duplicate_field = None;
  }

  pub fn set_incomplete_field(&mut self, incomplete_field: bool) {
    self.incomplete_field = Some(incomplete_field);
  }

  pub fn with_incomplete_field(mut self, incomplete_field: bool) -> GithubComSatstreamSsUtilsOrdServerResponsesInscriptionData {
    self.incomplete_field = Some(incomplete_field);
    self
  }

  pub fn incomplete_field(&self) -> Option<&bool> {
    self.incomplete_field.as_ref()
  }

  pub fn reset_incomplete_field(&mut self) {
    self.incomplete_field = None;
  }

  pub fn set_metadata(&mut self, metadata: String) {
    self.metadata = Some(metadata);
  }

  pub fn with_metadata(mut self, metadata: String) -> GithubComSatstreamSsUtilsOrdServerResponsesInscriptionData {
    self.metadata = Some(metadata);
    self
  }

  pub fn metadata(&self) -> Option<&String> {
    self.metadata.as_ref()
  }

  pub fn reset_metadata(&mut self) {
    self.metadata = None;
  }

  pub fn set_metaprotocol(&mut self, metaprotocol: String) {
    self.metaprotocol = Some(metaprotocol);
  }

  pub fn with_metaprotocol(mut self, metaprotocol: String) -> GithubComSatstreamSsUtilsOrdServerResponsesInscriptionData {
    self.metaprotocol = Some(metaprotocol);
    self
  }

  pub fn metaprotocol(&self) -> Option<&String> {
    self.metaprotocol.as_ref()
  }

  pub fn reset_metaprotocol(&mut self) {
    self.metaprotocol = None;
  }

  pub fn set_parents(&mut self, parents: Vec<String>) {
    self.parents = Some(parents);
  }

  pub fn with_parents(mut self, parents: Vec<String>) -> GithubComSatstreamSsUtilsOrdServerResponsesInscriptionData {
    self.parents = Some(parents);
    self
  }

  pub fn parents(&self) -> Option<&Vec<String>> {
    self.parents.as_ref()
  }

  pub fn reset_parents(&mut self) {
    self.parents = None;
  }

  pub fn set_pointer(&mut self, pointer: String) {
    self.pointer = Some(pointer);
  }

  pub fn with_pointer(mut self, pointer: String) -> GithubComSatstreamSsUtilsOrdServerResponsesInscriptionData {
    self.pointer = Some(pointer);
    self
  }

  pub fn pointer(&self) -> Option<&String> {
    self.pointer.as_ref()
  }

  pub fn reset_pointer(&mut self) {
    self.pointer = None;
  }

  pub fn set_rune(&mut self, rune: String) {
    self.rune = Some(rune);
  }

  pub fn with_rune(mut self, rune: String) -> GithubComSatstreamSsUtilsOrdServerResponsesInscriptionData {
    self.rune = Some(rune);
    self
  }

  pub fn rune(&self) -> Option<&String> {
    self.rune.as_ref()
  }

  pub fn reset_rune(&mut self) {
    self.rune = None;
  }

  pub fn set_unrecognized_even_field(&mut self, unrecognized_even_field: bool) {
    self.unrecognized_even_field = Some(unrecognized_even_field);
  }

  pub fn with_unrecognized_even_field(mut self, unrecognized_even_field: bool) -> GithubComSatstreamSsUtilsOrdServerResponsesInscriptionData {
    self.unrecognized_even_field = Some(unrecognized_even_field);
    self
  }

  pub fn unrecognized_even_field(&self) -> Option<&bool> {
    self.unrecognized_even_field.as_ref()
  }

  pub fn reset_unrecognized_even_field(&mut self) {
    self.unrecognized_even_field = None;
  }

}




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
pub struct Input {
  #[serde(rename = "previous_output")]
  previous_output: Option<String>,
  #[serde(rename = "script_sig")]
  script_sig: Option<String>,
  #[serde(rename = "sequence")]
  sequence: Option<i32>,
  #[serde(rename = "witness")]
  witness: Option<Vec<String>>
}

impl Input {
  pub fn new() -> Input {
    Input {
      previous_output: None,
      script_sig: None,
      sequence: None,
      witness: None
    }
  }

  pub fn set_previous_output(&mut self, previous_output: String) {
    self.previous_output = Some(previous_output);
  }

  pub fn with_previous_output(mut self, previous_output: String) -> Input {
    self.previous_output = Some(previous_output);
    self
  }

  pub fn previous_output(&self) -> Option<&String> {
    self.previous_output.as_ref()
  }

  pub fn reset_previous_output(&mut self) {
    self.previous_output = None;
  }

  pub fn set_script_sig(&mut self, script_sig: String) {
    self.script_sig = Some(script_sig);
  }

  pub fn with_script_sig(mut self, script_sig: String) -> Input {
    self.script_sig = Some(script_sig);
    self
  }

  pub fn script_sig(&self) -> Option<&String> {
    self.script_sig.as_ref()
  }

  pub fn reset_script_sig(&mut self) {
    self.script_sig = None;
  }

  pub fn set_sequence(&mut self, sequence: i32) {
    self.sequence = Some(sequence);
  }

  pub fn with_sequence(mut self, sequence: i32) -> Input {
    self.sequence = Some(sequence);
    self
  }

  pub fn sequence(&self) -> Option<&i32> {
    self.sequence.as_ref()
  }

  pub fn reset_sequence(&mut self) {
    self.sequence = None;
  }

  pub fn set_witness(&mut self, witness: Vec<String>) {
    self.witness = Some(witness);
  }

  pub fn with_witness(mut self, witness: Vec<String>) -> Input {
    self.witness = Some(witness);
    self
  }

  pub fn witness(&self) -> Option<&Vec<String>> {
    self.witness.as_ref()
  }

  pub fn reset_witness(&mut self) {
    self.witness = None;
  }

}




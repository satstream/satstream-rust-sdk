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
pub struct GithubComSatstreamSsApiServerApiPsbtRequestsCreatePsbtOutput {
  #[serde(rename = "address")]
  address: Option<String>,
  #[serde(rename = "data")]
  data: Option<String>
}

impl GithubComSatstreamSsApiServerApiPsbtRequestsCreatePsbtOutput {
  pub fn new() -> GithubComSatstreamSsApiServerApiPsbtRequestsCreatePsbtOutput {
    GithubComSatstreamSsApiServerApiPsbtRequestsCreatePsbtOutput {
      address: None,
      data: None
    }
  }

  pub fn set_address(&mut self, address: String) {
    self.address = Some(address);
  }

  pub fn with_address(mut self, address: String) -> GithubComSatstreamSsApiServerApiPsbtRequestsCreatePsbtOutput {
    self.address = Some(address);
    self
  }

  pub fn address(&self) -> Option<&String> {
    self.address.as_ref()
  }

  pub fn reset_address(&mut self) {
    self.address = None;
  }

  pub fn set_data(&mut self, data: String) {
    self.data = Some(data);
  }

  pub fn with_data(mut self, data: String) -> GithubComSatstreamSsApiServerApiPsbtRequestsCreatePsbtOutput {
    self.data = Some(data);
    self
  }

  pub fn data(&self) -> Option<&String> {
    self.data.as_ref()
  }

  pub fn reset_data(&mut self) {
    self.data = None;
  }

}



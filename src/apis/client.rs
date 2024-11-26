use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient<C: hyper::client::Connect> {
  configuration: Rc<Configuration<C>>,
  bitcoin_api: Box<::apis::BitcoinApi>,
  blocks_api: Box<::apis::BlocksApi>,
  inscriptions_api: Box<::apis::InscriptionsApi>,
}

impl<C: hyper::client::Connect> APIClient<C> {
  pub fn new(configuration: Configuration<C>) -> APIClient<C> {
    let rc = Rc::new(configuration);

    APIClient {
      configuration: rc.clone(),
      bitcoin_api: Box::new(::apis::BitcoinApiClient::new(rc.clone())),
      blocks_api: Box::new(::apis::BlocksApiClient::new(rc.clone())),
      inscriptions_api: Box::new(::apis::InscriptionsApiClient::new(rc.clone())),
    }
  }

  pub fn bitcoin_api(&self) -> &::apis::BitcoinApi{
    self.bitcoin_api.as_ref()
  }

  pub fn blocks_api(&self) -> &::apis::BlocksApi{
    self.blocks_api.as_ref()
  }

  pub fn inscriptions_api(&self) -> &::apis::InscriptionsApi{
    self.inscriptions_api.as_ref()
  }


}

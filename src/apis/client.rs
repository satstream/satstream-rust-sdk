use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient<C: hyper::client::Connect> {
  configuration: Rc<Configuration<C>>,
  addresses_api: Box<::apis::AddressesApi>,
  blocks_api: Box<::apis::BlocksApi>,
  fees_api: Box<::apis::FeesApi>,
  mempool_api: Box<::apis::MempoolApi>,
  runes_api: Box<::apis::RunesApi>,
  transactions_api: Box<::apis::TransactionsApi>,
}

impl<C: hyper::client::Connect> APIClient<C> {
  pub fn new(configuration: Configuration<C>) -> APIClient<C> {
    let rc = Rc::new(configuration);

    APIClient {
      configuration: rc.clone(),
      addresses_api: Box::new(::apis::AddressesApiClient::new(rc.clone())),
      blocks_api: Box::new(::apis::BlocksApiClient::new(rc.clone())),
      fees_api: Box::new(::apis::FeesApiClient::new(rc.clone())),
      mempool_api: Box::new(::apis::MempoolApiClient::new(rc.clone())),
      runes_api: Box::new(::apis::RunesApiClient::new(rc.clone())),
      transactions_api: Box::new(::apis::TransactionsApiClient::new(rc.clone())),
    }
  }

  pub fn addresses_api(&self) -> &::apis::AddressesApi{
    self.addresses_api.as_ref()
  }

  pub fn blocks_api(&self) -> &::apis::BlocksApi{
    self.blocks_api.as_ref()
  }

  pub fn fees_api(&self) -> &::apis::FeesApi{
    self.fees_api.as_ref()
  }

  pub fn mempool_api(&self) -> &::apis::MempoolApi{
    self.mempool_api.as_ref()
  }

  pub fn runes_api(&self) -> &::apis::RunesApi{
    self.runes_api.as_ref()
  }

  pub fn transactions_api(&self) -> &::apis::TransactionsApi{
    self.transactions_api.as_ref()
  }


}

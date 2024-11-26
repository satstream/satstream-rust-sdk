use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient<C: hyper::client::Connect> {
  configuration: Rc<Configuration<C>>,
  addresses_api: Box<::apis::AddressesApi>,
  blocks_api: Box<::apis::BlocksApi>,
  fees_api: Box<::apis::FeesApi>,
  inscriptions_api: Box<::apis::InscriptionsApi>,
  mempool_api: Box<::apis::MempoolApi>,
  mining_api: Box<::apis::MiningApi>,
  network_api: Box<::apis::NetworkApi>,
  outputs_api: Box<::apis::OutputsApi>,
  psbts_api: Box<::apis::PSBTsApi>,
  runes_api: Box<::apis::RunesApi>,
  satoshis_api: Box<::apis::SatoshisApi>,
  scripts_api: Box<::apis::ScriptsApi>,
  status_api: Box<::apis::StatusApi>,
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
      inscriptions_api: Box::new(::apis::InscriptionsApiClient::new(rc.clone())),
      mempool_api: Box::new(::apis::MempoolApiClient::new(rc.clone())),
      mining_api: Box::new(::apis::MiningApiClient::new(rc.clone())),
      network_api: Box::new(::apis::NetworkApiClient::new(rc.clone())),
      outputs_api: Box::new(::apis::OutputsApiClient::new(rc.clone())),
      psbts_api: Box::new(::apis::PSBTsApiClient::new(rc.clone())),
      runes_api: Box::new(::apis::RunesApiClient::new(rc.clone())),
      satoshis_api: Box::new(::apis::SatoshisApiClient::new(rc.clone())),
      scripts_api: Box::new(::apis::ScriptsApiClient::new(rc.clone())),
      status_api: Box::new(::apis::StatusApiClient::new(rc.clone())),
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

  pub fn inscriptions_api(&self) -> &::apis::InscriptionsApi{
    self.inscriptions_api.as_ref()
  }

  pub fn mempool_api(&self) -> &::apis::MempoolApi{
    self.mempool_api.as_ref()
  }

  pub fn mining_api(&self) -> &::apis::MiningApi{
    self.mining_api.as_ref()
  }

  pub fn network_api(&self) -> &::apis::NetworkApi{
    self.network_api.as_ref()
  }

  pub fn outputs_api(&self) -> &::apis::OutputsApi{
    self.outputs_api.as_ref()
  }

  pub fn psbts_api(&self) -> &::apis::PSBTsApi{
    self.psbts_api.as_ref()
  }

  pub fn runes_api(&self) -> &::apis::RunesApi{
    self.runes_api.as_ref()
  }

  pub fn satoshis_api(&self) -> &::apis::SatoshisApi{
    self.satoshis_api.as_ref()
  }

  pub fn scripts_api(&self) -> &::apis::ScriptsApi{
    self.scripts_api.as_ref()
  }

  pub fn status_api(&self) -> &::apis::StatusApi{
    self.status_api.as_ref()
  }

  pub fn transactions_api(&self) -> &::apis::TransactionsApi{
    self.transactions_api.as_ref()
  }


}

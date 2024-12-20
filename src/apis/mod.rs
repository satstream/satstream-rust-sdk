use hyper;
use serde;
use serde_json;

#[derive(Debug)]
pub enum Error<T> {
    Hyper(hyper::Error),
    Serde(serde_json::Error),
    ApiError(ApiError<T>),
}

#[derive(Debug)]
pub struct ApiError<T> {
    pub code: hyper::StatusCode,
    pub content: Option<T>,
}

impl<'de, T> From<(hyper::StatusCode, &'de [u8])> for Error<T> 
    where T: serde::Deserialize<'de> {
    fn from(e: (hyper::StatusCode, &'de [u8])) -> Self {
        if e.1.len() == 0 {
            return Error::ApiError(ApiError{
                code: e.0,
                content: None,
            });
        }
        match serde_json::from_slice::<T>(e.1) {
            Ok(t) => Error::ApiError(ApiError{
                code: e.0,
                content: Some(t),
            }),
            Err(e) => {
                Error::from(e)
            }
        }
    }
}

impl<T> From<hyper::Error> for Error<T> {
    fn from(e: hyper::Error) -> Self {
        return Error::Hyper(e)
    }
}

impl<T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        return Error::Serde(e)
    }
}

use super::models::*;

mod addresses_api;
pub use self::addresses_api::{ AddressesApi, AddressesApiClient };
mod blocks_api;
pub use self::blocks_api::{ BlocksApi, BlocksApiClient };
mod fees_api;
pub use self::fees_api::{ FeesApi, FeesApiClient };
mod inscriptions_api;
pub use self::inscriptions_api::{ InscriptionsApi, InscriptionsApiClient };
mod mempool_api;
pub use self::mempool_api::{ MempoolApi, MempoolApiClient };
mod mining_api;
pub use self::mining_api::{ MiningApi, MiningApiClient };
mod network_api;
pub use self::network_api::{ NetworkApi, NetworkApiClient };
mod outputs_api;
pub use self::outputs_api::{ OutputsApi, OutputsApiClient };
mod psbts_api;
pub use self::psbts_api::{ PSBTsApi, PSBTsApiClient };
mod runes_api;
pub use self::runes_api::{ RunesApi, RunesApiClient };
mod satoshis_api;
pub use self::satoshis_api::{ SatoshisApi, SatoshisApiClient };
mod scripts_api;
pub use self::scripts_api::{ ScriptsApi, ScriptsApiClient };
mod status_api;
pub use self::status_api::{ StatusApi, StatusApiClient };
mod transactions_api;
pub use self::transactions_api::{ TransactionsApi, TransactionsApiClient };

pub mod configuration;
pub mod client;

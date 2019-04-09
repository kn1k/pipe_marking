use exonum::{
    api::{self, ServiceApiBuilder, ServiceApiState},
    blockchain::{self, BlockProof, TransactionMessage},
    crypto::{Hash, PublicKey},
    explorer::BlockchainExplorer,
    helpers::Height,
    storage::{ListProof, MapProof},
};

use crate::pipe_type::PipeType;
use super::{schema::Schema, SERVICE_ID};


/// Public service API description.
#[derive(Debug, Clone, Copy)]
pub struct PublicApi;

impl PublicApi {
    /// Wires the above endpoint to public scope of the given `ServiceApiBuilder`.
    pub fn wire(builder: &mut ServiceApiBuilder) {
//        builder
//            .public_scope()
//            .endpoint("v1/wallets/info", Self::wallet_info);
    }
}
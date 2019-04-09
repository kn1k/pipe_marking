//! Pipe marking database schema.

use exonum::{
    crypto::{Hash, PublicKey},
    storage::{Fork, ProofListIndex, ProofMapIndex, Snapshot},
};

use crate::pipe_type::PipeType;

/// Pipe types table name
pub const PIPE_TYPES_TABLE: &str = "pipe_marking.pipe_type";
/// Pipe type history table name
pub const PIPE_TYPES_HISTORY_TABLE: &str = "pipe_marking.pipe_type.history";

/// Database schema.
#[derive(Debug)]
pub struct Schema<T> {
    view: T,
}

impl<T> AsMut<T> for Schema<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.view
    }
}

impl<T> Schema<T>
    where
        T: AsRef<dyn Snapshot>,
{
    /// Creates a new schema from the database view.
    pub fn new(view: T) -> Self {
        Schema { view }
    }

    /// Returns `ProofMapIndex` with pipe types.
    pub fn pipe_types(&self) -> ProofMapIndex<&T, PublicKey, PipeType> {
        ProofMapIndex::new(PIPE_TYPES_TABLE, &self.view)
    }

    /// Returns history of the pipe type with the given public key.
    pub fn pipe_type_history(&self, public_key: &PublicKey) -> ProofListIndex<&T, Hash> {
        ProofListIndex::new_in_family(PIPE_TYPES_HISTORY_TABLE, public_key, &self.view)
    }

    /// Returns pipe type for the given public key.
    pub fn pipe_type(&self, pub_key: &PublicKey) -> Option<PipeType> {
        self.pipe_types().get(pub_key)
    }

    /// Returns the state hash of service.
    pub fn state_hash(&self) -> Vec<Hash> {
        vec![self.pipe_types().merkle_root()]
    }
}

/// Implementation of mutable methods.
impl<'a> Schema<&'a mut Fork> {
    /// Returns mutable `ProofMapIndex` with pipe types.
    pub fn pipe_types_mut(&mut self) -> ProofMapIndex<&mut Fork, PublicKey, PipeType> {
        ProofMapIndex::new(PIPE_TYPES_TABLE, &mut self.view)
    }

    /// Returns history for the pipe type by the given public key.
    pub fn pipe_type_history_mut(
        &mut self,
        public_key: &PublicKey,
    ) -> ProofListIndex<&mut Fork, Hash> {
        ProofListIndex::new_in_family(PIPE_TYPES_HISTORY_TABLE, public_key, &mut self.view)
    }

    /// Create new pipe type and append first record to its history.
    pub fn create_pipe_type(
        &mut self,
        pipe_type: &PublicKey,
        pipe_producer: &PublicKey,
        pipe_type_id_1c: &str,
        pipe_type_name_1c: &str,
        pipe_type_name_gost: &str,
        weight_meter_of_pipe: u32,
        certificate_number: &str,
        certificate_status: &str,
        sdr: &str,
        outer_diameter: u32,
        dn: u32,
        pu: u32,
        check_result: &str,
        transaction: &Hash) {
        let created_pipe_type = {
            let mut history = self.pipe_type_history_mut(pipe_type);
            history.push(*transaction);
            let history_hash = history.merkle_root();

            PipeType::new(pipe_type, pipe_producer, pipe_type_id_1c, pipe_type_name_1c,
                        pipe_type_name_gost, weight_meter_of_pipe, certificate_number,
                        certificate_status, sdr, outer_diameter, dn, pu, check_result,
                        history.len(), &history_hash)
        };
        self.pipe_types_mut().put(pipe_type, created_pipe_type);
    }
}


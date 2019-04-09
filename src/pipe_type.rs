use exonum::crypto::{Hash, PublicKey};
use super::proto;

/// Stores information about a pipe type
#[derive(Clone, Debug, ProtobufConvert)]
#[exonum(pb = "proto::PipeType", serde_pb_convert)]
pub struct PipeType {
    /// `PublicKey` of pipe type.
    pub pipe_type: PublicKey,
    /// UID of pipe producer
    pub pipe_producer: PublicKey,
    /// Pipe type id from 1C
    pub pipe_type_id_1c: String,
    /// Pipe type name from 1c
    pub pipe_type_name_1c: String,
    /// Pipe type name from GOST
    pub pipe_type_name_gost: String,
    /// Weight of 1 meter of pipe
    pub weight_meter_of_pipe: u32,
    /// Certificate of conformity number
    pub certificate_number: String,
    /// Certificate of conformity status TODO change type?
    pub certificate_status: String,
    /// SDR
    pub sdr: String,
    /// Outer diameter
    pub outer_diameter: u32,
    /// DN
    pub dn: u32,
    /// PU
    pub pu: u32,
    /// Check result
    pub check_result: String,
    /// Length of the transactions history.
    pub history_len: u64,
    /// `Hash` of the transactions history.
    pub history_hash: Hash,
}

impl PipeType {
    /// Creates new pipe type
    pub fn new(
        &pipe_type: &PublicKey,
        &pipe_producer: &PublicKey,
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
        history_len: u64,
        &history_hash: &Hash,
    ) -> Self {
        Self {
            pipe_type,
            pipe_producer,
            pipe_type_id_1c: pipe_type_id_1c.to_owned(),
            pipe_type_name_1c: pipe_type_name_1c.to_owned(),
            pipe_type_name_gost: pipe_type_name_gost.to_owned(),
            weight_meter_of_pipe,
            certificate_number: certificate_number.to_owned(),
            certificate_status: certificate_status.to_owned(),
            sdr: sdr.to_owned(), outer_diameter, dn, pu,
            check_result: check_result.to_owned(), history_len, history_hash
        }
    }
}
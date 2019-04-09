#![allow(bare_trait_objects)]

use exonum::{
    blockchain::{ExecutionError, ExecutionResult, Transaction, TransactionContext},
    crypto::{PublicKey, SecretKey},
    messages::{Message, RawTransaction, Signed},
};

use super::{ proto, schema::Schema, SERVICE_ID };

/// Error codes emitted by pipes transactions during execution.
#[derive(Debug, Fail)]
#[repr(u8)]
pub enum Error {
    /// Pipe type already exists.
    ///
    /// Can be emitted by `CreatePipeType`.
    #[fail(display = "Pipe type already exists")]
    PipeTypeAlreadyExists = 0,

    // TODO add some errors
}

impl From<Error> for ExecutionError {
    fn from(value: Error) -> ExecutionError {
        let description = format!("{}", value);
        ExecutionError::with_description(value as u8, description)
    }
}

/// Create pipe type.
#[derive(Serialize, Deserialize, Clone, Debug, ProtobufConvert)]
#[exonum(pb = "proto::CreatePipeType")]
pub struct CreatePipeType {
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
}

/// Transaction group.
#[derive(Serialize, Deserialize, Clone, Debug, TransactionSet)]
pub enum PipeMarkingTransactions {
    /// CreatePipeType tx.
    CreatePipeType(CreatePipeType),
}

impl CreatePipeType {
    #[doc(hidden)]
    pub fn sign(
        pk: &PublicKey,
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
        sk: &SecretKey) -> Signed<RawTransaction> {

        Message::sign_transaction(
            Self { pipe_type, pipe_producer, pipe_type_id_1c: pipe_type_id_1c.to_owned(),
                pipe_type_name_1c: pipe_type_name_1c.to_owned(),
                pipe_type_name_gost: pipe_type_name_gost.to_owned(), weight_meter_of_pipe,
                certificate_number: certificate_number.to_owned(),
                certificate_status: certificate_status.to_owned(), sdr: sdr.to_owned(),
                outer_diameter, dn, pu,
                check_result: check_result.to_owned() },
            SERVICE_ID,
            *pk,
            sk,
        )
    }
}

impl Transaction for CreatePipeType {
    fn execute(&self, mut context: TransactionContext) -> ExecutionResult {
        // let pub_key = &context.author();
        let hash = context.tx_hash();

        let mut schema = Schema::new(context.fork());
            
        let pipe_type = &self.pipe_type;

        if schema.pipe_type(pipe_type).is_none() {
            //let name = &self.name;

            let pipe_producer = &self.pipe_producer;
            let pipe_type_id_1c = &self.pipe_type_id_1c;
            let pipe_type_name_1c = &self.pipe_type_name_1c;
            let pipe_type_name_gost = &self.pipe_type_name_gost;
            let weight_meter_of_pipe = self.weight_meter_of_pipe;
            let certificate_number = &self.certificate_number;
            let certificate_status = &self.certificate_status;
            let sdr = &self.sdr;
            let outer_diameter = self.outer_diameter;
            let dn = self.dn;
            let pu = self.pu;
            let check_result = &self.check_result;

            schema.create_pipe_type(pipe_type, pipe_producer, pipe_type_id_1c, 
                pipe_type_name_1c, pipe_type_name_gost, weight_meter_of_pipe, 
                certificate_number, certificate_status, sdr, outer_diameter, 
                dn, pu, check_result, &hash);
            Ok(())
        } else {
            Err(Error::PipeTypeAlreadyExists)?
        }
    }
}
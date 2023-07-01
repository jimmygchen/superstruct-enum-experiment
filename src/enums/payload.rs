use serde::{Deserialize, Serialize};
use ssz_derive::{Decode, Encode};
use superstruct::superstruct;
use tree_hash::Hash256;

#[superstruct(
    variants(Bellatrix, Capella, Deneb,),
    variant_attributes(derive(Debug, Serialize, Deserialize, Default, Decode, Encode),)
)]
#[derive(Serialize, Deserialize, Debug, Encode)]
#[serde(untagged)]
#[ssz(enum_behaviour = "transparent")]
pub struct Payload {
    #[superstruct(only(Bellatrix, Capella, Deneb))]
    pub transactions: Vec<Hash256>,
    #[superstruct(only(Capella, Deneb))]
    pub withdrawals: Vec<Hash256>,
    #[superstruct(only(Deneb))]
    pub excess_data_gas: Vec<u64>,
}

#[superstruct(
    variants(Bellatrix, Capella, Deneb,),
    variant_attributes(derive(Debug, Serialize, Deserialize, Default, Decode, Encode),)
)]
#[derive(Serialize, Deserialize, Debug, Encode)]
#[serde(untagged)]
#[ssz(enum_behaviour = "transparent")]
pub struct PayloadHeader {
    #[superstruct(only(Bellatrix, Capella, Deneb))]
    pub transaction_roots: Vec<Hash256>,
    #[superstruct(only(Capella, Deneb))]
    pub withdrawals_roots: Vec<Hash256>,
    #[superstruct(only(Deneb))]
    pub excess_data_gas: Vec<u64>,
}

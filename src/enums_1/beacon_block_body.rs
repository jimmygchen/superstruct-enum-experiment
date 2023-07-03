use super::payload::*;
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
pub struct FullBeaconBlockBody {
    pub eth1_block_hash: Hash256,
    #[superstruct(
        only(Bellatrix),
        partial_getter(rename = "execution_payload_bellatrix")
    )]
    pub execution_payload: PayloadBellatrix,
    #[superstruct(only(Capella), partial_getter(rename = "execution_payload_capella"))]
    pub execution_payload: PayloadCapella,
    #[superstruct(only(Deneb), partial_getter(rename = "execution_payload_deneb"))]
    pub execution_payload: PayloadDeneb,
    #[superstruct(only(Capella, Deneb))]
    pub bls_to_execution_changes: Vec<u64>,
    #[superstruct(only(Deneb))]
    pub kzg_commitments: Vec<Hash256>,
}

#[superstruct(
    variants(Bellatrix, Capella, Deneb,),
    variant_attributes(derive(Debug, Serialize, Deserialize, Default, Decode, Encode),)
)]
#[derive(Serialize, Deserialize, Debug, Encode)]
#[serde(untagged)]
#[ssz(enum_behaviour = "transparent")]
pub struct BlindedBeaconBlockBody {
    pub eth1_block_hash: Hash256,
    #[superstruct(
        only(Bellatrix),
        partial_getter(rename = "execution_payload_header_bellatrix")
    )]
    pub execution_payload_header: PayloadHeaderBellatrix,
    #[superstruct(
        only(Capella),
        partial_getter(rename = "execution_payload_header_capella")
    )]
    pub execution_payload_header: PayloadHeaderCapella,
    #[superstruct(only(Deneb), partial_getter(rename = "execution_payload_header_deneb"))]
    pub execution_payload_header: PayloadHeaderDeneb,
    #[superstruct(only(Capella, Deneb))]
    pub bls_to_execution_changes: Vec<u64>,
    #[superstruct(only(Deneb))]
    pub kzg_commitments: Vec<Hash256>,
}

use super::beacon_block_body::*;
use serde::{Deserialize, Serialize};
use superstruct::superstruct;

#[superstruct(
    variants(Full, Blinded),
    variant_attributes(derive(Debug, Serialize, Deserialize))
)]
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub struct SignedBlockAndBlobSidecars {
    /// full variant
    #[superstruct(only(Full))]
    pub signed_block: Signed<BeaconBlock>,
    /// using the enum type so it can be passed to functions that uses enum as parameters
    #[superstruct(only(Full))]
    pub signed_blob_sidecars: Signed<BlobSidecars>,
    /// blinded variant
    #[superstruct(only(Blinded))]
    pub signed_blinded_block: Signed<BeaconBlock>,
    #[superstruct(only(Blinded))]
    pub signed_blinded_blob_sidecars: Signed<BlobSidecars>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Signed<T> {
    pub message: T,
    pub signature: String,
}

impl<T> Signed<T> {
    pub fn new(message: T, signature: String) -> Self {
        Self { message, signature }
    }
}

#[superstruct(
    variants(Full, Blinded),
    variant_attributes(derive(Debug, Serialize, Deserialize))
)]
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub struct BlockAndBlobSidecars {
    /// full variant
    #[superstruct(only(Full))]
    pub block: BeaconBlock,
    #[superstruct(only(Full))]
    pub blob_sidecars: BlobSidecars,
    /// blinded variant
    #[superstruct(only(Blinded))]
    pub blinded_block: BeaconBlock,
    #[superstruct(only(Blinded))]
    pub blinded_blob_sidecars: BlobSidecars,
}

#[superstruct(
    variants(Full, Blinded),
    variant_attributes(derive(Debug, Serialize, Deserialize, Default))
)]
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub struct BlobSidecars {
    #[superstruct(only(Full))]
    pub blob: String,
    #[superstruct(only(Blinded))]
    pub blot_root: String,
}

#[superstruct(
    variants(
        BellatrixBlinded,
        BellatrixFull,
        CapellaBlinded,
        CapellaFull,
        DenebBlinded,
        DenebFull
    ),
    variant_attributes(derive(Debug, Serialize, Deserialize, Default))
)]
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub struct BeaconBlock {
    pub slot: usize,
    #[superstruct(
        only(BellatrixBlinded),
        partial_getter(rename = "body_bellatrix_blinded")
    )]
    pub body: BeaconBlockBodyBellatrixBlinded,
    #[superstruct(only(BellatrixFull), partial_getter(rename = "body_bellatrix_full"))]
    pub body: BeaconBlockBodyBellatrixFull,
    #[superstruct(only(CapellaBlinded), partial_getter(rename = "body_capella_blinded"))]
    pub body: BeaconBlockBodyCapellaBlinded,
    #[superstruct(only(CapellaFull), partial_getter(rename = "body_capella_full"))]
    pub body: BeaconBlockBodyCapellaFull,
    #[superstruct(only(DenebBlinded), partial_getter(rename = "body_deneb_blinded"))]
    pub body: BeaconBlockBodyDenebBlinded,
    #[superstruct(only(DenebFull), partial_getter(rename = "body_deneb_full"))]
    pub body: BeaconBlockBodyDenebFull,
}

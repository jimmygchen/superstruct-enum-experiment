use super::beacon_block_body::*;
use serde::{Deserialize, Serialize};
use ssz_derive::{Decode, Encode};
use superstruct::superstruct;
use tree_hash::Hash256;

/// Only used in APIs, may need encoding but no decoding required
#[derive(Serialize, Deserialize, Debug)]
pub struct SignedBlockAndBlobSidecars {
    pub signed_block: Signed<BeaconBlock>,
    pub signed_blob_sidecars: Signed<BlobSidecarsFull>,
}

/// Only used in APIs, may need encoding but no decoding required
#[derive(Serialize, Deserialize, Debug)]
pub struct SignedBlindedBlockAndBlobSidecars {
    pub signed_blinded_block: Signed<BlindedBeaconBlock>,
    pub signed_blinded_blob_sidecars: Signed<BlobSidecarsBlinded>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Signed<T> {
    pub message: T,
    pub signature: Hash256,
}

impl<T: ssz::Encode> Signed<T> {
    pub fn new(message: T, signature: Hash256) -> Self {
        Self { message, signature }
    }
}

#[superstruct(
    variants(Full, Blinded),
    variant_attributes(derive(Debug, Serialize, Deserialize))
)]
// This struct has no fork versions, so we are able to use superstruct for full/blinded purpose.
/// Only used in APIs, may need encoding but no decoding required
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub struct BlockAndBlobSidecars {
    /// full variant
    #[superstruct(only(Full))]
    pub block: BeaconBlock,
    #[superstruct(only(Full))]
    pub blob_sidecars: BlobSidecarsFull,
    /// blinded variant
    #[superstruct(only(Blinded))]
    pub blinded_block: BlindedBeaconBlock,
    #[superstruct(only(Blinded))]
    pub blinded_blob_sidecars: BlobSidecarsBlinded,
}

impl BlockAndBlobSidecarsFull {
    pub fn deconstruct(self) -> (BeaconBlock, BlobSidecarsFull) {
        (self.block, self.blob_sidecars)
    }
}

impl BlockAndBlobSidecarsBlinded {
    pub fn deconstruct(self) -> (BlindedBeaconBlock, BlobSidecarsBlinded) {
        (self.blinded_block, self.blinded_blob_sidecars)
    }
}

#[superstruct(
    variants(Full, Blinded),
    variant_attributes(derive(Debug, Serialize, Deserialize, Default, Decode, Encode))
)]
#[derive(Serialize, Deserialize, Debug, Encode)]
#[serde(untagged)]
#[ssz(enum_behaviour = "transparent")]
pub struct BlobSidecars {
    #[superstruct(only(Full))]
    pub blob: Hash256,
    #[superstruct(only(Blinded))]
    pub blot_root: Hash256,
}

#[superstruct(
    variants(Bellatrix, Capella, Deneb,),
    variant_attributes(derive(Debug, Serialize, Deserialize, Default, Encode, Decode))
)]
#[derive(Serialize, Deserialize, Debug, Encode)]
#[serde(untagged)]
#[ssz(enum_behaviour = "transparent")]
pub struct BeaconBlock {
    pub slot: usize,
    #[superstruct(only(Bellatrix), partial_getter(rename = "body_bellatrix"))]
    pub body: FullBeaconBlockBodyBellatrix,
    #[superstruct(only(Capella), partial_getter(rename = "body_capella"))]
    pub body: FullBeaconBlockBodyCapella,
    #[superstruct(only(Deneb), partial_getter(rename = "body_deneb"))]
    pub body: FullBeaconBlockBodyDeneb,
}

#[superstruct(
    variants(Bellatrix, Capella, Deneb,),
    variant_attributes(derive(Debug, Serialize, Deserialize, Default, Encode, Decode))
)]
#[derive(Serialize, Deserialize, Debug, Encode)]
#[serde(untagged)]
#[ssz(enum_behaviour = "transparent")]
pub struct BlindedBeaconBlock {
    pub slot: usize,
    #[superstruct(only(Bellatrix), partial_getter(rename = "body_bellatrix"))]
    pub body: BlindedBeaconBlockBodyBellatrix,
    #[superstruct(only(Capella), partial_getter(rename = "body_capella"))]
    pub body: BlindedBeaconBlockBodyCapella,
    #[superstruct(only(Deneb), partial_getter(rename = "body_deneb"))]
    pub body: BlindedBeaconBlockBodyDeneb,
}

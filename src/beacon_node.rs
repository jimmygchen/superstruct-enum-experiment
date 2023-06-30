use crate::enums::*;
use serde::{Deserialize, Serialize};

pub enum BlobEnabled {
    True,
    False,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum BlockContents {
    Block(BeaconBlock),
    BlockAndBlobSidecar(BlockAndBlobSidecars),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum SignedBlockContents {
    Block(Signed<BeaconBlock>),
    BlockAndBlobSidecar(SignedBlockAndBlobSidecars),
}

pub struct BeaconNode {}

impl BeaconNode {
    pub fn get_blinded_block_contents(&self, blob_enabled: BlobEnabled) -> BlockContents {
        match blob_enabled {
            BlobEnabled::True => BlockContents::BlockAndBlobSidecar(BlockAndBlobSidecars::Blinded(
                BlockAndBlobSidecarsBlinded {
                    blinded_block: BeaconBlock::DenebBlinded(BeaconBlockDenebBlinded::default()),
                    blinded_blob_sidecars: BlobSidecars::Blinded(BlobSidecarsBlinded::default()),
                },
            )),
            BlobEnabled::False => BlockContents::Block(BeaconBlock::CapellaBlinded(
                BeaconBlockCapellaBlinded::default(),
            )),
        }
    }

    pub fn get_block_contents(&self, blob_enabled: BlobEnabled) -> BlockContents {
        match blob_enabled {
            BlobEnabled::True => BlockContents::BlockAndBlobSidecar(BlockAndBlobSidecars::Full(
                BlockAndBlobSidecarsFull {
                    block: BeaconBlock::DenebFull(BeaconBlockDenebFull::default()),
                    blob_sidecars: BlobSidecars::Full(BlobSidecarsFull::default()),
                },
            )),
            BlobEnabled::False => {
                BlockContents::Block(BeaconBlock::CapellaFull(BeaconBlockCapellaFull::default()))
            }
        }
    }

    pub fn submit_blinded_block_contents(&self, signed_block_and_blobs: SignedBlockContents) {
        let serialized = serde_json::to_string(&signed_block_and_blobs).unwrap();
        println!("Block submitted to builder: {}", serialized);
    }

    pub fn submit_full_block_contents(&self, signed_block_and_blobs: SignedBlockContents) {
        let serialized = serde_json::to_string(&signed_block_and_blobs).unwrap();
        println!("Block submitted to builder: {}", serialized);
    }
}

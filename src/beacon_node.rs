use crate::enums::*;
use serde::{Deserialize, Serialize};

pub enum BlobEnabled {
    True,
    False,
}

#[derive(Serialize, Deserialize, Debug)] // encode/decode not required
#[serde(untagged)]
// FULL ONLY
pub enum BlockContents {
    Block(BeaconBlock),
    BlockAndBlobSidecar(BlockAndBlobSidecarsFull),
}

// BLINDED ONLY
pub enum BlindedBlockContents {
    Block(BlindedBeaconBlock),
    BlockAndBlobSidecar(BlockAndBlobSidecarsBlinded),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum SignedBlockContents {
    Block(Signed<BeaconBlock>),
    BlockAndBlobSidecar(SignedBlockAndBlobSidecars),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum SignedBlindedBlockContents {
    Block(Signed<BlindedBeaconBlock>),
    BlockAndBlobSidecar(SignedBlindedBlockAndBlobSidecars),
}

#[derive(Clone, Copy)]
pub struct BeaconNode {}

impl BeaconNode {
    pub fn get_blinded_block_contents(&self, blob_enabled: BlobEnabled) -> BlindedBlockContents {
        match blob_enabled {
            BlobEnabled::True => {
                BlindedBlockContents::BlockAndBlobSidecar(BlockAndBlobSidecarsBlinded {
                    blinded_block: BlindedBeaconBlock::Deneb(BlindedBeaconBlockDeneb::default()),
                    blinded_blob_sidecars: BlobSidecarsBlinded::default(),
                })
            }
            BlobEnabled::False => BlindedBlockContents::Block(BlindedBeaconBlock::Capella(
                BlindedBeaconBlockCapella::default(),
            )),
        }
    }

    pub fn get_block_contents(&self, blob_enabled: BlobEnabled) -> BlockContents {
        match blob_enabled {
            BlobEnabled::True => BlockContents::BlockAndBlobSidecar(BlockAndBlobSidecarsFull {
                block: BeaconBlock::Deneb(BeaconBlockDeneb::default()),
                blob_sidecars: BlobSidecarsFull::default(),
            }),
            BlobEnabled::False => {
                BlockContents::Block(BeaconBlock::Capella(BeaconBlockCapella::default()))
            }
        }
    }

    pub fn submit_blinded_block_contents(
        &self,
        signed_block_and_blobs: SignedBlindedBlockContents,
    ) {
        let serialized = serde_json::to_string(&signed_block_and_blobs).unwrap();
        println!("Block submitted to builder: {}", serialized);
    }

    pub fn submit_full_block_contents(&self, signed_block_and_blobs: SignedBlockContents) {
        let serialized = serde_json::to_string(&signed_block_and_blobs).unwrap();
        println!("Block submitted to builder: {}", serialized);
    }
}

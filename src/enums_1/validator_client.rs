use crate::enums_1::beacon_node::{
    BeaconNode, BlindedBlockContents, BlobEnabled, BlockContents, SignedBlindedBlockContents,
    SignedBlockContents,
};
use crate::enums_1::*;
use tree_hash::Hash256;

pub struct ValidatorClient {
    pub(crate) beacon_node: BeaconNode,
}

impl ValidatorClient {
    pub fn propose_blinded_block(&self, blob_enabled: BlobEnabled) {
        let block_contents = self.beacon_node.get_blinded_block_contents(blob_enabled);
        let signed = self.sign_blinded_block_contents(block_contents);
        self.beacon_node.submit_blinded_block_contents(signed)
    }

    pub fn propose_full_block(&self, blob_enabled: BlobEnabled) {
        let block_contents = self.beacon_node.get_block_contents(blob_enabled);
        let signed = self.sign_block_contents(block_contents);
        self.beacon_node.submit_full_block_contents(signed)
    }

    fn sign_block_contents(&self, block_contents: BlockContents) -> SignedBlockContents {
        match block_contents {
            BlockContents::Block(block) => {
                SignedBlockContents::Block(Signed::new(block, Hash256::random()))
            }
            BlockContents::BlockAndBlobSidecar(block_and_blobs) => {
                let signed_block_and_blobs = SignedBlockAndBlobSidecars {
                    signed_block: Signed::new(block_and_blobs.block, Hash256::random()),
                    signed_blob_sidecars: Self::sign(block_and_blobs.blob_sidecars),
                };
                SignedBlockContents::BlockAndBlobSidecar(signed_block_and_blobs)
            }
        }
    }

    fn sign_blinded_block_contents(
        &self,
        block_contents: BlindedBlockContents,
    ) -> SignedBlindedBlockContents {
        match block_contents {
            BlindedBlockContents::Block(block) => {
                SignedBlindedBlockContents::Block(Signed::new(block, Hash256::random()))
            }
            BlindedBlockContents::BlockAndBlobSidecar(block_and_blobs) => {
                let signed_block_and_blobs = SignedBlindedBlockAndBlobSidecars {
                    signed_blinded_block: Signed::new(
                        block_and_blobs.blinded_block,
                        Hash256::random(),
                    ),
                    signed_blinded_blob_sidecars: Self::sign(block_and_blobs.blinded_blob_sidecars),
                };
                SignedBlindedBlockContents::BlockAndBlobSidecar(signed_block_and_blobs)
            }
        }
    }

    fn sign<T>(blobs: T) -> Signed<T> {
        Signed {
            message: blobs,
            signature: Hash256::random(),
        }
    }
}

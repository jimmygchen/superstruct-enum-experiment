use crate::beacon_node::{BeaconNode, BlockContents, SignedBlockContents};
use crate::enums::*;
use crate::BlobEnabled;

pub struct ValidatorClient {
    pub(crate) beacon_node: BeaconNode,
}

impl ValidatorClient {
    pub fn propose_blinded_block(&self, blob_enabled: BlobEnabled) {
        let block_contents = self.beacon_node.get_blinded_block_contents(blob_enabled);
        let signed = self.sign_block_contents(block_contents);
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
                SignedBlockContents::Block(Signed::new(block, "block_sig".to_string()))
            }
            BlockContents::BlockAndBlobSidecar(block_and_blobs) => {
                let signed_block_and_blobs = match block_and_blobs {
                    BlockAndBlobSidecars::Full(block_and_blobs) => {
                        SignedBlockAndBlobSidecars::Full(SignedBlockAndBlobSidecarsFull {
                            signed_block: Signed::new(
                                block_and_blobs.block,
                                "block_sig".to_string(),
                            ),
                            signed_blob_sidecars: Self::sign_blob_sidecars(
                                block_and_blobs.blob_sidecars,
                            ),
                        })
                    }
                    BlockAndBlobSidecars::Blinded(block_and_blobs) => {
                        SignedBlockAndBlobSidecars::Blinded(SignedBlockAndBlobSidecarsBlinded {
                            signed_blinded_block: Signed::new(
                                block_and_blobs.blinded_block,
                                "blinded_block_sig".to_string(),
                            ),
                            signed_blinded_blob_sidecars: Self::sign_blob_sidecars(
                                block_and_blobs.blinded_blob_sidecars,
                            ),
                        })
                    }
                };
                SignedBlockContents::BlockAndBlobSidecar(signed_block_and_blobs)
            }
        }
    }

    fn sign_blob_sidecars(blobs: BlobSidecars) -> Signed<BlobSidecars> {
        Signed {
            message: blobs,
            signature: "signed_blobs".to_string(),
        }
    }
}

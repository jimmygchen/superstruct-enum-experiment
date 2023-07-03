use crate::enums_1::*;
use ssz::{Decode, Encode};
use std::fmt::Debug;

mod enums_1;
mod enums_2;

/// tests the `enums_1` mod
fn main() {
    let beacon_node = BeaconNode {};
    let validator_client = ValidatorClient { beacon_node };

    // Test serialize with & without blobs
    validator_client.propose_blinded_block(BlobEnabled::False);
    validator_client.propose_full_block(BlobEnabled::False);
    validator_client.propose_blinded_block(BlobEnabled::True);
    validator_client.propose_full_block(BlobEnabled::True);

    // Test encode / decode with & without blobs
    test_encode_then_decode_block_contents(beacon_node.get_block_contents(BlobEnabled::False));
    test_encode_then_decode_block_contents(beacon_node.get_block_contents(BlobEnabled::True));
    test_encode_then_decode_blinded_block_contents(
        beacon_node.get_blinded_block_contents(BlobEnabled::False),
    );
    test_encode_then_decode_blinded_block_contents(
        beacon_node.get_blinded_block_contents(BlobEnabled::True),
    );
}

fn test_encode_then_decode_block_contents(block_contents: BlockContents) {
    match block_contents {
        BlockContents::Block(block) => {
            map_beacon_block!(block, |inner, _| {
                test_encode_then_decode(inner).unwrap();
            });
        }
        BlockContents::BlockAndBlobSidecar(block_and_blob_sidedcars) => {
            let (block, blob_sidecars) = block_and_blob_sidedcars.deconstruct();
            map_beacon_block!(block, |inner, _| {
                test_encode_then_decode(inner).unwrap();
            });
            test_encode_then_decode(blob_sidecars).unwrap();
        }
    }
}

fn test_encode_then_decode_blinded_block_contents(block_contents: BlindedBlockContents) {
    match block_contents {
        BlindedBlockContents::Block(block) => {
            map_blinded_beacon_block!(block, |inner, _| {
                test_encode_then_decode(inner).unwrap();
            });
        }
        BlindedBlockContents::BlockAndBlobSidecar(block_and_blob_sidedcars) => {
            let (block, blob_sidecars) = block_and_blob_sidedcars.deconstruct();
            map_blinded_beacon_block!(block, |inner, _| {
                test_encode_then_decode(inner).unwrap();
            });
            test_encode_then_decode(blob_sidecars).unwrap();
        }
    }
}

fn test_encode_then_decode<T: Encode + Decode + Debug>(obj: T) -> Result<(), String> {
    let encoded = obj.as_ssz_bytes();
    let decoded = T::from_ssz_bytes(encoded.as_ref()).map_err(|e| format!("{e:?}"))?;
    dbg!(decoded);
    Ok(())
}

use crate::beacon_node::{BeaconNode, BlobEnabled};
use crate::validator_client::ValidatorClient;

mod beacon_node;
mod enums;
mod validator_client;

fn main() {
    let vc = ValidatorClient {
        beacon_node: BeaconNode {},
    };
    vc.propose_blinded_block(BlobEnabled::False);
    vc.propose_full_block(BlobEnabled::False);
    vc.propose_blinded_block(BlobEnabled::True);
    vc.propose_full_block(BlobEnabled::True);
}

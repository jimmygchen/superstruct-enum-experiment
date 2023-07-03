/// Use superstruct, but separate full & blinded fork types, tests both serialization & encoding.
mod beacon_block;
mod beacon_block_body;
mod beacon_node;
mod payload;
mod validator_client;

pub use beacon_block::*;
pub use beacon_block_body::*;
pub use beacon_node::*;
pub use payload::*;
pub use validator_client::*;

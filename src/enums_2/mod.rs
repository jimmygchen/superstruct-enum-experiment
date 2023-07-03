/// Use superstruct, but separate full & blinded fork types,
/// and added wrapper enums for sharing code across block types  
use crate::enums_2::blinded_variants::*;
use crate::enums_2::full_variants::*;

enum ProducedBlock {
    Full(BlockV2),
    Blinded(BlindedBlockV2),
}

impl ProducedBlock {
    pub fn try_into_block(&self) -> Result<&BlockV2, String> {
        match self {
            ProducedBlock::Full(b) => Ok(b),
            _ => Err("Not a full block".to_string()),
        }
    }

    // a field shared across both block types and only some forks
    pub fn deneb_field(&self) -> Result<&u64, ()> {
        match self {
            ProducedBlock::Full(inner) => inner.deneb_field(),
            ProducedBlock::Blinded(inner) => inner.deneb_field(),
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn match_and_print_debug() {
        let print = |block| match block {
            ProducedBlock::Full(b) => print_block_debug(b),
            ProducedBlock::Blinded(b) => print_blinded_block_debug(b),
        };

        let full_block = ProducedBlock::Full(BlockV2::Capella(BlockV2Capella::default()));
        print(full_block);

        let blinded_block =
            ProducedBlock::Blinded(BlindedBlockV2::Capella(BlindedBlockV2Capella::default()));
        print(blinded_block);
    }

    /// useful for code reuse where per-fork logic is required
    #[test]
    fn code_reuse_across_forks() {
        // achieved by using superstruct
        let wrapper = ProducedBlock::Full(BlockV2::Capella(BlockV2Capella::default()));
        let block = wrapper.try_into_block().unwrap();
        println!("{}", block.base_field());

        match block {
            BlockV2::Capella(_) => {
                println!("Capella!")
            }
            _ => {
                println!("Other")
            }
        }
    }

    /// useful for code reuse in block production
    #[test]
    fn code_reuse_across_block_types() {
        // common field across both blinded and non blinded type
        // needs accessor implemented on top level enum
        let block = ProducedBlock::Blinded(BlindedBlockV2::Deneb(BlindedBlockV2Deneb::default()));
        println!("{}", block.deneb_field().unwrap())
    }
}

mod full_variants {
    use superstruct::superstruct;

    #[superstruct(
        variants(Base, Altair, Merge, Capella, Deneb),
        variant_attributes(derive(Default, Debug))
    )]
    pub struct BlockV2 {
        pub base_field: u64,
        #[superstruct(only(Altair, Merge, Capella, Deneb))]
        pub altair_field: u64,
        #[superstruct(only(Merge, Capella, Deneb))]
        pub merge_field: u64,
        #[superstruct(only(Capella, Deneb))]
        pub capella_field: u64,
        #[superstruct(only(Deneb))]
        pub deneb_field: u64,

        #[superstruct(only(Merge), partial_getter(rename = "payload_merge"))]
        pub payload: PayloadV2Merge,
        #[superstruct(only(Capella), partial_getter(rename = "payload_capella"))]
        pub payload: PayloadV2Capella,
        #[superstruct(only(Deneb), partial_getter(rename = "payload_deneb"))]
        pub payload: PayloadV2Deneb,
    }

    #[superstruct(
        variants(Merge, Capella, Deneb),
        variant_attributes(derive(Default, Debug))
    )]
    pub struct PayloadV2 {
        pub merge_field: u64,
        #[superstruct(only(Capella, Deneb))]
        pub capella_field: u64,
        #[superstruct(only(Deneb))]
        pub deneb_field: u64,
    }

    pub fn print_block_debug(block: BlockV2) {
        map_block_v2!(block, |inner, _| {
            dbg!(inner);
        })
    }
}

mod blinded_variants {
    use superstruct::superstruct;

    #[superstruct(
        variants(Base, Altair, Merge, Capella, Deneb),
        variant_attributes(derive(Default, Debug))
    )]
    pub struct BlindedBlockV2 {
        pub base_field: u64,
        #[superstruct(only(Altair, Merge, Capella, Deneb))]
        pub altair_field: u64,
        #[superstruct(only(Merge, Capella, Deneb))]
        pub merge_field: u64,
        #[superstruct(only(Capella, Deneb))]
        pub capella_field_blinded: u64,
        #[superstruct(only(Deneb))]
        pub deneb_field: u64,

        #[superstruct(only(Merge), partial_getter(rename = "payload_header_merge"))]
        pub payload_header: PayloadHeaderV2Merge,
        #[superstruct(only(Capella), partial_getter(rename = "payload_header_capella"))]
        pub payload_header: PayloadHeaderV2Capella,
        #[superstruct(only(Deneb), partial_getter(rename = "payload_header_deneb"))]
        pub payload_header: PayloadHeaderV2Deneb,
    }

    #[superstruct(
        variants(Merge, Capella, Deneb),
        variant_attributes(derive(Default, Debug))
    )]
    pub struct PayloadHeaderV2 {
        pub merge_field: u64,
        #[superstruct(only(Capella, Deneb))]
        pub capella_field_blinded: u64,
        #[superstruct(only(Deneb))]
        pub deneb_field: u64,
    }

    pub fn print_blinded_block_debug(block: BlindedBlockV2) {
        map_blinded_block_v2!(block, |inner, _| {
            dbg!(inner);
        })
    }
}

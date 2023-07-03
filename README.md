# `superstruct` Enums Experiment

Experiments for using `superstruct` / enums over traits + generics for defining & handling full / blinded blocks. 
Earlier discussions in this PR: https://github.com/sigp/lighthouse/pull/4428

## Options

1. Using Generics + Traits 
   - Existing approach, blob implementation on [this branch](https://github.com/sigp/lighthouse/pull/4428)
   - Complexity in trait definitions, difficult to understand (but encapsulated in one place)
   - Acceptable readability in usage and code re-usability (_my personal opinion_)
   - Existing approach that we're already comfortable with, need to consider if the switch to other option is worth it at this stage.
2. Enums with `superstruct`
   - Implementation in [the `enums_2` directory](/src/enums_2), a version with encoding / decoding included under [`enums_1`](/src/enums_1)
   - Separate `superstructs` for blinded types, and add custom enum wrappers over both `superstruct` types
   - Pros:
     - Simplicity and ease of understanding, no complicated traits required
     - Simplified function signatures, no more generics type params, just enums as param / return types
     - Possible to write code in a generic way
   - Cons:
     - Some code duplication required, could be a problem if we introduce a 3rd block type (a "half-blinded" block)
     - Any generic code for full/blinded block will need [custom implementation](/src/enums_2/mod.rs) on the top level enum
3. `superduperstruct`
   - Multi level variants (fork version, blinded block types), with ability to match on either dimensions
   - Basically automating #2, and reducing boilerplate
   - How it could look like:
     - field definition: `only(Blinded(only(Capella)))`
     - enum: `enum ProducedBeaconBlock { Blinded(BlindedBeaconBlock), Full(FullBeaconBlock) }`
       - where `BlindedBeaconBlock` and `FullBeaconBlock` are `superstruct`s with fork variants
       - I think having the hierarchy in this order: `block_type -> fork` would make it easier for us, because
         - having `block_type` on the top level makes block production code more generic without having to match on each fork, and then block types
         - most of the time when we are doing per-fork logic, we already know the block type, so we can unwrap the outer `block_type` and just operate on the fork versioned block enum
   - library _may_ become more specialised and complex, `superstruct` definitions could get messy because we need a 
     `payload` field for each variant on the beacon block

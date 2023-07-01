# Superstruct for blinded flow experiment

## Learnings

- Pass around using the enum type instead of the inner type, this allows writing code in a generic way 
- Extra boilerplate code but less complexity than traits & generics
- It might be worth keeping Blinded & Full types completely separate, since we don't use blinded objects for consensus,
  p2p or storage, we only really use them for block production and beacon apis. The main shared code is block production,
  which could be a bit of a pain if we need to can't reuse it for both.

## Advantages

- Simplified function signatures, no more generics type params, just enums as param / return types
- It is possible to write code in a generic way
- No complicated traits required

## Disadvantages

- The 2D superstructs definition could get a bit messy, because we now need to double the variant types when defining fields
- Constructing the top level object not as straight forward as without enums
- Could be more error-prone because it means any variants can be passed around and can't restrict which variants are used

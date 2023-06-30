# Superstruct for blinded flow experiment

## Learnings

- Pass around using the enum type instead of the inner type, this allows writing code in a generic way 
- Extra boilerplate code but less complexity than traits & generics

## Advantages

- Simplified function signatures, no more generics type params, just enums as param / return types
- It is possible to write code in a generic way
- No complicated traits required

## Disadvantages

- The 2D superstructs definition could get a bit messy, because we now need to double the variant types when defining fields
- Constructing the top level object not as straight forward as without enums 

## TODO

- Encoding and decoding SSZ
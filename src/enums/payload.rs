use serde::{Deserialize, Serialize};
use superstruct::superstruct;

#[superstruct(
    variants(
        BellatrixFull,
        BellatrixBlinded,
        CapellaFull,
        CapellaBlinded,
        DenebFull,
        DenebBlinded,
    ),
    variant_attributes(derive(Debug, Serialize, Deserialize, Default),)
)]
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub struct Payload {
    #[superstruct(only(BellatrixFull, CapellaFull, DenebFull))]
    pub transactions: Vec<String>,
    #[superstruct(only(BellatrixBlinded, CapellaBlinded, DenebBlinded))]
    pub transaction_roots: Vec<String>,
    #[superstruct(only(CapellaFull, DenebFull))]
    pub withdrawals: Vec<String>,
    #[superstruct(only(CapellaBlinded, DenebBlinded))]
    pub withdrawals_roots: Vec<String>,
    #[superstruct(only(DenebFull, DenebBlinded))]
    pub excess_data_gas: Vec<String>,
}

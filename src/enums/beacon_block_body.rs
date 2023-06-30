use super::payload::*;
use serde::{Deserialize, Serialize};
use superstruct::superstruct;

#[superstruct(
    variants(
        BellatrixBlinded,
        BellatrixFull,
        CapellaBlinded,
        CapellaFull,
        DenebBlinded,
        DenebFull
    ),
    variant_attributes(derive(Debug, Serialize, Deserialize, Default),)
)]
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub struct BeaconBlockBody {
    pub randao_reveal: String,
    #[superstruct(
        only(BellatrixFull),
        partial_getter(rename = "execution_payload_bellatrix_full")
    )]
    pub execution_payload: PayloadBellatrixFull,
    #[superstruct(
        only(BellatrixBlinded),
        partial_getter(rename = "execution_payload_bellatrix_blinded")
    )]
    pub execution_payload_header: PayloadBellatrixBlinded,
    #[superstruct(
        only(CapellaFull),
        partial_getter(rename = "execution_payload_capella_full")
    )]
    pub execution_payload: PayloadCapellaFull,
    #[superstruct(
        only(CapellaBlinded),
        partial_getter(rename = "execution_payload_capella_blinded")
    )]
    pub execution_payload_header: PayloadCapellaBlinded,
    #[superstruct(
        only(DenebFull),
        partial_getter(rename = "execution_payload_deneb_full")
    )]
    pub execution_payload: PayloadDenebFull,
    #[superstruct(
        only(DenebBlinded),
        partial_getter(rename = "execution_payload_deneb_blinded")
    )]
    pub execution_payload_header: PayloadDenebBlinded,
    #[superstruct(only(CapellaFull, CapellaBlinded, DenebFull, DenebBlinded))]
    pub bls_to_execution_changes: String,
    #[superstruct(only(DenebFull, DenebBlinded))]
    pub kzg_commitments: Vec<String>,
}

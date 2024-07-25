use models::{R1CSFile, SnarkJsWitnessFile, SnarkjsZkeyFile};
use std::path::PathBuf;

use crate::models;

pub fn deserialize_zkey_json(
    input: &PathBuf,
) -> Result<SnarkjsZkeyFile, Box<dyn std::error::Error>> {
    let input_data = std::fs::read_to_string(input)?;
    let zkey_file: SnarkjsZkeyFile = serde_json::from_str(&input_data)?;
    Ok(zkey_file)
}

/// Deserializes the witness file
pub fn deserialize_witness_json(
    input: &PathBuf,
) -> Result<SnarkJsWitnessFile, Box<dyn std::error::Error>> {
    let input_data = std::fs::read_to_string(input)?;
    let witness_file: SnarkJsWitnessFile = serde_json::from_str(&input_data)?;
    Ok(witness_file)
}

pub fn deserialize_r1cs_json(input: &PathBuf) -> Result<R1CSFile, Box<dyn std::error::Error>> {
    let input_data = std::fs::read_to_string(input)?;
    let r1cs_file: R1CSFile = serde_json::from_str(&input_data)?;
    Ok(r1cs_file)
}

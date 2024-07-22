use models::SnarkjsZkeyFile;
use std::path::PathBuf;

use crate::models;

pub fn deserialize_zkey_file(
    input: &PathBuf,
) -> Result<SnarkjsZkeyFile, Box<dyn std::error::Error>> {
    let input_data = std::fs::read_to_string(input)?;
    let zkey_file: SnarkjsZkeyFile = serde_json::from_str(&input_data)?;
    Ok(zkey_file)
}

pub fn deserialize_witness_file(
    input: &PathBuf,
) -> Result<models::SnarkJsWitnessFile, Box<dyn std::error::Error>> {
    let input_data = std::fs::read_to_string(input)?;
    let witness_file: models::SnarkJsWitnessFile = serde_json::from_str(&input_data)?;
    Ok(witness_file)
}

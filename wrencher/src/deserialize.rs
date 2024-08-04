use models::{R1CSFile, SnarkJsWitnessFile};
use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::models;

/// Deserializes the witness file
pub fn deserialize_witnesses_json(
    input_dir: &Path,
) -> Result<Vec<SnarkJsWitnessFile>, Box<dyn std::error::Error>> {
    let mut witness_files = Vec::new();

    if input_dir.is_dir() {
        for entry in fs::read_dir(input_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file()
                && path.extension() == Some(std::ffi::OsStr::new("json"))
                && path
                    .file_name()
                    .and_then(|s| s.to_str())
                    .map_or(false, |s| s.ends_with(".wtns.json"))
            {
                let input_data = fs::read_to_string(&path)?;
                let witness_file: SnarkJsWitnessFile = serde_json::from_str(&input_data)?;
                witness_files.push(witness_file);
            }
        }
    } else {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Input path is not a directory",
        )));
    }

    Ok(witness_files)
}

pub fn deserialize_r1cs_json(input: &PathBuf) -> Result<R1CSFile, Box<dyn std::error::Error>> {
    let input_data = std::fs::read_to_string(input)?;
    let r1cs_file: R1CSFile = serde_json::from_str(&input_data)?;
    Ok(r1cs_file)
}

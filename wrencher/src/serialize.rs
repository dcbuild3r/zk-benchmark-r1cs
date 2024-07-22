use crate::models::{Matrix, SnarkJsWitnessFile, SnarkjsZkeyFile};
use serde::Serialize;
use std::path::PathBuf;

#[allow(unused)]
pub fn serialize_zkey_file(
    data: &SnarkjsZkeyFile,
    output: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let output_data = serde_json::to_string_pretty(data)?;
    std::fs::write(output, output_data)?;
    Ok(())
}

#[derive(Serialize, Debug)]
pub struct SerializedSnarkJs<'a> {
    pub num_public: usize,
    pub num_variables: usize,
    pub num_constraints: usize,
    pub a: Vec<Matrix>,
    pub b: Vec<Matrix>,
    pub c: Vec<Matrix>,
    #[serde(flatten)]
    pub witness: &'a SnarkJsWitnessFile,
}

pub fn convert_to_serialize_format<'a>(
    zkey: &'a SnarkjsZkeyFile,
    witness: &'a SnarkJsWitnessFile,
) -> SerializedSnarkJs<'a> {
    let mut a = Vec::new();
    let mut b = Vec::new();
    let mut c = Vec::new();

    for coef in &zkey.coefficients {
        let entry = Matrix {
            constraint: coef.data.constraint,
            signal: coef.data.signal,
            value: coef.data.value.clone(),
        };

        match coef.matrix {
            0 => a.push(entry),
            1 => b.push(entry),
            2 => c.push(entry),
            _ => {} // Ignore any other values
        }
    }

    SerializedSnarkJs {
        num_public: zkey.num_public,
        num_variables: zkey.num_variables,
        num_constraints: a.len(),
        a,
        b,
        c,
        witness,
    }
}

pub fn serialize_snarkjs(
    serialized: &SerializedSnarkJs,
    output: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let output_data = serde_json::to_string_pretty(&serialized)?;
    std::fs::write(output, output_data)?;
    Ok(())
}

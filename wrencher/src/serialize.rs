use crate::models::{Matrix, SnarkjsZkeyFile};
use serde::Serialize;
use std::path::PathBuf;

#[derive(Serialize, Debug)]
pub struct SerializeZkeyFile {
    pub num_public: usize,
    pub num_variables: usize,
    pub num_constraints: usize,
    pub A: Vec<Matrix>,
    pub B: Vec<Matrix>,
    pub C: Vec<Matrix>,
}

pub fn convert_to_serialize_format(input: &SnarkjsZkeyFile) -> SerializeZkeyFile {
    let mut a = Vec::new();
    let mut b = Vec::new();
    let mut c = Vec::new();

    for coef in &input.coefficients {
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

    SerializeZkeyFile {
        num_public: input.num_public,
        num_variables: input.num_variables,
        num_constraints: a.len(),
        A: a,
        B: b,
        C: c,
    }
}

pub fn serialize_zkey_file(
    data: &SnarkjsZkeyFile,
    output: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let serialize_data = convert_to_serialize_format(data);
    let output_data = serde_json::to_string_pretty(&serialize_data)?;
    std::fs::write(output, output_data)?;
    Ok(())
}

use models::SnarkjsZkeyFile;
use serialize::SerializeZkeyFile;
use std::path::PathBuf;

use crate::models;
use crate::models::Matrix;
use crate::serialize;

pub fn deserialize_zkey_file(
    input: &PathBuf,
) -> Result<SnarkjsZkeyFile, Box<dyn std::error::Error>> {
    let input_data = std::fs::read_to_string(input)?;
    let zkey_file: SnarkjsZkeyFile = serde_json::from_str(&input_data)?;
    Ok(zkey_file)
}

pub fn serialize_zkey_file(
    data: &SnarkjsZkeyFile,
    output: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let output_data = serde_json::to_string_pretty(data)?;
    std::fs::write(output, output_data)?;
    Ok(())
}

impl SnarkjsZkeyFile {
    pub fn to_serialize_format(&self) -> SerializeZkeyFile {
        let mut a = Vec::new();
        let mut b = Vec::new();
        let mut c = Vec::new();

        for coef in &self.coefficients {
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
            num_public: self.num_public,
            num_variables: self.num_variables,
            num_constraints: a.len(), // Assuming the number of constraints is the same as the number of entries in A
            A: a,
            B: b,
            C: c,
        }
    }
}

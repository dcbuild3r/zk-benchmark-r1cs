use crate::models::{Matrix, R1CSFile, SnarkJsWitnessFile, SnarkjsZkeyFile};
use serde::Serialize;
use std::{collections::HashMap, path::PathBuf};

/// Serialized output format for wrencher to use with the benchmarking tool
///
/// It contains the number of public inputs, variables, constraints and the a, b, c matrices as well as the witness values.
#[derive(Serialize, Debug)]
pub struct SerializedSnarkJs<'a> {
    pub num_public: usize,
    pub num_variables: usize,
    pub num_constraints: usize,
    pub a: Vec<Matrix>,
    pub b: Vec<Matrix>,
    pub c: Vec<Matrix>,
    pub witness: &'a SnarkJsWitnessFile,
}

/// Serializes the SnarkJS zkey and witness files to a format that can be used by the wrencher library
pub fn convert_zkey_witness_to_serialize_format<'a>(
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

/// Converts the R1CS and witness files to a serialized format that can be understood by the benchmarking tool
pub fn convert_r1cs_witness_to_serialize_format<'a>(
    r1cs: &R1CSFile,
    witness: &'a SnarkJsWitnessFile,
) -> SerializedSnarkJs<'a> {
    let mut a = Vec::new();
    let mut b = Vec::new();
    let mut c = Vec::new();

    for (constraint_idx, constraint) in r1cs.constraints.iter().enumerate() {
        process_constraint(&mut a, &constraint[0], constraint_idx);
        process_constraint(&mut b, &constraint[1], constraint_idx);
        process_constraint(&mut c, &constraint[2], constraint_idx);
    }

    SerializedSnarkJs {
        num_variables: r1cs.num_variables,
        num_public: r1cs.num_pub_inputs + r1cs.num_outputs,
        num_constraints: r1cs.num_constraints,
        a,
        b,
        c,
        witness,
    }
}

/// Processes the R1CS constraints, separates a, b and c coefficients and adds them to the corresponding vector
fn process_constraint(
    matrix: &mut Vec<Matrix>,
    constraint: &HashMap<String, String>,
    constraint_idx: usize,
) {
    for (signal, value) in constraint {
        matrix.push(Matrix {
            constraint: constraint_idx,
            signal: signal.parse().unwrap_or(0),
            value: value.clone(),
        });
    }
}

/// Serializes the new SnarkJS format to a JSON file
pub fn serialize_snarkjs(
    serialized: &SerializedSnarkJs,
    output: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let output_data = serde_json::to_string_pretty(&serialized)?;
    std::fs::write(output, output_data)?;
    Ok(())
}

/// Serializes the new R1CS format to a JSON file
#[allow(unused)]
pub fn serialize_r1cs(
    serialized: &R1CSFile,
    output: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let output_data = serde_json::to_string_pretty(&serialized)?;
    std::fs::write(output, output_data)?;
    Ok(())
}

/// Serializes the SnarkJS zkey file to a JSON file
#[allow(unused)]
pub fn serialize_zkey_file(
    data: &SnarkjsZkeyFile,
    output: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let output_data = serde_json::to_string_pretty(data)?;
    std::fs::write(output, output_data)?;
    Ok(())
}
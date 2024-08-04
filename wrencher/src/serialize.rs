use crate::models::{Matrix, R1CSFile, SnarkJsWitnessFile};
use crate::r1cs::{
    check_r1cs_satisfiability, compute_witness_bound_constraints, WitnessBoundConstraints,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

/// Serialized output format for wrencher to use with the benchmarking tool
///
/// It contains the number of public inputs, variables, constraints and the a, b, c matrices as well as the witness values.
#[derive(Deserialize, Serialize, Debug)]
pub struct SerializedSnarkJs {
    pub constraints: Constraints,
    pub witnesses: Vec<SnarkJsWitnessFile>,
    pub constrained_witnesses: Vec<WitnessBoundConstraints>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Constraints {
    pub num_public: usize,
    pub num_variables: usize,
    pub num_constraints: usize,
    pub a: Vec<Matrix>,
    pub b: Vec<Matrix>,
    pub c: Vec<Matrix>,
}

/// Converts an R1CS file with several witness files to a serialized format that can be understood by the benchmarking tool
pub fn convert_r1cs_witnesses_to_serialize_format(
    r1cs: &R1CSFile,
    witnesses: Vec<SnarkJsWitnessFile>,
) -> SerializedSnarkJs {
    let mut a = Vec::new();
    let mut b = Vec::new();
    let mut c = Vec::new();

    for (constraint_idx, constraint) in r1cs.constraints.iter().enumerate() {
        process_constraint(&mut a, &constraint[0], constraint_idx);
        process_constraint(&mut b, &constraint[1], constraint_idx);
        process_constraint(&mut c, &constraint[2], constraint_idx);
    }

    let constraints = Constraints {
        num_public: r1cs.num_pub_inputs + r1cs.num_outputs,
        num_variables: r1cs.num_variables,
        num_constraints: r1cs.num_constraints,
        a,
        b,
        c,
    };

    let constrained_witnesses = witnesses
        .iter()
        .map(|witness| compute_witness_bound_constraints(&constraints, witness))
        .collect::<Vec<_>>();

    assert!(
        check_r1cs_satisfiability(&constrained_witnesses),
        "r1cs constraints are not satisfied"
    );

    SerializedSnarkJs {
        constraints,
        witnesses,
        constrained_witnesses,
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

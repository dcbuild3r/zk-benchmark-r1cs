use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Updated witness file structure
pub type SnarkJsWitnessFile = Vec<String>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Matrix {
    pub constraint: usize,
    pub signal: usize,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct R1CSFile {
    pub n8: u32,
    pub prime: String,
    #[serde(rename = "nVars")]
    pub num_variables: usize,
    #[serde(rename = "nOutputs")]
    pub num_outputs: usize,
    #[serde(rename = "nPubInputs")]
    pub num_pub_inputs: usize,
    #[serde(rename = "nPrvInputs")]
    pub num_prv_inputs: usize,
    #[serde(rename = "nLabels")]
    pub num_labels: usize,
    #[serde(rename = "nConstraints")]
    pub num_constraints: usize,
    #[serde(rename = "useCustomGates")]
    pub use_custom_gates: Option<bool>,
    pub constraints: Vec<Vec<HashMap<String, String>>>,
    pub map: Vec<usize>,
}

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct SnarkjsZkeyFile {
    #[serde(rename = "nPublic")]
    pub num_public: usize,

    #[serde(rename = "nVars")]
    pub num_variables: usize,

    #[serde(rename = "ccoefs")]
    pub coefficients: Vec<ZkeyCoefficients>,
}

// Updated witness file structure
pub type SnarkJsWitnessFile = Vec<String>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Matrix {
    pub constraint: usize,
    pub signal: usize,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ZkeyCoefficients {
    pub matrix: usize,
    #[serde(flatten)]
    pub data: Matrix,
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
    pub use_custom_gates: bool,
    pub constraints: Vec<Vec<HashMap<String, String>>>,
    pub map: Vec<usize>,
}

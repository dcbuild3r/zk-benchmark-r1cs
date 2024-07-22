use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SnarkjsZkeyFile {
    #[serde(rename = "nPublic")]
    pub num_public: usize,

    #[serde(rename = "nVars")]
    pub num_variables: usize,

    #[serde(rename = "ccoefs")]
    pub coefficients: Vec<ZkeyCoefficients>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SnarkJsWitnessFile {
    pub witness: Vec<String>,
}

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

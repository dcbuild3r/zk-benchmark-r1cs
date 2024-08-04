use std::{collections::HashMap, str::FromStr};

use ruint::{aliases::U256, uint, Uint};
use serde::{Deserialize, Serialize};

use crate::{
    models::{Matrix, SnarkJsWitnessFile},
    serialize::Constraints,
};

/// An element of the BN254 scalar field Fr.
///
/// Represented as a big-endian byte vector without Montgomery reduction.
// TODO: Make sure value is always reduced.
pub type FieldElement = U256;

// See <https://docs.rs/ark-bn254/latest/ark_bn254>
pub const MODULUS: FieldElement =
    uint!(21888242871839275222246405745257275088548364400416034343698204186575808495617_U256);

#[derive(Deserialize, Serialize, Debug)]
pub struct WitnessBoundConstraints {
    pub az: Vec<FieldElement>,
    pub bz: Vec<FieldElement>,
    pub cz: Vec<FieldElement>,
}

pub fn compute_witness_bound_constraints(
    constraints: &Constraints,
    witness: &SnarkJsWitnessFile,
) -> WitnessBoundConstraints {
    let mut a_map: HashMap<usize, HashMap<usize, FieldElement>> = HashMap::new();
    let mut b_map: HashMap<usize, HashMap<usize, FieldElement>> = HashMap::new();
    let mut c_map: HashMap<usize, HashMap<usize, FieldElement>> = HashMap::new();

    // Convert witness to FieldElement
    let witness: Vec<FieldElement> = witness.iter().map(|w| Uint::from_str(w).unwrap()).collect();

    // Function to populate the hashmaps
    let populate_map = |map: &mut HashMap<usize, HashMap<usize, FieldElement>>,
                        matrix: &[Matrix]| {
        for m in matrix {
            map.entry(m.constraint)
                .or_default()
                .insert(m.signal, Uint::from_str(&m.value).unwrap());
        }
    };

    // Populate the hashmaps
    populate_map(&mut a_map, &constraints.a);
    populate_map(&mut b_map, &constraints.b);
    populate_map(&mut c_map, &constraints.c);

    // Function to compute dot product
    let dot_product =
        |map: &HashMap<usize, FieldElement>, witness: &[FieldElement]| -> FieldElement {
            map.iter()
                .map(|(&signal, value)| value.mul_mod(witness[signal], MODULUS))
                .fold(FieldElement::ZERO, |acc, x| acc.add_mod(x, MODULUS))
        };

    let mut az: Vec<Uint<256, 4>> = Vec::new();
    let mut bz: Vec<Uint<256, 4>> = Vec::new();
    let mut cz: Vec<Uint<256, 4>> = Vec::new();

    // Check each constraint
    for i in 0..constraints.num_constraints {
        let a_result = dot_product(a_map.get(&i).unwrap_or(&HashMap::new()), &witness);
        az.push(a_result);

        let b_result = dot_product(b_map.get(&i).unwrap_or(&HashMap::new()), &witness);
        bz.push(b_result);

        let c_result = dot_product(c_map.get(&i).unwrap_or(&HashMap::new()), &witness);
        cz.push(c_result);
    }

    WitnessBoundConstraints { az, bz, cz }
}

pub fn check_r1cs_satisfiability_single(
    constrained_witness: &WitnessBoundConstraints,
) -> (bool, WitnessBoundConstraints) {
    let (az, bz, cz) = (
        constrained_witness.az.clone(),
        constrained_witness.bz.clone(),
        constrained_witness.cz.clone(),
    );

    for i in 0..az.len() {
        let (a, b, c) = (az[i], bz[i], cz[i]);
        if a.mul_mod(b, MODULUS) != c {
            println!("Constraint {} is not satisfied:", i);
            println!("LHS (A * witness * B * witness): {}", a * b);
            println!("RHS (C * witness): {}", c);
            return (false, WitnessBoundConstraints { az, bz, cz });
        }
    }

    (true, WitnessBoundConstraints { az, bz, cz })
}

pub fn check_r1cs_satisfiability(constrained_witnesses: &[WitnessBoundConstraints]) -> bool {
    for constrained_witness in constrained_witnesses.iter() {
        let (satisfied, _) = check_r1cs_satisfiability_single(constrained_witness);
        if !satisfied {
            return false;
        }
    }

    println!("All constraints are satisfied!");
    true
}

#[cfg(test)]
mod tests {
    use crate::{models::SnarkJsWitnessFile, serialize::Constraints};

    use serde::Deserialize;

    #[test]
    fn test_r1cs_satisfiability() {
        // 3 * 5 = 15
        let data = r#"
        {
            "constraints": {
                "num_public" : 2,
                "num_variables" : 2,
                "num_constraints": 1,
                "a": [
                    {
                        "constraint": 0,
                        "signal": 1,
                        "value": "1"
                    }
                ],
                "b": [
                    {
                        "constraint": 0,
                        "signal": 2,
                        "value": "1"
                    }
                ],
                "c": [
                    {
                        "constraint": 0,
                        "signal": 3,
                        "value": "1"
                    }
                ]
            }, 
            "witnesses": [
                ["1", "3", "5", "15"],
                ["1", "4", "6", "24"],
                ["1", "5", "7", "35"],
                ["1", "1000", "1000", "1000000"],
                ["1", "420", "69", "28980"]
            ]
        }
        "#;

        #[derive(Deserialize)]
        struct ConstraintsWithWitnesses {
            constraints: Constraints,
            witnesses: Vec<SnarkJsWitnessFile>,
        }

        let data: ConstraintsWithWitnesses = serde_json::from_str(data).unwrap();
        let constraints = &data.constraints;
        let witnesses = &data.witnesses;
        let constrained_witnesses = witnesses
            .iter()
            .map(|witness| super::compute_witness_bound_constraints(constraints, witness))
            .collect::<Vec<_>>();
        assert!(super::check_r1cs_satisfiability(&constrained_witnesses));
    }
}

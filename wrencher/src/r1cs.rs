use std::{collections::HashMap, str::FromStr};

use ruint::{aliases::U256, uint, Uint};

use crate::{models::Matrix, serialize::SerializedSnarkJs};

/// An element of the BN254 scalar field Fr.
///
/// Represented as a big-endian byte vector without Montgomery reduction.
// TODO: Make sure value is always reduced.
pub type FieldElement = U256;

// See <https://docs.rs/ark-bn254/latest/ark_bn254>
pub const MODULUS: FieldElement =
    uint!(21888242871839275222246405745257275088548364400416034343698204186575808495617_U256);

pub fn check_r1cs_satisfiability(data: &SerializedSnarkJs) -> bool {
    let witness = &data.witnesses[0]; // Assuming we're checking the first witness

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
    populate_map(&mut a_map, &data.a);
    populate_map(&mut b_map, &data.b);
    populate_map(&mut c_map, &data.c);

    // Function to compute dot product
    let dot_product =
        |map: &HashMap<usize, FieldElement>, witness: &[FieldElement]| -> FieldElement {
            map.iter()
                .map(|(&signal, value)| value.mul_mod(witness[signal], MODULUS))
                .fold(FieldElement::ZERO, |acc, x| acc.add_mod(x, MODULUS))
        };

    // Check each constraint
    for i in 0..data.num_constraints {
        let a_result = dot_product(a_map.get(&i).unwrap_or(&HashMap::new()), &witness);
        let b_result = dot_product(b_map.get(&i).unwrap_or(&HashMap::new()), &witness);
        let c_result = dot_product(c_map.get(&i).unwrap_or(&HashMap::new()), &witness);

        if a_result.mul_mod(b_result, MODULUS) != c_result {
            println!("Constraint {} is not satisfied:", i);
            println!("LHS (A * witness * B * witness): {}", a_result * b_result);
            println!("RHS (C * witness): {}", c_result);
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_r1cs_satisfiability() {
        // 3 * 5 = 15
        let data = r#"
        {
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
            ],
            "witnesses": [
                ["1", "3", "5", "15"]
            ]
        }
        "#;

        let data: super::SerializedSnarkJs = serde_json::from_str(data).unwrap();
        assert_eq!(super::check_r1cs_satisfiability(&data), true);
    }
}

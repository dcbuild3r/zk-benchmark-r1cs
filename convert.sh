#!/bin/bash

# first, create a dir named <CIRCUIT_NAME> and add inputs.json and <CIRCUIT_NAME>.circom to it
CIRCUIT_NAME="multiplier"
DESCRIPTION='"Multiplies two numbers."'

circom ${CIRCUIT_NAME}/${CIRCUIT_NAME}.circom --r1cs --O1 --wasm --output ${CIRCUIT_NAME}

snarkjs r1cs export json ${CIRCUIT_NAME}/${CIRCUIT_NAME}.r1cs ${CIRCUIT_NAME}/${CIRCUIT_NAME}.r1cs.json

node ${CIRCUIT_NAME}/${CIRCUIT_NAME}_js/generate_witness.js ${CIRCUIT_NAME}/${CIRCUIT_NAME}_js/${CIRCUIT_NAME}.wasm ${CIRCUIT_NAME}/inputs.json ${CIRCUIT_NAME}/witness.wtns

snarkjs wtns export json ${CIRCUIT_NAME}/witness.wtns ${CIRCUIT_NAME}/witness.wtns.json

echo '{
  "description": '"${DESCRIPTION}"',
  "r1cs": '"$(cat ${CIRCUIT_NAME}/${CIRCUIT_NAME}.r1cs.json)"',
  "witness": '"$(cat ${CIRCUIT_NAME}/witness.wtns.json)"'
}' > ${CIRCUIT_NAME}/combined.json

rm -rf ${CIRCUIT_NAME}/${CIRCUIT_NAME}_js
rm -rf ${CIRCUIT_NAME}/${CIRCUIT_NAME}.r1cs
rm -rf ${CIRCUIT_NAME}/${CIRCUIT_NAME}.r1cs.json
rm -rf ${CIRCUIT_NAME}/witness.wtns
rm -rf ${CIRCUIT_NAME}/witness.wtns.json
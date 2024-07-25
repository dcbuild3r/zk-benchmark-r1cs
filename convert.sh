#!/bin/bash

# Function to check if a command is available
check_command() {
    if ! command -v "$1" &> /dev/null; then
        echo "Error: $1 is not installed or not in the PATH."
        return 1
    fi
    return 0
}

# Check for required commands
required_commands=("circom" "snarkjs" "node" "cargo")
missing_commands=()

for cmd in "${required_commands[@]}"; do
    if ! check_command "$cmd"; then
        missing_commands+=("$cmd")
    fi
done

# If any commands are missing, exit with an error
if [ ${#missing_commands[@]} -ne 0 ]; then
    echo "The following required commands are missing:"
    printf -- "- %s\n" "${missing_commands[@]}"
    echo "Please install the missing commands and try again."
    exit 1
fi

echo "All required commands are installed. Proceeding with the script..."


# first, create a dir named <CIRCUIT_NAME> and add inputs.json and <CIRCUIT_NAME>.circom to it
CIRCUIT_NAME="multiplier"
DESCRIPTION='"Multiplies two numbers."'

circom ${CIRCUIT_NAME}/${CIRCUIT_NAME}.circom -l ${CIRCUIT_NAME}/node_modules --r1cs --O1 --wasm --output ${CIRCUIT_NAME}

snarkjs r1cs export json ${CIRCUIT_NAME}/${CIRCUIT_NAME}.r1cs ${CIRCUIT_NAME}/${CIRCUIT_NAME}.r1cs.json

node ${CIRCUIT_NAME}/${CIRCUIT_NAME}_js/generate_witness.js ${CIRCUIT_NAME}/${CIRCUIT_NAME}_js/${CIRCUIT_NAME}.wasm ${CIRCUIT_NAME}/inputs.json ${CIRCUIT_NAME}/witness.wtns

snarkjs wtns export json ${CIRCUIT_NAME}/witness.wtns ${CIRCUIT_NAME}/witness.wtns.json

cd wrencher

cargo run --release -- ser-r1cs --r1cs-path ../${CIRCUIT_NAME}/${CIRCUIT_NAME}.r1cs.json --witness-path ../${CIRCUIT_NAME}/witness.wtns.json --output ../${CIRCUIT_NAME}/${CIRCUIT_NAME}_wrencher.json

rm -rf ../${CIRCUIT_NAME}/witness.wtns.json
rm -rf ../${CIRCUIT_NAME}/${CIRCUIT_NAME}.r1cs.json
rm -rf ../${CIRCUIT_NAME}/${CIRCUIT_NAME}.r1cs
rm -rf ../${CIRCUIT_NAME}/${CIRCUIT_NAME}_js
rm -rf ../${CIRCUIT_NAME}/witness.wtns
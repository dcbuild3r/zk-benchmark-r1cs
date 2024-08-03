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

build_and_serialize_circuit() {
    circom ${CIRCUIT_NAME}/${CIRCUIT_NAME}.circom -l ${CIRCUIT_NAME}/node_modules --r1cs --O1 --wasm --output ${CIRCUIT_NAME}

    snarkjs r1cs export json ${CIRCUIT_NAME}/${CIRCUIT_NAME}.r1cs ${CIRCUIT_NAME}/${CIRCUIT_NAME}.r1cs.json

    node ${CIRCUIT_NAME}/${CIRCUIT_NAME}_js/generate_witness.js ${CIRCUIT_NAME}/${CIRCUIT_NAME}_js/${CIRCUIT_NAME}.wasm ${CIRCUIT_NAME}/inputs.json ${CIRCUIT_NAME}/witness.wtns

    snarkjs wtns export json ${CIRCUIT_NAME}/witness.wtns ${CIRCUIT_NAME}/witness.wtns.json

    if [[ "${CIRCUIT_NAME}" == "multiplier" ]]; then
        if [ ! -e ${CIRCUIT_NAME}/powersOfTau28_hez_final_11.ptau ]; then
            curl -o ${CIRCUIT_NAME}/powersOfTau28_hez_final_11.ptau https://storage.googleapis.com/zkevm/ptau/powersOfTau28_hez_final_11.ptau
        fi

        snarkjs groth16 setup ${CIRCUIT_NAME}/${CIRCUIT_NAME}.r1cs ${CIRCUIT_NAME}/powersOfTau28_hez_final_11.ptau ${CIRCUIT_NAME}/${CIRCUIT_NAME}.zkey

    elif [[ "${CIRCUIT_NAME}" == "rsa" ]]; then
        if [ ! -e ${CIRCUIT_NAME}/powersOfTau28_hez_final_18.ptau ]; then
            curl -o ${CIRCUIT_NAME}/powersOfTau28_hez_final_18.ptau https://storage.googleapis.com/zkevm/ptau/powersOfTau28_hez_final_18.ptau
        fi 

        snarkjs groth16 setup ${CIRCUIT_NAME}/${CIRCUIT_NAME}.r1cs ${CIRCUIT_NAME}/powersOfTau28_hez_final_18.ptau ${CIRCUIT_NAME}/${CIRCUIT_NAME}.zkey
    fi

    snarkjs zkey export json ${CIRCUIT_NAME}/${CIRCUIT_NAME}.zkey ${CIRCUIT_NAME}/${CIRCUIT_NAME}.zkey.json

    cd ../

    cargo run --release -- ser-r1cs --r1cs-path ./tests/${CIRCUIT_NAME}/${CIRCUIT_NAME}.r1cs.json --witness-dir ./tests/${CIRCUIT_NAME}/ --output ./tests/${CIRCUIT_NAME}/${CIRCUIT_NAME}_wrencher_r1cs.json

    cargo run --release -- ser-zkey --zkey-path ./tests/${CIRCUIT_NAME}/${CIRCUIT_NAME}.zkey.json --witness-dir ./tests/${CIRCUIT_NAME}/ --output ./tests/${CIRCUIT_NAME}/${CIRCUIT_NAME}_wrencher_zkey.json

    echo "Circuit ${CIRCUIT_NAME}: (${DESCRIPTION}) built and serialized successfully."

    if cmp -s "./tests/${CIRCUIT_NAME}/${CIRCUIT_NAME}_wrencher_r1cs.json" "./tests/${CIRCUIT_NAME}/${CIRCUIT_NAME}_wrencher_zkey.json"; then
        echo "The zkey and r1cs serializations are identical."
    else
        echo "error: The zkey and r1cs serializations are different."
    fi

    rm -rf ./tests/${CIRCUIT_NAME}/${CIRCUIT_NAME}.zkey.json
    rm -rf ./tests/${CIRCUIT_NAME}/${CIRCUIT_NAME}.zkey
    rm -rf ./tests/${CIRCUIT_NAME}/witness.wtns.json
    rm -rf ./tests/${CIRCUIT_NAME}/${CIRCUIT_NAME}.r1cs.json
    rm -rf ./tests/${CIRCUIT_NAME}/${CIRCUIT_NAME}.r1cs
    rm -rf ./tests/${CIRCUIT_NAME}/${CIRCUIT_NAME}_js
    rm -rf ./tests/${CIRCUIT_NAME}/witness.wtns
}

CIRCUIT_NAME="rsa"
DESCRIPTION="RSA circuit"

build_and_serialize_circuit $CIRCUIT_NAME $DESCRIPTION

cd tests

CIRCUIT_NAME="multiplier"
DESCRIPTION="Multiplier circuit"

build_and_serialize_circuit $CIRCUIT_NAME $DESCRIPTION

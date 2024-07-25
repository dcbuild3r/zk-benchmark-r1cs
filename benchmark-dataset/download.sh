#!/bin/bash

echo "Downloading benchmark datasets..."

# Proof of Passport circuits

echo "Proof of Passport circuits: 1.4GB download size"

echo "Downloading Proof of Passport circuits + witness (wrencher exports)..."
mkdir proof-of-passport
cd proof-of-passport

curl -o disclose_wrencher.json https://zk-benchmark-dataset.s3.eu-central-1.amazonaws.com/circuits/proof-of-passport/disclose_output.json
curl -o register_sha1WithRSAEncryption_65537_wrencher.json https://zk-benchmark-dataset.s3.eu-central-1.amazonaws.com/circuits/proof-of-passport/register_sha1WithRSAEncryption_65537_output.json
curl -o register_sha256WithRSAEncryption_65537_wrencher.json https://zk-benchmark-dataset.s3.eu-central-1.amazonaws.com/circuits/proof-of-passport/register_sha256WithRSAEncryption_65537_output.json
curl -o register_sha256WithRSASSAPSS_65537_wrencher.json https://zk-benchmark-dataset.s3.eu-central-1.amazonaws.com/circuits/proof-of-passport/register_sha256WithRSASSAPSS_65537_output.json
curl -o rsa_verifier_65537_2048_wrencher.json https://zk-benchmark-dataset.s3.eu-central-1.amazonaws.com/circuits/proof-of-passport/rsa_verifier_65537_2048_output.json
curl -o rsa_verifier_65537_4096_wrencher.json https://zk-benchmark-dataset.s3.eu-central-1.amazonaws.com/circuits/proof-of-passport/rsa_verifier_65537_4096_output.json

cd ../

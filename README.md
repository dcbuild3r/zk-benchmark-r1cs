# zk-benchmark-r1cs

A set of tools to serialize r1cs and witness native representations into a standardized dataset that can be used to benchmark different provers with an r1cs front-end.

## Usage

Download an example r1cs file, run `snarkjs rej [circuit.r1cs] [circuit.json]` to convert a r1cs file a json file which the convert command can understand.

Download or generate the corresponding witness file for the r1cs file. run `snarkjs wtns export json [witness.wtns] [witnes.json]` to convert a witness file a json file which the convert command can understand.

```bash
# R1CS + Witness -> JSON
cargo run --release -- ser-r1cs --r1cs-path <path_to_r1cs_export_file> --witness <path_to_witness_export_file> --output output.json
```

## Example

The `convert.sh` script in the `test/` directory will convert example `mulitplier.circom` and `rsa.circom` corresponding r1cs and witness files into a wrencher json file.

```bash
./wrencher/test/convert.sh
```

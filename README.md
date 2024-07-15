# zk-benchmark-r1cs

A set of tools to serialize r1cs and witness native representations into a standardized dataset that can be used to benchmark different provers with an r1cs front-end.

### Usage

Witness parsing unimplemented.

Download an example zkey file, run `snarkjs zkej [circuit_final.zkey] [circuit_final.zkey.json]` to convert a zkey file a json file which the convert command can understand.

```
cargo run --release -- convert-zkey --input semaphore.zkey.json --witness witness.wtns --output semaphore_zkey_and_witness.json
```

# Circuits

## Useful Commands

```sh
# Compile circuit
circom multiplier.circom --r1cs --wasm

# Powers of Tau (Needs to be done once)
npx snarkjs powersoftau new bn128 12 pot12_0000.ptau -v
npx snarkjs powersoftau contribute pot12_0000.ptau pot12_0001.ptau --name="First contribution" -v

# Compute witness with WebAssembly (Run from within the circuit's root directory)
node multiplier_js/generate_witness.js multiplier_js/multiplier.wasm input.json witness.wtns

## Powers of Tau (Run from with in the circuit's root directory)
npx snarkjs powersoftau prepare phase2 ../pot12_0001.ptau pot12_final.ptau -v
npx snarkjs groth16 setup multiplier.r1cs pot12_final.ptau multiplier_0000.zkey
npx snarkjs zkey contribute multiplier_0000.zkey multiplier_0001.zkey --name="1st Contributor Name" -v
npx snarkjs zkey export verificationkey multiplier_0001.zkey verification_key.json

## Generate proof with snarkjs (Run from with in the circuit's root directory)
npx snarkjs groth16 prove multiplier_0001.zkey witness.wtns proof.json public.json

## Verify proof with snarkjs (Run from with in the circuit's root directory)
npx snarkjs groth16 verify verification_key.json public.json proof.json
```

## Useful Resources

- [Circom 2 Documentation - Getting Started](https://docs.circom.io/getting-started/installation/)

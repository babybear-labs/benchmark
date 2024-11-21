**Getting Started:**

1.  Run npm install.
2.  Download powersoftau file from [here](https://github.com/iden3/snarkjs#7-prepare-phase-2). It should be stored in `poweroftau` directory in this project.
3.  In `scripts/compile_circuit.sh`, properly set `CIRCOMLIB_PATH` path to npm_modules.
4.  Update the size of input for circuits in their test files.
5.  For benchmarking run the following code:
    Poseidon: python3 test_poseidon.py
    Fibonacci: python3 test_fibonacci.py
    SHA256: python3 test_sha256.py

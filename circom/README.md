# Benchmark Circuits
[![Circom Badge](https://img.shields.io/badge/circuits-circom-black)](https://github.com/iden3/circom)
[![Snarkjs Badge](https://img.shields.io/badge/proof_system-snarkjs-yellow)](https://github.com/iden3/snarkjs)
![Python Badge](https://img.shields.io/badge/compile-python-green)





This repository is dedicated to a powerful and user-friendly framework designed for testing various circuits using Circom. Our main objective is to simplify the testing process, allowing developers to easily add new circuits and conduct reliable tests.

**Key features of the framework:**

1.  **Extensibility:** The framework is highly extensible. To add new circuits, simply follow a few clear rules and place the circuits in the `circuits/base` folder. Additionally, provide a commented `main` file following the appropriate formatting.
    
2.  **Easy Customization:** You can effortlessly customize circuit values using Python. Just make the necessary changes in the files `test_circuits.py` to adapt it to the new circuit.
    

**Getting Started:**

1.  Clone this repository to your local system.
2.  Add new circuits to the `circuits/base` folder.
3.  Modify the `main` line of the circuit with the correct formatting and values to be customized in Python.
4.  Edit the files `test_circuits.py` to adapt it to the new circuit, updating the circuit name, powersoftau, and other parameters as needed.
5.  Run the benchmark to ensure the circuit functions correctly.

**Troubleshooting** 
If you get any error, take care of properly set the following parameters:
* In `scripts/compile_circuit.sh`, properly set `CIRCOMLIB_PATH` path. If `circomlib` is not installed, run: `npm install circomlib`.
* In `test_circuits.py`, properly set `POT` variable with the path to the `*.ptau` file (download it from [here](https://github.com/iden3/snarkjs#7-prepare-phase-2)). It should be in `poweroftau` directory but it can be elsewhere.
* In `scripts/proving_system/prover.sh`, properly set `RAPIDSNARK` variable with the path to the `build/prover` executable. For information on how to install Rapidsnark visit [here](https://github.com/iden3/rapidsnark)

**Contributions are Welcome:**

[![MIT License](https://img.shields.io/badge/License-MIT-red.svg)](https://choosealicense.com/licenses/mit/)

We are open to contributions from the community to improve this framework and make it even more useful. Feel free to open issues, propose enhancements, or submit pull requests.

Thank you for choosing our framework. We hope it simplifies your Circom testing work and contributes to your success in developing secure and efficient circuits.


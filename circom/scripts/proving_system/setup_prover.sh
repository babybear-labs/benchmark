
if [ $# -ne 2 ]; then
    echo "Usage: ./setup_prover.sh <circuit_name> <pot_file>"
    exit 1
fi

CIRCUIT_NAME=$1
POT=$(readlink -f ${2})
CIRCUIT_DIR=$(readlink -f ./output/compiled_circuit/compiled_${CIRCUIT_NAME})

R1CS="${CIRCUIT_DIR}/${CIRCUIT_NAME}.r1cs"


mkdir -p output/snarkjs_circuit > /dev/null
mkdir -p output/snarkjs_circuit/${CIRCUIT_NAME} > /dev/null

cd output/snarkjs_circuit/${CIRCUIT_NAME}


snarkjs groth16 setup ${R1CS} ${POT} circuit_final.zkey
snarkjs zkey export verificationkey circuit_final.zkey verification_key.json


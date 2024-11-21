if [ $# -ne 1 ]; then
    echo "Usage: ./prover.sh <circuit_name>"
    exit 1
fi

CIRCUIT_NAME=$1
CIRCUIT_DIR=$(readlink -f ./output/compiled_circuit/compiled_${CIRCUIT_NAME})
WITNESS="${CIRCUIT_DIR}/${CIRCUIT_NAME}_witness.wtns"
RAPIDSNARK=/bin/prover

cd output/snarkjs_circuit/${CIRCUIT_NAME}

if [ -f "$RAPIDSNARK" ]; then
    ${RAPIDSNARK} circuit_final.zkey ${WITNESS} proof.json public.json
else
    echo "RapidSnark not found, using snarkjs instead."
    snarkjs groth16 prove circuit_final.zkey ${WITNESS} proof.json public.json
fi

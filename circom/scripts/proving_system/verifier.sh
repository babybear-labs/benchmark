#!/bin/bash

if [ $# -lt 1 ]; then
    echo "Usage: ./verifier.sh <circuit_name> [--generate-contract <contract_name>]"
    exit 1
fi

VERIFICATION_KEY=$(readlink -f ./output/snarkjs_circuit/${1}/verification_key.json)
PUBLIC=$(readlink -f ./output/snarkjs_circuit/${1}/public.json)
PROOF=$(readlink -f ./output/snarkjs_circuit/${1}/proof.json)

snarkjs groth16 verify ${VERIFICATION_KEY} ${PUBLIC} ${PROOF}

if [ "$2" == "--generate-contract" ]; then
    if [ -z "$3" ]; then
        echo "Missing contract name after --generate-contract option."
        exit 1
    fi

    CONTRACT_NAME="$3"
    ZKEY=$(readlink -f ./output/snarkjs_circuit/${1}/circuit_final.zkey)
    snarkjs zkey export solidityverifier ${ZKEY} ./output/snarkjs_circuit/${1}/Verifier_${CONTRACT_NAME}.sol
fi
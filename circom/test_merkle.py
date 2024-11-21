#!/usr/bin/python3.10
import random
# import node_modules.circomlib.test as nt
# from node_modules.circomlib.test.eddsa import buildEddsa 
from scripts.util import extract_contraints, generate_circuit, generate_input, measure_command, create_merkle_tree, generate_merkle_proof, validate_merkle_proof, generate_input_merkle

def test_circuit(circuit_name, input_path,pot_path,verbose=True,time = True, memory = True):
    # print("inside test_circuit")
    r1cs_path = 'output/compiled_circuit/compiled_{}/{}.r1cs'
    # print("r1cs_path: ",r1cs_path)
    t_c,m_c = measure_command(f'./scripts/compile_circuit.sh ./circuits/benchmark/{circuit_name}.circom {input_path} --nodejs',time,memory)
    if verbose:
        print(f'[{circuit_name}] Compile Circuit: {"" if t_c is None else f"{t_c} seconds"} {"" if m_c is None else f"{m_c} KB"}')
    constraints = extract_contraints(r1cs_path.format(circuit_name,circuit_name))
    if verbose:
        print(f'[{circuit_name}] Constraints: {constraints}')
    # print("before setup prover")
    t_sp,m_sp = measure_command(f'./scripts/proving_system/setup_prover.sh {circuit_name} {pot_path}',time,memory)
    if verbose:
        print(f'[{circuit_name}] Setup Prover: {"" if t_sp is None else f"{t_sp} seconds"} {"" if m_sp is None else f"{m_sp} KB"}')
    # print("after setup prover")
    # print("before prove")
    t_p,m_p = measure_command(f'./scripts/proving_system/prover.sh {circuit_name} ',time,memory)
    if verbose:
        print(f'[{circuit_name}] Prover: {"" if t_p is None else f"{t_p} seconds"} {"" if m_p is None else f"{m_p} KB"}')
    # print("after prove")
    t_v,m_v = measure_command(f'./scripts/proving_system/verifier.sh {circuit_name}',time,memory)
    if verbose:
        print(f'[{circuit_name}] Verifier: {"" if t_v is None else f"{t_v} seconds"} {"" if m_v is None else f"{m_v} KB"}')
    
    return {'CIRCUIT':circuit_name,
           'INPUT SIZE':input_path.split('_')[-1].split('.')[0],
           'CONSTRAINTS':constraints,
           'COMPILE_TIME':t_c,
           'COMPILE_MEMORY':m_c,
           'SETUP_TIME':t_sp,
           'SETUP_MEMORY':m_sp,
           'PROVER_TIME':t_p,
           'PROVER_MEMORY':m_p,
           'VERIFIER_TIME':t_v,
           'VERIFIER_MEMORY':m_v}


if __name__ == '__main__':
    # test sha256 circuit given the size of an image as input
    TIME, MEMORY = True, True
    POT = './powersoftau/28pot.ptau'
    NUM = 3
    # Generate a random array of BigInt-like numbers
    def random_array(length):
        return [int(random.random() * 10 ** 45) for _ in range(length)]
    # Create the array of secret keys
    secret_keys = random_array(2 ** NUM)
    # Choose a random key from the array
    chosen_key = random.choice(secret_keys)
    # Get the index of the chosen key
    chosen_key_index = secret_keys.index(chosen_key)
    # Print the index of the chosen key
    print(chosen_key_index)

    circuit_name = f'merkle'
    
    # print("before generate_circuit")
    generate_circuit({'NUM':NUM},f'./circuits/base/{circuit_name}.circom',id=NUM)
    # print("after generate_circuit")
    # print("before generate_input")
    # eddsa = nt.eddsa.buildEddsa()
    # field = eddsa.baby_jub.F
    # hash_function = eddsa.poseidon

    merkle_tree = create_merkle_tree(secret_keys, NUM)
    merkle_root = merkle_tree[0]
    merkle_proof = generate_merkle_proof(merkle_tree, chosen_key_index, NUM)
    print("merkle_root: ",merkle_root)
    print("merkle_proof: ",merkle_proof)

    generate_input_merkle(f'./input/input_merkle_{NUM}.json',chosen_key_index,chosen_key,merkle_root,merkle_proof)
    # print("after generate_input")
    # print("before test_circuit")

    measures = test_circuit(f'{circuit_name}_{NUM}',f'/input/input_merkle_{NUM}.json',POT,time=TIME,memory=MEMORY)
    print("measures: ",measures)
    
    
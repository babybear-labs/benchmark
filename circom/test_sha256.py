#!/usr/bin/python3.10

from scripts.util import extract_contraints, generate_circuit, generate_input, measure_command

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
    NUM = 2048

    circuit_name = f'sha256_bytes'
    
    # print("before generate_circuit")
    generate_circuit({'NUM':NUM},f'./circuits/base/{circuit_name}.circom',id=NUM)
    # print("after generate_circuit")
    # print("before generate_input")
    generate_input(f'./input/input_{NUM}.json',NUM)
    # print("after generate_input")
    # print("before test_circuit")

    measures = test_circuit(f'{circuit_name}_{NUM}',f'/input/input_{NUM}.json',POT,time=TIME,memory=MEMORY)
    print("measures: ",measures)
    
    
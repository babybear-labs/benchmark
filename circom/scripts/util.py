#!/usr/bin/env python3

from csv import DictWriter
import json
import os
os.environ['TF_CPP_MIN_LOG_LEVEL'] = '2'
import tensorflow as tf
import numpy as np
import pandas as pd
from math import sqrt
import random
import subprocess
import psutil
import re
import hashlib



def measure_command(command, time = True, memory = True):
    """
    Measure the time and memory usage of a specified command.

    :param command: The command to execute and measure.
    :param time: True if you want to measure time, False otherwise.
    :param memory: True if you want to measure memory usage, False otherwise.

    :return: A tuple containing the elapsed time (if time=True) and memory usage (if memory=True).
    """
    command = f'/usr/bin/time -p -f "%e %M" {command} > /dev/null'
    # print("command: ",command)
    if memory:
        init_swap_used = psutil.swap_memory().used
        max_swap_used = init_swap_used
    # print("after memory check")
    process = subprocess.Popen(command, 
                               shell=True,
                               stdout=subprocess.PIPE,
                               stderr=subprocess.PIPE)
    
    if memory:
        while process.poll() is None:
            max_swap_used = max(max_swap_used, psutil.swap_memory().used)
    # print("After second memory check")
    command_output = process.communicate()[1].decode('utf-8')
    print("command_output: \n",command_output)
    # print("after command output")
    t,mem = command_output.split('\n')[0].split(' ')
    t = float(t)

    if memory:
        swap = (max_swap_used-init_swap_used)/1024
        m = float(mem)+(swap if swap > 0 else 0)
    
    return t if time else None, m if memory else None


def measure_pagefault_time_command(command, password=None):
    """
    Measure the time and number of page faults of a specified command.

    :param command: The command to execute and measure.
    :param password: The password to pass to the command, if it necessitates sudo privileges.
    :return: A tuple containing the elapsed time and number of page faults.
    """
    command = f'/usr/bin/time -p -f "%e %F %P %W %M" {command}'
    print(command)
    if password is not None:
        command = f'echo "{password}" | sudo -S {command}'
    
    process = subprocess.Popen(command,
                                shell=True,
                                stdout=subprocess.PIPE,
                                stderr=subprocess.PIPE)

    command_output = process.communicate()[1].decode('utf-8')
    
    time, pagefaults, cpu_percentage, swap_out, mem = command_output.split('\n')[0].split(' ') if (len(command_output.split(": ")) == 1) else command_output.split(": ")[1].split('\n')[0].split(' ')

    return float(time), int(pagefaults), cpu_percentage, int(swap_out), mem


def generate_circuit(info, circuit_template, id = None):
    """
    Generate a circuit from a template
    :param info: dictionary with the information to replace in the template
    :param circuit_template: path to the template
    :param id: id of the circuit

    """
    out_circuit = circuit_template.split('/')[-1].split('.')[0]
    print("out_circuit: {}",out_circuit)
    os.makedirs('circuits/benchmark',exist_ok=True)

    with open(circuit_template, 'r') as infile:
        circuit = infile.read()
        for k,v in info.items():
            circuit = circuit.replace(k, str(v))
        circuit = circuit.replace('//MAIN', '')
        
        id = f'_{id}' if id is not None else ''
        out_path = f'circuits/benchmark/{out_circuit}{id}.circom'
        print("out_path: {}",out_path)
        with open(out_path, 'w') as outfile:
            outfile.write(circuit)
    return out_path


def append_to_csv(row,csv_path):
    """
    Append a row to a csv file if it exists, otherwise create it
    :param row: dictionary with the row to append
    :param csv_path: path to the csv file
    """
    with open(csv_path, 'a+', newline='') as csv_file:
        dict_writer = DictWriter(csv_file, fieldnames=list(row.keys()))
        if 0 == os.stat(csv_path).st_size:
            dict_writer.writeheader()
        dict_writer.writerow(row)


def generate_input(output_path, size):
    """
    Generate a random input for a circuit of a given size
    :param output_path: path to the output file
    :param size: size of the input
    """
    json_input = {'in':[str(random.randint(0, 255)) for _ in range(size)] }
    os.makedirs('input',exist_ok=True)
    with open(output_path, 'w') as outfile:
        json.dump(json_input, outfile)

def generate_input_fibonnaci(output_path):
    """
    Generate a random input for a circuit of a given size
    :param output_path: path to the output file
    :param size: size of the input
    """
    json_input = {'in':['1','1'] }
    os.makedirs('input',exist_ok=True)
    with open(output_path, 'w') as outfile:
        json.dump(json_input, outfile)

def generate_input_merkle(output_path,chosen_key_index,chosen_key,merkle_root,merkle_proof):
    """
    Generate a random input for a circuit of a given size
    :param output_path: path to the output file
    :param size: size of the input
    """
    json_input = {'key': chosen_key_index,
      'value': chosen_key,
      'root': merkle_root,
      'siblings': merkle_proof
      }
    os.makedirs('input',exist_ok=True)
    with open(output_path, 'w') as outfile:
        json.dump(json_input, outfile)

def compute_input(GB):
    """
    Compute the size of the input from a line, built with linear regression, 
    measuring the number constraints from the input size
    :param GB: the amount of available memory
    """
    # Read data from the CSV file
    data = pd.read_csv('./scripts/benchmark_circuits_resize.csv')

    # Extract 'INPUT SIZE' and 'CONSTRAINTS' columns from the DataFrame
    input_size = data['INPUT SIZE']
    constraints = data['CONSTRAINTS']

    # Perform linear regression
    slope, intercept = np.polyfit(input_size, constraints, 1)
    
    best_constraints = 8388608 * GB / 8.996896768
    
    return int(sqrt(abs(best_constraints / slope - intercept))), int(best_constraints)



def extract_contraints(r1cs_file):
    infos = subprocess.check_output(f'snarkjs r1cs info {r1cs_file}',shell=True).decode('utf-8')
    return int(re.search(r'# of Constraints: (\d+)',infos).group(1))



def resize_image(image, height, width):
    """
    Resize an image to the given dimensions.
    :param image_path: Path to the image to resize.
    :param height: Height of the resized image.
    :param width: Width of the resized image.
    :return: the resized image.
    """
    original_height, original_width, _ = image.shape

    if (original_height-1) % (height-1) != 0 or (original_width-1) % (width-1) != 0:
        divisors_h = [v+1 for v in range(1, (original_height - 1)//2) if (original_height - 1) % v == 0]
        divisors_w = [v+1 for v in range(1, (original_width - 1)//2) if (original_width - 1) % v == 0]
        raise ValueError(f"The image cannot be resized to the given dimensions.\n The height must be one of this numbers: {divisors_h}\
                          \n The width must be one of this numbers: {divisors_w}")

    return (
        (
            tf.compat.v1.image.resize(
                image,
                [height, width],
                align_corners=True,
                method=tf.image.ResizeMethod.BILINEAR,
            )
        )
        .numpy()
        .round()
        .astype(np.uint8)
    )
    


def generate_random_image(height, width = None):
    """
    Generate a random image.
    :param height: Height of the image.
    :param width: Width of the image.
    :return: the random image.
    """
    if width is None:
        width = height
    rimg =  np.random.randint(0, 256, size=(height, width, 3), dtype=np.uint8)
    return rimg

def generate_resize_input(output_path, f_height,f_width,r_height,r_width):
    """
    Generate a random input that fits the cirtuit for the resize operation check.
    :param output_path: Path to the output file.
    :param f_height: Height of the full image.
    :param f_width: Width of the full image.
    :param r_height: Height of the resized image.
    :param r_width: Width of the resized image.
    """
    img = generate_random_image(f_height,f_width)
    rsz = resize_image(img,r_height,r_width)

    json_input = {'full_image':img.astype(str).tolist(), 'resize_image':rsz.astype(str).tolist() }
    os.makedirs('input',exist_ok=True)
    with open(output_path, 'w') as outfile:
        json.dump(json_input, outfile)

def create_merkle_tree(arr, n_levels):
    extended_len = 1 << n_levels

    h_arr = []
    for i in range(extended_len):
        if i < len(arr):
            h_arr.append(hashlib.sha256(str(arr[i]).encode()).hexdigest())
        else:
            h_arr.append(0)
    
    return __merkelize(h_arr)

def __merkelize(arr):
    if len(arr) == 1:
        return arr
    
    h_arr = []
    for i in range(len(arr) // 2):
        combined = str(arr[2 * i]) + str(arr[2 * i + 1])
        h_arr.append(hashlib.sha256(combined.encode()).hexdigest())
    
    m = __merkelize(h_arr)
    return m + arr

def generate_merkle_proof(m, key, n_levels):
    if n_levels == 0:
        return []
    
    extended_len = 1 << n_levels
    top_siblings = generate_merkle_proof(m, key >> 1, n_levels - 1)
    cur_sibling = m[extended_len - 1 + (key ^ 1)]
    return top_siblings + [cur_sibling]

def validate_merkle_proof(F, hash_fn, key, value, root, proof):
    h = hash_fn([value])
    for i in range(len(proof) - 1, -1, -1):
        if (1 << (len(proof) - 1 - i)) & key:
            h = hash_fn([proof[i], h])
        else:
            h = hash_fn([h, proof[i]])
    
    return F.eq(root, h)





if __name__ == '__main__':
    raise ValueError('This script is not meant to be executed directly')
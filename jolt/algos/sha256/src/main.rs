use jolt::Serializable;

pub fn main() {
    let start = std::time::Instant::now();
    let proving_time = std::time::Instant::now();
    let (prove_sha2, verify_sha2) = guest::build_sha2();

    let input = &[5u8; 4000];
    let (output, proof) = prove_sha2(input);
    println!("Prover Time {:?}", proving_time.elapsed());

    proof.save_to_file("proof.bin").expect("Failed to save proof to file");
    println!("Proof Size {:?}", proof.size().unwrap());
    println!();

    let verify_time = std::time::Instant::now();
    let is_valid = verify_sha2(proof);
    println!("Verify Time {:?}", verify_time.elapsed());
    println!();

    println!("result: {:?}", output);
    println!("valid: {}", is_valid);

    println!("Total Time elapsed: {:?}", start.elapsed());
}
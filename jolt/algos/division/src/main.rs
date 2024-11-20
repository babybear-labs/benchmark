use jolt::Serializable;

pub fn main() {
    let start = std::time::Instant::now();
    let proving_time = std::time::Instant::now();
    let (prove_div, verify_div) = guest::build_div();
    let (output, proof) = prove_div(40, 10);
    println!("Prover Time {:?}", proving_time.elapsed());

    proof.save_to_file("proof.bin").expect("Failed to save proof to file");
    println!("Proof Size {:?}", proof.size().unwrap());
    println!();

    let verify_time = std::time::Instant::now();
    let is_valid = verify_div(proof);
    println!("Verify Time {:?}", verify_time.elapsed());
    println!();

    println!("result: {:?}", output);
    println!("valid: {}", is_valid);

    println!("Total Time elapsed: {:?}", start.elapsed());
}

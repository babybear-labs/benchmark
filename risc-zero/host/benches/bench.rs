extern crate host;

use benchy::{benchmark, BenchmarkRun};
use host::{fib::fib, poseidon::poseidon, sha::sha};
use risc0_zkvm::{Receipt, SessionStats};

// #[benchmark]
// fn assert(b: &mut BenchmarkRun) {
//     let prove = host::assert::assert(1, 2);
//     log_session(&b.run(prove), b);
// }

#[benchmark("Fibonacci", [
    // ("1", 1),
    // ("10", 10),
    // ("100", 100),
    // ("1000", 1000),
    // ("10000", 10000),
    ("100000", 100000),
])]
fn fibonacci(b: &mut BenchmarkRun, n: u32) {
    let prove = fib(n);
    log_session(&b.run(prove), b);
}

#[benchmark("SHA256", [
    ("1k bytes", 1),
    // ("10k bytes", 10),
    // ("100k bytes", 100),
])]
fn sha256(b: &mut BenchmarkRun, n: usize) {
    let prove = sha(n);
    log_session(&b.run(prove), b);
}

// #[benchmark("Blake3", [
//     ("1k bytes", 1),
//     ("10k bytes", 10),
//     // ("100k bytes", 100),
// ])]
// fn blake3_bench(b: &mut BenchmarkRun, n: usize) {
//     let prove = blake3(n);
//     log_session(&b.run(prove), b);
// }

// #[benchmark("Merkle Tree Merge", [
//     ("1 + 1", (tree_size_n::<Sha>(0), tree_size_n::<Sha>(0))),
//     // ("2^10 + 2^10", (tree_size_n(10), tree_size_n(10))),
//     // ("2^10 + 2^20", (tree_size_n(10), tree_size_n(20))),
//     // ("2^20 + 2^20", (tree_size_n(20), tree_size_n(20))),
// ])]
// fn merkle_merge(b: &mut BenchmarkRun, (tree1, tree2): (Tree<Sha>, Tree<Sha>)) {
//     let prove = merkle::merkle(tree1, tree2);
//     log_session(&b.run(prove), b);
// }

#[benchmark("Poseidon")]
fn poseidon_hash(b: &mut BenchmarkRun) {
    let n: u32 = 1;
    let prove = poseidon(n);
    log_session(&b.run(prove), b);
}

fn log_session((receipt, session): &(Receipt, SessionStats), b: &mut BenchmarkRun) {
    let cycles = session.total_cycles;
    // let segments = session.resolve().unwrap();
    // let (cycles, insn_cycles) = segments
    //     .iter()
    //     .fold((0, 0), |(cycles, insn_cycles), segment| {
    //         (
    //             cycles + (1 << segment.po2),
    //             insn_cycles + segment.insn_cycles,
    //         )
    //     });
    b.log("cycles", cycles.try_into().unwrap());
    // b.log("instruction_cycles", insn_cycles);

    let proof = bincode::serialize(receipt).unwrap();
    b.log("proof_size_bytes", proof.len());
    b.log(
        "compressed_proof_size_bytes",
        zstd::encode_all(&proof[..], 21).unwrap().len(),
    );
}

benchy::main!(
    "risc-zero",
    // assert,
    fibonacci,
    sha256,
    // blake3_bench,
    // merkle_merge,
    poseidon_hash,
);

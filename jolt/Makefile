# TODO: Separate out run and build, store run logs in prove.log instead of cycles.txt
export RUST_LOG=info

# Rust benchmarks
merkle-rust = algos/merkle
merkle-proof-rust = algos/merkle_proof
nth-prime-rust = algos/nth_prime
keccak-rust = algos/keccak
poseidon-rust = algos/poseidon
bls-agg-rust = algos/bls-agg
ecdsa-verify-rust = algos/ecdsa_verify
bls-verify-rust = algos/bls-verify

all:
	@echo "Running all benchmarks..."
	make run-nth-prime

run-nth-prime:
	@echo "Running nth-prime benchmark..."
	cd ${nth-prime-rust}/native_jolt; cargo run --release > cycles.txt
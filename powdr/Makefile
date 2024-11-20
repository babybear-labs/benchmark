# TODO: Separate out run and build, store run logs in prove.log instead of cycles.txt
export RUST_LOG=info

# Rust benchmarks
nth-prime-rust = algos/nth_prime

all:
	@echo "Running all benchmarks..."
	make run-nth-prime

run-nth-prime:
	@echo "Running nth-prime benchmark..."
	cd ${nth-prime-rust}/; cargo run --release > cycles.txt
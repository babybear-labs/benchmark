# TODO: Separate out run and build, store run logs in prove.log instead of cycles.txt
export RUST_LOG=info

# Rust benchmarks
add = algos/addition
div = algos/division
multi = algos/multiplication
sub = algos/substraction
fib = algos/fib
sha = algos/sha256

all:
	@echo "Running all benchmarks..."
	make operations

operations:
	@echo "Running nth-prime benchmark..."
	cd ${add}/; cargo run --release > cycles.txt
	cd ${div}/; cargo run --release > cycles.txt
	cd ${multi}/; cargo run --release > cycles.txt
	cd ${sub}/; cargo run --release > cycles.txt
	cd ${fib}/; cargo run --release > cycles.txt
	cd ${sha}/; cargo run --release > cycles.txt
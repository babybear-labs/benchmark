# SP1

## Quickstart

Go to each benchmark folder. Then go to `script` folder. Run.
`cargo run --release -- --prove --n 10`


Two different execution environment.
- Native SP1
- WASM SP1

## Key Differences: Native SP1 vs. WASM SP1

| **Aspect**           | **Native SP1**                                       | **WASM SP1**                                           |
|-----------------------|-----------------------------------------------------|-------------------------------------------------------|
| **Execution Environment** | Runs directly as a compiled binary (native code).   | Executes in a WebAssembly (WASM) runtime.             |
| **Performance**       | Faster execution due to direct system-level integration and optimized native compilation. | Slower because it runs within a WASM interpreter or JIT engine. |
| **Proof Generation Speed** | Optimized for speed, with lower overhead during proof generation. | Higher latency in proof generation due to WASM overhead. |
| **Portability**       | Limited to specific system architectures and platforms where the native binary is built. | Highly portable across platforms that support WASM (browsers, WASM-compatible runtimes). |
| **Ease of Setup**     | Requires proper native toolchains and dependencies (e.g., Rust toolchain for compiling). | Easier setup since WASM binaries are platform-independent. |
| **Use Case**          | Ideal for performance-critical applications like zkRollups or large-scale computations. | Suitable for prototyping, cross-platform usage, or environments where portability is key. |
| **Debugging**         | Requires native debugging tools and a more complex environment for inspection. | Easier to debug in environments with WASM tooling support (e.g., browser dev tools). |
| **Binary Size**       | Smaller runtime overhead but larger native binaries. | Slightly larger runtime due to WASM sandboxing. |
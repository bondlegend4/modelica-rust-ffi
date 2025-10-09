# ModelicaRuntime - Safe Wrapper

## Overview

`ModelicaRuntime` provides a 100% safe Rust API to OpenModelica-generated C code.

## Safety Guarantees

- [x] No `unsafe` in public API
- [x] Automatic memory management via `Drop`
- [x] All errors returned as `Result`, never panics
- [x] Bounds checking on all variable access
- [x] Validation of all inputs

## run
### Build
cargo build

### Run all tests
cargo test

### Run specific test with output
cargo test test_simulation_step -- --nocapture

## Usage

See `tests/integration_test.rs` for comprehensive examples.

## Future Work

Currently uses Rust-based simulation. Next steps:
1. Initialize actual OpenModelica DATA structures
2. Call OpenModelica simulation functions
3. Add thread-safety with Arc<Mutex<>>
4. Support FMU export
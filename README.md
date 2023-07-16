# DEPRECATED
This repo is under developement in a private org.

# Experiments with WASM


### Go
The host folder contains the GO code for running rust based WASM using wazero.
There are two examples.
 - `power.go` exports a `power` function to WASM and
calls an `add` and `double_power` functon exported from WASM
 - `wasi_sys_call.go` is an example of IO using WASI


### Rust
The `rust_wasm` folder contains rust code I compiled down to WASM to show some
simple examples.

The `token_contract` folder is contains rust code for a simple token contract,
that will compiled to WASM and used by wazero runtime.

## Vocabulary

`Host`: The Host is the program that loads and executes the Wasm module. 

`Host Function`: Host Function is a function defined in the Host program. For
Wasm, the Host Function can be used as an import segment to be registered in a
module, and then it can be called when Wasm is running.

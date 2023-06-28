Expirements with WASM

The host folder contains the GO code for running rust based WASM using wazero.
There are two examples.
- `power.go` exports a `power` function to WASM and calls an `add` and `double_power` functon exported from WASM
- `wasi_sys_call.go` is an example of IO using WASI

The rust_wasm folder contains rust code I compiled down to WASM to show some simple examples.

The token_contract folder is more complicated rust file that will be compiled to WASM and used by wazero runtime.



fn main() {
    println!("Hello from the rust main method!!");
    println!("Ben");
}


// Compiled with
// rustc -o hello.wasm --target wasm32-wasi src/wasi_sys_call.rs
// need to change the target to wasm32-wasi to import WASI syscalls
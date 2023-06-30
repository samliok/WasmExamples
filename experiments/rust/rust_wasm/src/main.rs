fn main() {
    println!("Hello from the RUST CODE");
    println!("I'Ve added 4 + 1 and got {}", add(4, 1));
}

// this will be an exported function.
// We use no_mangle to tell rust compiler to not mangle symbol name add
// We use extern "C" to ...
#[no_mangle]
pub extern "C" fn add(x: i32, y: i32) -> i32 {
    x + y
}

#[no_mangle]
pub extern "C" fn double_power(x: i32) -> i32 {
    add(power(x), power(x))
}

// wraps the unsafe _power function in an unsafe block
fn power(x: i32) -> i32 {
    unsafe {
        // tells rust compiler what is in this block might contain unsafe operations
        // b/c the function is an external function
        _power(x)
    }
}

#[link(wasm_import_module = "env")] // function will be imported from env module(set by host)
extern "C" {
    // declares foreign functions following c calling convention
    #[link_name = "power"] // specifies the name of the symbol to be linked and links to fn _power
    fn _power(size: i32) -> i32;
}

// converted to wasm via
// rustc -o add.wasm --target wasm32-unknown-unknown src/main.rs

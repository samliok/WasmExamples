fn main() {
    //     // create a `Vec<u8>` as input
    //     let input = vec![104,101,108,108,111];
    //     // call the `alloc` function
    //     let ptr = alloc(input.len());
    //     let res: usize;
    //     unsafe {
    //         // copy the contents of `input`into the buffer
    //         // returned by `alloc`
    //         std::ptr::copy(input.as_ptr(), ptr, input.len());
    //         // call the `array_sum` function with the pointer
    //         // and the length of the array
    //         res = read_bytes(ptr, input.len());
    //         dealloc(ptr, input.len());
    //     }
    //     // print the result
    //     println!("Result: {:#?}", res);
}

// exports a function, that reads reads bytes to be used by wasm
#[no_mangle]
pub extern "C" fn read_bytes(bytes_ptr: *const u8, length: usize) -> usize {
    println!("Length of the array: {}", length);
    let bytes_slice = unsafe { std::slice::from_raw_parts(bytes_ptr, length) };
    // Print the bytes
    if let Ok(string) = std::str::from_utf8(bytes_slice) {
        println!("String: {}", string);
    } else {
        println!("Invalid UTF-8 sequence");
    }
    length
}

/* memory functions ------------------------------------------- */
// https://radu-matei.com/blog/practical-guide-to-wasm-memory/

// Allocate memory into the module's linear memory
// and return the offset to the start of the block.

#[no_mangle]
pub fn alloc(len: usize) -> *mut u8 {
    // create a new mutable buffer with capacity `len`
    let mut buf = Vec::with_capacity(len);
    // take a mutable pointer to the buffer
    let ptr = buf.as_mut_ptr();
    // take ownership of the memory block and
    // ensure that its destructor is not
    // called when the object goes out of scope
    // at the end of the function
    std::mem::forget(buf);
    // return the pointer so the runtime
    // can write data at this offset
    return ptr;
}

#[no_mangle]
pub unsafe fn dealloc(ptr: *mut u8, size: usize) {
    let data = Vec::from_raw_parts(ptr, size, size);
    std::mem::drop(data);
}

// rustc -o bytes.wasm --target wasm32-wasi src/main.rs

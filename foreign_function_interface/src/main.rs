// Run with
//   > RUSTFLAGS="-A dead_code -A unused -L ." LD_LIBRARY_PATH=. cargo run 

use libc::size_t;
use libc::c_int;

// The extern block is a list of function signatures in a foreign library, in this 
// case with the platform’s C ABI. The #[link(...)] attribute is used to instruct 
// the linker to link against the snappy library so the symbols can be resolved

#[link(name = "snappy")]

// Foreign functions are assumed to be unsafe so calls to them need to be wrapped 
// with unsafe {} as a promise to the compiler that everything contained within truly
// is safe

unsafe extern "C" {

    fn snappy_compress(
        input: *const u8,
        input_length: size_t,
        compressed: *mut u8,
        compressed_length: *mut size_t) -> c_int;

    fn snappy_uncompress(
        compressed: *const u8,
        compressed_length: size_t,
        uncompressed: *mut u8,
        uncompressed_length: *mut size_t) -> c_int;

    fn snappy_max_compressed_length(source_length: size_t) -> size_t;

    fn snappy_uncompressed_length(
        compressed: *const u8,
        compressed_length: size_t,
        result: *mut size_t) -> c_int;

    fn snappy_validate_compressed_buffer(
        compressed: *const u8,
        compressed_length: size_t) -> c_int;
}

pub fn validate_compressed_buffer(src: &[u8]) -> bool {
    unsafe {
        snappy_validate_compressed_buffer(src.as_ptr(), src.len() as size_t) == 0
    }
}

pub fn compress(src: &[u8]) -> Vec<u8> {
    unsafe {
        let srclen = src.len() as size_t;
        let psrc = src.as_ptr();

        let mut dstlen = snappy_max_compressed_length(srclen);
        let mut dst = Vec::with_capacity(dstlen as usize);
        let pdst = dst.as_mut_ptr();

        snappy_compress(psrc, srclen, pdst, &mut dstlen);
        dst.set_len(dstlen as usize);
        dst
    }
}

pub fn uncompress(src: &[u8]) -> Option<Vec<u8>> {
    unsafe {
        let srclen = src.len() as size_t;
        let psrc = src.as_ptr();

        let mut dstlen: size_t = 0;
        snappy_uncompressed_length(psrc, srclen, &mut dstlen);

        let mut dst = Vec::with_capacity(dstlen as usize);
        let pdst = dst.as_mut_ptr();

        if snappy_uncompress(psrc, srclen, pdst, &mut dstlen) == 0 {
            dst.set_len(dstlen as usize);
            Some(dst)
        } else {
            None // SNAPPY_INVALID_INPUT
        }
    }
}

//-----------------------------------------
// Callbacks from C code to Rust functions

// Some external libraries require the usage of callbacks to report back their current
// state or intermediate data to the caller

// Use explicit "C" ABI
extern "C" fn callback_i32(a: i32) {
    println!("I'm called from C with value {0}", a);
}

#[link(name = "extlib")]
unsafe extern "C" {
    fn register_callback_i32(cb: extern "C" fn(i32)) -> i32;
    fn trigger_callback_i32();
}

//-------------------------------------
// Targeting callbacks to Rust objects

// Tell Rust to use C-compatible layout rules
#[repr(C)]
struct RustObject {
    a: i32,
    // Other members...
}

unsafe extern "C" fn callback_rust(target: *mut RustObject, a: i32) {
    println!("I'm called from C with value {0}", a);
    unsafe {
        // Update the value in RustObject with the value received from the callback:
        (*target).a = a;
    }
}

#[link(name = "extlib")]
unsafe extern "C" {

    fn register_callback_rust(
        target: *mut RustObject,
        cb: unsafe extern "C" fn(*mut RustObject, i32)) -> i32;

    fn trigger_callback_rust();
}

/*
 *  Partial vs. Final Artifacts
 *    When building a Rust project, rustc creates two distinct categories of compilation
 *    outputs:
 *
 *    Partial Artifacts (rlib, staticlib): 
 *      These are intermediate building blocks. An rlib (Rust Library) is not a fully 
 *      standalone file ready for deployment; it is essentially an archive of compiled 
 *      object code packaged alongside metadata, meant to be consumed by other Rust code   
 *
 *    Final Artifacts (dylib, binary / bin): 
 *      These are the end products. They are either a standalone executable file (.exe / 
 *      ELF binary) or a dynamic library (.so / .dll / .dylib) intended to be loaded by 
 *      the operating system or an external application
 */

//---------------------------
// Accessing foreign globals

#[link(name = "extlib")]
unsafe extern "C" {

    // Declare the foreign global variable. 
    // Use `static mut` if you intend to modify it.
    static mut FOO: i32;
    
    // Helper function from C
    fn get_foo() -> i32;
}

// Note: strings are not terminated with \0. If you need a NUL-terminated string for 
// interoperability with C, you should use the CString type in the std::ffi module

//--------------------
// Variadic functions

#[link(name = "extlib")]
unsafe extern "C" {
    // The `...` tells Rust this is a variadic C function
    fn sum_values(count: i32, ...) -> i32;
}

//-----------------------------
// Representing opaque structs

// An empty enum models an opaque type in Rust FFI. 
// It cannot be instantiated by value, which matches C's opaque handle behavior.
pub enum Foo {}

#[link(name = "extlib")]
unsafe extern "C" {
    fn create_foo(initial_value: i32) -> *mut Foo;
    fn get_foo_value(f: *mut Foo) -> i32;
    fn set_foo_value(f: *mut Foo, new_value: i32);
    fn destroy_foo(f: *mut Foo);
}

fn main() {
    {
        let x = unsafe { snappy_max_compressed_length(100) };
        println!("max compressed length of a 100 byte buffer: {}", x);
    }

    {
        unsafe {
            register_callback_i32(callback_i32);
            trigger_callback_i32(); // Triggers the callback.
        }
    }

    {
        // Create the object that will be referenced in the callback:
        let mut rust_object = Box::new(RustObject { a: 5 });

        unsafe {
            register_callback_rust(&mut *rust_object, callback_rust);
            trigger_callback_rust();
        }
    }

    {
        unsafe {
            // 1. Read using raw pointer address to avoid a shared reference
            let initial = std::ptr::addr_of!(FOO).read();
            println!("Initial FOO from C: {}", initial);

            // 2. Modify the foreign global variable directly
            FOO = 100;

            // 3. Read again via raw pointer
            let modified = std::ptr::addr_of!(FOO).read();
            println!("Modified FOO inside Rust: {}", modified);

            // 4. Call C function to prove it sees the change
            println!("FOO read back via C function: {}", get_foo());
        }
    }

    {
        unsafe {
            // Pass 3 values: 10, 20, and 30
            let result = sum_values(3, 10, 20, 30);
            println!("Sum from C variadic function: {}", result);
        }
    }

    {
        unsafe {
            // 1. Create the opaque object managed by C
            let foo_ptr = create_foo(42);
            
            if foo_ptr.is_null() {
                eprintln!("Failed to allocate Foo");
                return;
            }

            // 2. Read the value via C function
            println!("Initial value from opaque struct: {}", get_foo_value(foo_ptr));

            // 3. Update the value via C function
            set_foo_value(foo_ptr, 100);
            println!("Updated value from opaque struct: {}", get_foo_value(foo_ptr));

            // 4. Clean up memory
            destroy_foo(foo_ptr);
        }
    }
}

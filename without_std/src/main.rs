/*
 *  This Rust code is a minimal, bare-metal-style program that manually implements
 *  the underlying machinery normally provided automatically by the Rust Standard 
 *  Library (std)
 *
 *  By default, Rust programs rely on std, which sets up the runtime, handles panics, 
 *  and provides an entry point. This code strips all of that away, opting instead
 *  to build a program from scratch using only core and raw system interfaces
 *
 *  Execute with:
 *    RUSTFLAGS="-A unused" cargo +nightly run
 */

// Disables the Rust Standard Library (std). The program only has access to core, 
// which means no heap allocation, no standard I/O (println!), and no OS threads 
// unless implemented manually
#![no_std]

// Disables Rust’s standard startup process. Usually, std provides a hidden main 
// function that initializes things before calling your main. With this attribute, 
// you are defining the raw entry point yourself
#![no_main]

// These enable unstable, compiler-internal features usually reserved for developing
// the standard library itself
#![feature(lang_items, core_intrinsics, rustc_private)]
#![allow(internal_features)]

// Pulls in Rust’s compiler-internal stack-unwinding crate (used to clean up stack 
// frames when a panic occurs
#![feature(panic_unwind)]
extern crate unwind;

// Links the system’s C standard library (libc), which is required for low-level system
// functions like aborting the program
#[cfg(not(windows))]
extern crate libc;

use core::ffi::{c_char, c_int};
use core::panic::PanicInfo;
use libc::abort;

// Entry point for this program

// Tells the Rust compiler not to change the name of the function during compilation. 
// The linker expects to find a symbol named literally main
#[unsafe(no_mangle)]

// extern "C": Uses the C calling convention so that the operating system or system 
// loader can successfully call this function when the program starts
extern "C" fn main(_argc: c_int, _argv: *const *const c_char) -> c_int {
    0
}

// Because there is no standard library to provide them, you must manually define what 
// happens during a panic and how exception handling works

/*
 *  eh stands for Exception Handling. The personality function is a helper routine 
 *  defined by the language runtime that the underlying operating system or compiler 
 *  uses during this unwinding process
 *
 *  When a panic happens, the system looks at the compiled binary's metadata tables. 
 *  It calls the eh_personality function to answer questions like:
 *
 *    * "Does this current function want to catch this panic?"
 *    * "What clean-up code (destructors) needs to be run before we leave this stack 
 *      frame?"
 *
 *  In a standard Rust application, eh_personality contains complex logic provided by 
 *  the standard library (std) to handle unwinding safely across different platforms
 *
 *  Because the Rust compiler requires the symbol to exist whenever stack unwinding  
 *  is enabled (which is the default behavior), the code provides an empty stub 
 *  (fn rust_eh_personality() {}) just to satisfy the compiler's linker requirements 
 *  so the code will successfully compile
 *
 */
#[lang = "eh_personality"]
fn rust_eh_personality() {}

// Defines what Rust should do if a panic occurs
#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! { 
    unsafe { 
        abort() // libc::abort requires an unsafe block
    }
}

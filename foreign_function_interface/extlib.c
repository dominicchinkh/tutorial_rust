
// Compile extlib.c into a shared library (.so on Linux, .dylib on macOS, or .dll on Windows):
//   > gcc -shared -fPIC extlib.c -o libextlib.so

/*
 *  -shared
 *    Tells GCC to produce a shared object library (.so) rather than a standard 
 *    standalone executable program. Shared libraries contain compiled code that 
 *    can be loaded and executed dynamically at runtime by other programs (like 
 *    your Rust binary)
 *
 *  -fPIC (Position Independent Code)
 *    Stands for Position-Independent Code. This is a crucial flag for shared 
 *    libraries. It instructs the compiler to generate machine code that can be 
 *    loaded at any memory address in RAM, rather than a fixed memory location. 
 *    Because shared libraries can be loaded alongside many other libraries and 
 *    programs, their memory addresses cannot be predicted beforehand. PIC ensures 
 *    the code uses relative addressing so it works no matter where it lands in 
 *    memory
 */

#include <stdio.h>
#include <stdlib.h>
#include <stdarg.h>
#include <stdint.h>

//-----------------------------------------
// Callbacks from C code to Rust functions

typedef void (*rust_callback_i32)(int32_t);
rust_callback_i32 cb_i32;

int32_t register_callback_i32(rust_callback_i32 callback) {
    cb_i32 = callback;
    return 1;
}

void trigger_callback_i32() {
    cb_i32(7); // Will call callback(7) in Rust.
}

//-------------------------------------
// Targeting callbacks to Rust objects

typedef void (*rust_callback_rust)(void*, int32_t);
void* cb_target;
rust_callback_rust cb_rust;

int32_t register_callback_rust(void* callback_target, rust_callback_rust callback) {
    cb_target = callback_target;
    cb_rust = callback;
    return 1;
}

void trigger_callback_rust() {
  cb_rust(cb_target, 8); // Will call callback(&rustObject, 8) in Rust.
}

//---------------------------
// Accessing foreign globals

// A foreign global variable
int32_t FOO = 42;

// Helper to check the value from C's perspective
int32_t get_foo() {
    return FOO;
}

//--------------------
// Variadic functions

// A C variadic function that sums a variable number of integers
int32_t sum_values(int32_t count, ...) {
    va_list args;
    va_start(args, count);
    
    int32_t sum = 0;
    for (int32_t i = 0; i < count; i++) {
        // Retrieve each argument as an int32_t
        sum += va_arg(args, int32_t);
    }
    
    va_end(args);
    return sum;
}

//-----------------------------
// Representing opaque structs

// The actual definition is hidden from consumers (Opaque)
struct Foo {
    int32_t value;
};

// Allocate and initialize the struct
struct Foo* create_foo(int32_t initial_value) {
    struct Foo* f = (struct Foo*)malloc(sizeof(struct Foo));
    f->value = initial_value;
    return f;
}

// Read the value
int32_t get_foo_value(struct Foo* f) {
    if (f == NULL) return 0;
    return f->value;
}

// Modify the value
void set_foo_value(struct Foo* f, int32_t new_value) {
    if (f != NULL) {
        f->value = new_value;
    }
}

// Free the memory
void destroy_foo(struct Foo* f) {
    free(f);
}

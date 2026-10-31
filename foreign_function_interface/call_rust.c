
// At top directory (./foreign_function_interface):
//   > gcc call_rust.c -o call_rust -lforeign_function_interface  -L./target/debug
//   > LD_LIBRARY_PATH=./target/debug ./call_rust

extern void hello_from_rust();

int main(void) {
    hello_from_rust();
    return 0;
}

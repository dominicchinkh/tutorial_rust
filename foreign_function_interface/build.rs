fn main() {
    println!("cargo:rustc-link-lib=dylib=stdc++"); // This line may be unnecessary for some environments.
    println!("cargo:rustc-link-search=<YOUR SNAPPY LIBRARY PATH>");
}
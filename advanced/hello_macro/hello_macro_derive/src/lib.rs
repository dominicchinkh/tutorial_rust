// Procedural macros need to be in their own crate. Eventually, this restriction might
// be lifted. The convention for structuring crates and macro crates is as follows: For 
// a crate named foo, a custom derive procedural macro crate is called foo_derive

// The proc_macro crate is the compiler’s API that allows us to read and manipulate Rust
// code from our code
use proc_macro::TokenStream;

// The quote crate turns syn data structures back into Rust code
use quote::quote;

// The `hello_macro_derive` function will be called when a user of our library specifies 
// #[derive(HelloMacro)] on a type
#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {

    // Construct a representation of Rust code as a syntax tree that we can manipulate.
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation.
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {

    /*
     *  DeriveInput {
     *      // --snip--
     *  
     *      ident: Ident {
     *          ident: "Pancakes",
     *          span: #0 bytes(95..103)
     *      },
     *      data: Struct(
     *          DataStruct {
     *              struct_token: Struct,
     *              fields: Unit,
     *              semi_token: Some(
     *                  Semi
     *              )
     *          }
     *      )
     *  }
     */
    
    // An Ident struct instance that, when printed, will be the name of the struct
    let name = &ast.ident;

    // The quote! macro lets us define the Rust code that we want to return
    let generated = quote! {

        // quote! will replace #name with the value in the variable name
        impl HelloMacro for #name {
            fn hello_macro() {

                // `stringify!` takes a Rust expression, such as 1 + 2, and at compile 
                // time turns the expression into a string literal, such as "1 + 2"
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };

    // The compiler expects something different from the direct result of the quote! 
    // macro’s execution, so we need to convert it to a TokenStream. We do this by 
    // calling the into method, which consumes this intermediate representation and 
    // returns a value of the required TokenStream type
    generated.into()
}

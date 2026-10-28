
fn main() {

    //-------------
    // Data Layout
    //-------------

    /*
     *  Null-Pointer Optimization (NPO)
     *
     *    In Rust, certain types have bit patterns that are guaranteed never to be 
     *    valid values. These invalid bit patterns are called niches. 
     *
     *    A reference like &u16 can never be null (0x0). Rust takes advantage of this: 
     *    * Some(&u16) stores the memory address of the u16
     *    * None is represented by storing all zeros (0x0) in that exact same memory space
     *
     *    Because 0x0 can never be a valid reference, Rust doesn't need an extra byte (a 
     *    discriminant/tag) to distinguish Some from None
     */
    enum MyOption<T> {
        Some(T),
        None,
    }

    /*
     *  Why does repr(u8) suppress the NPO?
     *
     *    When you add #[repr(u8)] (or repr(C), repr(i32), etc.) to an enum with 
     *    fields, you force the compiler to follow strict structural layout rules:
     *    Allocate an explicit 1-byte integer tag (u8) to store the variant index 
     *    (0 for Some, 1 for None). Add alignment padding so the payload field (&u16) 
     *    aligns properly in memory. On 64-bit platforms, an 8-byte pointer must be 
     *    aligned to an 8-byte boundary
     */
    #[repr(u8)]
    enum MyReprOption<T> {
        Some(T),
        None,
    }

    {
        assert_eq!(8, size_of::<MyOption<&u16>>());
        assert_eq!(16, size_of::<MyReprOption<&u16>>());
    }
}

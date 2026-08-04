/// Adds one to the value behind the given raw pointer.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// // We must use an unsafe block to call an unsafe function
/// let answer = unsafe { cargo::add_one(&arg) };
///
/// assert_eq!(Ok(6), answer);
/// ```
///
/// # Panics
///
/// This function will panic if the dereferenced value is `0`.
///
/// # Errors
///
/// This function will return an `Err` if the dereferenced value is `i32::MAX`, 
/// as adding one would result in an integer overflow.
///
/// # Safety
///
/// The `ptr` parameter must be a valid, aligned, and non-null pointer to an 
/// initialized `i32` in memory. Calling this function with a dangling or null 
/// pointer results in undefined behavior.
///
pub unsafe fn add_one(ptr: *const i32) -> Result<i32, String> {
    // SAFETY: The caller's contract guarantees `ptr` is valid to dereference.
    // https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-op-in-unsafe-fn.html
    let val = unsafe { *ptr };

    // Triggers the scenario described in # Panics
    if val == 0 {
        panic!("Cannot add one to zero in this specific implementation!");
    }

    // Triggers the scenario described in # Errors
    if val == i32::MAX {
        return Err(String::from("Overflow error: value is at maximum capacity"));
    }

    Ok(val + 1)
}

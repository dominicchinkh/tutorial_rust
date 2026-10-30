use std::alloc::{self, Layout};
use std::marker::PhantomData;
use std::mem::{self, ManuallyDrop};
use std::ops::{Deref, DerefMut};
use std::ptr::{self, NonNull};

struct RawVec<T> {

    // NonNull<T>, another wrapper around a raw pointer, gives us covariant over T and 
    // is declared to never be null. NonNull<T> is Send/Sync if T is Send/Sync

    ptr: NonNull<T>,
    cap: usize,
}

unsafe impl<T: Send> Send for RawVec<T> {}
unsafe impl<T: Sync> Sync for RawVec<T> {}

impl<T> RawVec<T> {
    fn new() -> Self {
        /*
         * By setting cap = usize::MAX right from the start, the vector effectively
         * tells itself: "We already have infinite capacity; we never need to allocate
         * or grow."
         *
         * When you call .push(), checks like len == cap will naturally evaluate as false
         * (since len can never reach usize::MAX), meaning it will never trigger grow()
         *
         * The pointer remains NonNull::dangling() forever. alloc::alloc, alloc::realloc, 
         * and alloc::dealloc are never called for ZST vectors
         *
         */
        
        // assert!(mem::size_of::<T>() != 0, "TODO: implement ZST support");
        let cap = if mem::size_of::<T>() == 0 { usize::MAX } else { 0 };

        RawVec {
            ptr: NonNull::dangling(),
            cap,
        }
    }

    fn grow(&mut self) {

        // Since we set the capacity to usize::MAX when T has size 0,
        // getting to here necessarily means the Vec is overfull.
        assert!(mem::size_of::<T>() != 0, "capacity overflow");

        /*
         * Limitation: isize::MAX Bytes
         *
         *   Array indices are unsigned (usize), meaning they can go up to usize::MAX
         *   (e.g., 18 quintillion on a 64-bit system)
         *
         *   Pointer offsets (GEP / ptr::offset) take a signed integer (isize), meaning 
         *   their maximum positive value is isize::MAX (roughly half of usize::MAX)
         *
         *   Because of this, any allocation can normally contain at most isize::MAX 
         *   elements so that you can legally point to the last element using an offset
         *
         *   For most large types (like a u16 or u64), you will naturally run out of RAM 
         *   long before you reach isize::MAX elements, because the total size in bytes 
         *   would exceed system memory
         *
         *   However, for zero-sized types (ZSTs like ()) or single-byte types (u8), you 
         *   can theoretically allocate a huge number of items without instantly exhausting 
         *   physical memory
         *
         *   To prevent these dangerous edge cases where code safely creates an array of 
         *   elements (thinking it's fine) but later safely or unsafely reinterprets that 
         *   same memory as raw bytes (where the byte limit is breached), std enforces a 
         *   strict rule: no allocation may ever exceed isize::MAX bytes in total size
         */

        // Layout::array::<T>(n) is a utility function used to calculate the memory 
        // requirements (size and alignment) needed to store an array of n elements 
        // of type T

        let (new_cap, new_layout) = if self.cap == 0 {
            (1, Layout::array::<T>(1))
        } else {
            // This can't overflow because we ensure self.cap <= isize::MAX.
            let new_cap = 2 * self.cap;

            // If the resulting byte size would exceed isize::MAX, Layout::array 
            // refuses it and returns an error (Err)
            (new_cap, Layout::array::<T>(new_cap))
        };

        // `Layout::array` checks that the number of bytes allocated is
        // in 1..=isize::MAX and will error otherwise.  An allocation of
        // 0 bytes isn't possible thanks to the above condition.
        let new_layout = new_layout.expect("Allocation too large");

        let new_ptr = if self.cap == 0 {
            unsafe { alloc::alloc(new_layout) }
        } else {
            let old_layout = Layout::array::<T>(self.cap).unwrap();
            let old_ptr = self.ptr.as_ptr() as *mut u8;
            unsafe { alloc::realloc(old_ptr, old_layout, new_layout.size()) }
        };

        // If allocation fails, `new_ptr` will be null, in which case we abort.
        self.ptr = match NonNull::new(new_ptr as *mut T) {
            Some(p) => p,
            None => alloc::handle_alloc_error(new_layout),
        };
        self.cap = new_cap;
    }
}

impl<T> Drop for RawVec<T> {
    fn drop(&mut self) {
        println!("RawVec<T> Drop called, deallocating memory");
        
        if self.cap != 0 && std::mem::size_of::<T>() > 0 {
            let layout = Layout::array::<T>(self.cap).unwrap();
            unsafe {
                std::alloc::dealloc(self.ptr.as_ptr() as *mut _, layout);
            }
        }
    }
}

pub struct Vec<T> {
    buf: RawVec<T>,
    len: usize,
}

unsafe impl<T: Send> Send for Vec<T> {}
unsafe impl<T: Sync> Sync for Vec<T> {}

impl<T> Vec<T> {
    fn ptr(&self) -> *mut T {
        self.buf.ptr.as_ptr()
    }

    fn cap(&self) -> usize {
        self.buf.cap
    }

    pub fn new() -> Self {
        Vec {
            buf: RawVec::new(),
            len: 0,
        }
    }

    pub fn push(&mut self, elem: T) {
        if self.len == self.cap() { 
            self.buf.grow(); 
        }

        unsafe {
            // We can’t just index to the memory and dereference it, because that will 
            // evaluate the memory as a valid instance of T. At worst, it’s truly 
            // uninitialized memory from the allocator. At best it’s the bits of some 
            // old value we popped off

            // ptr::write blindly overwrites the target address with the bits of the 
            // value we provide. No evaluation involved

            ptr::write(self.ptr().add(self.len), elem);
        }

        // Can't fail, we'll OOM first.
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            unsafe {

                // Although this time the index we want to access is initialized, Rust 
                // won’t just let us dereference the location of memory to move the 
                // value out, because that would leave the memory uninitialized

                // ptr::read just copies out the bits from the target address and 
                // interprets it as a value of type T

                Some(ptr::read(self.ptr().add(self.len)))
            }
        }
    }

    pub fn insert(&mut self, index: usize, elem: T) {

        // Note: `<=` because it's valid to insert after everything
        // which would be equivalent to push.
        assert!(index <= self.len, "index out of bounds");

        if self.len == self.cap() { 
            self.buf.grow(); 
        }

        // If the allocation fails during grow(), the program does not continue; 
        // it immediately aborts or panics

        unsafe {
            // ptr::copy(src, dest, len): "copy from src to dest len elems"
            ptr::copy(
                self.ptr().add(index),
                self.ptr().add(index + 1),
                self.len - index,
            );
            ptr::write(self.ptr().add(index), elem);
        }

        self.len += 1;
    }

    pub fn remove(&mut self, index: usize) -> T {

        // Note: `<` because it's *not* valid to remove after everything
        assert!(index < self.len, "index out of bounds");

        unsafe {
            self.len -= 1;
            let result = ptr::read(self.ptr().add(index));
            ptr::copy(
                self.ptr().add(index + 1),
                self.ptr().add(index),
                self.len - index,
            );
            result
        }
    }

    pub fn drain(&mut self) -> Drain<'_, T> {
        let iter = unsafe { 
            RawValIter::new(&self) 
        };

        /*
         *  When you call vec.drain(), you are transferring the responsibility of
         *  owning, yielding, and eventually dropping the vector's elements over 
         *  to the Drain iterator.
         *
         *  If self.len remained unchanged (e.g., 5), the original Vec would still 
         *  believe it contains 5 valid elements.
         *
         *  If the user were to drop the original Vec while Drain is active or if 
         *  something went wrong and Drain was leaked/forgotten, the Vec would 
         *  eventually run its own Drop implementation. It would loop through indices
         *  0 to 4 and try to drop all those elements a second time.
         *
         *  Dropping the same memory/value twice (double-drop) leads to catastrophic 
         *  undefined behavior (memory corruption, crashes, or security vulnerabilities).
         */

        self.len = 0;

        Drain {
            iter,
            vec: PhantomData,
        }
    }
}

impl<T> Drop for Vec<T> {
    fn drop(&mut self) {
        if self.cap() != 0 {
            while let Some(_) = self.pop() {}
            // deallocation is handled by RawVec
        }
    }
}

impl<T> Deref for Vec<T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        unsafe {
            std::slice::from_raw_parts(self.ptr(), self.len)
        }
    }
}

impl<T> DerefMut for Vec<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe {
            std::slice::from_raw_parts_mut(self.ptr(), self.len)
        }
    }
}

impl<T> IntoIterator for Vec<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> IntoIter<T> {
        unsafe {
            let iter = RawValIter::new(&self);
            let buf = ptr::read(&self.buf);

            mem::forget(self);

            IntoIter {
                iter,
                _buf: buf,
            }
        }
    }
}

struct RawValIter<T> {
    start: *const T,
    end: *const T,
}

impl<T> RawValIter<T> {

    /*
     *  Normally, standard library iterators (like std::slice::Iter<'a, T>) carry a
     *  lifetime parameter ('a).
     *
     *  That lifetime ties the iterator to the slice or vector it was created from.
     *
     *  The borrow checker uses this lifetime to guarantee that you can never use the 
     *  iterator after the underlying vector has been dropped or mutated.
     *
     *  RawValIter<T>, on the other hand, deliberately drops the lifetime. It only 
     *  holds raw pointers (start: *const T, end: *const T). Because it lacks a lifetime, 
     *  Rust's borrow checker cannot automatically verify that RawValIter is safe to use. 
     *  Anyone could theoretically hold onto a RawValIter long after the memory has been 
     *  deallocated. 
     *
     *  To prevent this, we mark the new function as unsafe, placing the burden of safety
     *  entirely on the programmer.
     *
     *  Because RawValIter is completely hidden inside the module and not exposed as part 
     *  of the crate's public API, external users cannot construct it or misuse it directly. 
     *  The unsafety is safely sealed behind public-facing iterators like IntoIter and Drain,
     *  which manage the raw pointers correctly under the hood.
     *
     */

    unsafe fn new(slice: &[T]) -> Self {
        RawValIter {
            start: slice.as_ptr(),
            end: if mem::size_of::<T>() == 0 {
                // Cast the pointers to integers, increment, and then cast them back:
                ((slice.as_ptr() as usize) + slice.len()) as *const _

            } else if slice.len() == 0 {
                // if `len = 0`, then this is not actually allocated memory.
                // Need to avoid offsetting because that will give wrong
                // information to LLVM via GEP.
                slice.as_ptr()

            } else {
                unsafe {
                    slice.as_ptr().add(slice.len())
                }
            }
        }
    }
}

impl<T> Iterator for RawValIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.start == self.end {
            None
        } else {
            unsafe {
                if mem::size_of::<T>() == 0 {
                    self.start = (self.start as usize + 1) as *const _;
                    Some(ptr::read(NonNull::<T>::dangling().as_ptr()))

                } else {
                    let old_ptr = self.start;
                    self.start = self.start.offset(1);
                    Some(ptr::read(old_ptr))
                }
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let elem_size = mem::size_of::<T>();
        let len = (self.end as usize - self.start as usize)
                    / if elem_size == 0 { 1 } else { elem_size };

        /*
         *  In Rust, the Iterator::size_hint() method returns a bound on the remaining
         *  number of elements in the iterator, formatted as a tuple: (lower_bound, 
         *  Option<upper_bound>)
         *
         *  lower_bound (usize): The absolute minimum number of elements the iterator 
         *  is guaranteed to yield
         *
         *  upper_bound (Option<usize>): An optional maximum bound
         *
         *    * If the iterator doesn't know its exact upper limit (e.g., an infinite 
         *      sequence or a filtering iterator), it returns None.
         *
         *    * If the iterator knows the exact, precise number of elements remaining, 
         *      it returns Some(n) where n matches the lower bound.
         */
        (len, Some(len))
    }
}

impl<T> DoubleEndedIterator for RawValIter<T> {
    fn next_back(&mut self) -> Option<T> {
        if self.start == self.end {
            None
        } else {
            unsafe {
                if mem::size_of::<T>() == 0 {
                    self.end = (self.end as usize - 1) as *const _;
                    Some(ptr::read(NonNull::<T>::dangling().as_ptr()))
                } else {
                    self.end = self.end.offset(-1);
                    Some(ptr::read(self.end))
                }
            }
        }
    }
}

/*
 *  If RawValIter used safe references instead of raw pointers, for example, if it 
 *  held a slice reference like &'a [T] tied to _buf, then IntoIter would become a 
 *  self-referential struct:
 *
 *    * Field 2 (iter) would contain a reference borrowing Field 1 (_buf).
 *    * If you moved IntoIter anywhere else in memory, that reference would instantly 
 *      become invalid.
 */
pub struct IntoIter<T> {
    _buf: RawVec<T>, // we don't actually care about this. Just need it to live.
    iter: RawValIter<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> { 
        self.iter.next() 
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint() 
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<T> { 
        self.iter.next_back() 
    }
}

impl<T> Drop for IntoIter<T> {
    fn drop(&mut self) {
        for _ in &mut *self {}
    }
}

pub struct Drain<'a, T: 'a> {
    vec: PhantomData<&'a mut Vec<T>>,
    iter: RawValIter<T>,
}

impl<'a, T> Iterator for Drain<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<T> { 
        self.iter.next() 
    }

    fn size_hint(&self) -> (usize, Option<usize>) { 
        self.iter.size_hint() 
    }
}

impl<'a, T> DoubleEndedIterator for Drain<'a, T> {
    fn next_back(&mut self) -> Option<T> { 
        self.iter.next_back() 
    }
}

impl<'a, T> Drop for Drain<'a, T> {
    fn drop(&mut self) {
        for _ in &mut *self {}
    }
}

fn main() {
    println!("--- Testing Vec<T>::new() and push() ---");
    let mut vec = Vec::new();
    vec.push(10);
    vec.push(20);
    vec.push(30);
    vec.push(40);
    vec.push(50);

    println!("Vector contents (via Deref): {:?}", &vec[..]);
    println!("Length: {}, Capacity: {}", vec.len(), vec.cap());

    println!("\n--- Testing insert() and remove() ---");
    vec.insert(2, 99); // Insert 99 at index 2
    println!("After insert(2, 99): {:?}", &vec[..]);

    let removed = vec.remove(1); // Remove item at index 1 (20)
    println!("Removed element: {}", removed);
    println!("After remove(1): {:?}", &vec[..]);

    println!("\n--- Testing pop() ---");
    while let Some(val) = vec.pop() {
        print!("Popped: {} | ", val);
    }
    println!("\nLength after popping all: {}", vec.len());

    println!("\n--- Testing IntoIterator (for-loop consumption) ---");
    let mut vec2 = Vec::new();
    vec2.push(100);
    vec2.push(200);
    vec2.push(300);

    for val in vec2 {
        println!("Consumed via into_iter: {}", val);
    }
    // vec2 is now consumed and cannot be used here.

    println!("\n--- Testing drain() ---");
    let mut vec3 = Vec::new();
    vec3.push(1);
    vec3.push(2);
    vec3.push(3);

    {
        let drained = vec3.drain();
        println!("Drained iterator values:");
        for v in drained {
            print!("{} ", v);
        }
        println!();
    }
    println!("Vec3 length after drain (should be 0): {}", vec3.len());

    println!("\n--- End of main (Drop messages will follow) ---");
}

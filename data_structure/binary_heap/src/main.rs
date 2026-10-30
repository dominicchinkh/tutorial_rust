use std::ptr;

#[inline]
fn parent(index: usize) -> usize {
    (index - 1) / 2
}

struct Hole<'a, T: 'a> {
    // A mutable reference to a slice of T (where the slice lives for lifetime 'a)
    data: &'a mut [T],

    /// `elt` is always `Some` from new until drop.
    elt: Option<T>,
    pos: usize,
}

impl<'a, T> Hole<'a, T> {
    fn new(data: &'a mut [T], pos: usize) -> Self {
        unsafe {
            let elt = ptr::read(&data[pos]);
            
            Hole {
                data,
                elt: Some(elt),
                pos,
            }
        }
    }

    fn pos(&self) -> usize { self.pos }

    fn removed(&self) -> &T { self.elt.as_ref().unwrap() }

    fn get(&self, index: usize) -> &T { &self.data[index] }

    // This unsafe helper function moves an element from index to self.pos within an
    // internal slice/buffer
    unsafe fn move_to(&mut self, index: usize) {

        // Obtains a raw, immutable pointer (*const T) to the element at self.data[index]
        let index_ptr: *const _ = &self.data[index];

        // Obtains a raw mutable pointer (*mut T) to the current "hole" location (self.pos)
        let hole_ptr = &mut self.data[self.pos];

        // Performs a raw bitwise copy (memcpy) of 1 element (sizeof::<T>() bytes) from 
        // index_ptr to hole_ptr

        // Explicit unsafe block required in Rust 2024 Edition:
        unsafe {
            ptr::copy_nonoverlapping(index_ptr, hole_ptr, 1);
        }

        // Updates the internal cursor position self.pos to index - "The New Hole"
        self.pos = index;
    }
}

impl<'a, T> Drop for Hole<'a, T> {
    fn drop(&mut self) {

        // Fill the hole again
        unsafe {
            let pos = self.pos;
            ptr::write(&mut self.data[pos], self.elt.take().unwrap());
        }
    }
}

pub struct BinaryHeap<T> {
    data: Vec<T>,
}

impl<T: Ord> BinaryHeap<T> {
    pub fn new() -> Self {
        BinaryHeap { data: Vec::new() }
    }

    pub fn push(&mut self, item: T) {
        let old_len = self.data.len();
        self.data.push(item);
        self.sift_up(old_len);
    }

    // Bubble the newly added item up the tree until the max-heap invariant 
    // (parent >= child) is restored
    fn sift_up(&mut self, pos: usize) {

        // // The root node has no parent to compare against
        if pos == 0 { return; }

        unsafe {
            let mut hole = Hole::new(&mut self.data, pos);

            while hole.pos() != 0 {
                let parent_idx = parent(hole.pos());

                // Max-Heap condition: if new element <= parent, heap invariant holds
                if hole.removed() <= hole.get(parent_idx) { 
                    break; 
                }

                // Move parent down into the hole, shifting the hole up to parent_idx
                hole.move_to(parent_idx);
            }

            // `hole` drops here, placing `hole.elt` safely back into `hole.pos`
        }
    }
}

fn main() {
    let mut heap = BinaryHeap::new();
    heap.push(10);
    heap.push(30);
    heap.push(20);
    println!("BinaryHeap created and sift_up executed safely!");
}

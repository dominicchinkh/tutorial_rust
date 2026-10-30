use std::mem;

/// A custom mutable slice iterator.
pub struct IterMut<'a, T>(&'a mut [T]);

impl<'a, T> IterMut<'a, T> {
    /// Creates a new `IterMut` from a mutable slice.
    pub fn new(slice: &'a mut [T]) -> Self {
        IterMut(slice)
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        
        // Replace self.0 with an empty slice (&mut []) to temporarily take ownership
        let slice = mem::take(&mut self.0);

        if slice.is_empty() {
            return None;
        }

        // Split off the first element: l contains index 0, r contains 1..
        let (l, r) = slice.split_at_mut(1);
        self.0 = r;
        l.get_mut(0)
    }
}

impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {

        let slice = mem::take(&mut self.0);
        
        if slice.is_empty() {
            return None;
        }

        // Split off the last element: l contains 0..new_len, r contains the last element
        let new_len = slice.len() - 1;
        let (l, r) = slice.split_at_mut(new_len);
        self.0 = l;
        r.get_mut(0)
    }
}

fn main() {
    let mut numbers = vec![10, 20, 30, 40, 50];

    println!("Original slice: {:?}", numbers);

    // Instantiate custom iterator
    let mut iter = IterMut::new(&mut numbers);

    // Pop and modify from the front (Iterator)
    if let Some(first) = iter.next() {
        *first += 5; // 10 -> 15
    }

    // Pop and modify from the back (DoubleEndedIterator)
    if let Some(last) = iter.next_back() {
        *last *= 2; // 50 -> 100
    }

    // Modify remaining elements in a loop
    for item in iter {
        *item += 100; // 20 -> 120, 30 -> 130, 40 -> 140
    }

    println!("Modified slice: {:?}", numbers);
}

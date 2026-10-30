type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

pub struct LinkedList<T> {
    head: Link<T>,
}

pub struct IterMut<'a, T: 'a>(Option<&'a mut Node<T>>);

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    // Using IterMut<'_, T> tells the reader and the compiler:
    //   "I know IterMut expects a lifetime parameter."
    //   "I want Rust to connect that lifetime to &mut self automatically using lifetime
    //    elision."

    fn iter_mut(&mut self) -> IterMut<'_, T> {

        // self.head.as_mut() turns Option<Box<Node<T>>> into Option<&mut Box<Node<T>>>

        // .map(|node| &mut **node) dereferences the Box twice (first the &mut, then the 
        // Box) and re-borrows it to obtain a &mut Node<T>.

        IterMut(self.head.as_mut().map(|node| &mut **node))
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {

        // take() pulls the current &'a mut Node<T> out of the iterator, leaving None 
        // temporarily in self.0. This gives the iterator exclusive ownership of the 
        // node reference for the current step

        self.0.take().map(|node| {

            // self.0 = node.next.as_mut()... advances the iterator's internal pointer
            // to the next node (node.next)

            // Finally, it returns a mutable reference to the current element (&mut 
            // node.elem)

            self.0 = node.next.as_mut().map(|node| &mut **node);
            &mut node.elem
        })
    }
}

fn main() {
    let mut list = LinkedList::new();

    // 1. Push elements onto the list
    list.push(10);
    list.push(20);
    list.push(30);

    println!("--- Modifying items via iter_mut ---");

    // 2. Iterate mutably and double every value
    for elem in list.iter_mut() {
        *elem *= 2;
    }

    println!("--- Reading updated items ---");

    // 3. Iterate mutably again to print the modified values
    for elem in list.iter_mut() {
        println!("Value: {}", elem);
    }
}

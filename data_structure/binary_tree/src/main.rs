use std::collections::VecDeque;

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    left: Link<T>,
    right: Link<T>,
}

pub struct Tree<T> {
    root: Link<T>,
}

struct NodeIterMut<'a, T: 'a> {
    elem: Option<&'a mut T>,
    left: Option<&'a mut Node<T>>,
    right: Option<&'a mut Node<T>>,
}

enum State<'a, T: 'a> {
    Elem(&'a mut T),
    Node(&'a mut Node<T>),
}

pub struct IterMut<'a, T: 'a>(VecDeque<NodeIterMut<'a, T>>);

impl<T: Ord> Tree<T> {

    pub fn new() -> Self {
        Tree { root: None }
    }

    pub fn insert(&mut self, elem: T) {
        Self::insert_rec(&mut self.root, elem);
    }

    fn insert_rec(link: &mut Link<T>, elem: T) {
        match link {
            Some(node) => {
                if elem < node.elem {
                    Self::insert_rec(&mut node.left, elem);
                } else {
                    Self::insert_rec(&mut node.right, elem);
                }
            }
            None => {
                *link = Some(Box::new(Node {
                    elem,
                    left: None,
                    right: None,
                }));
            }
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        let mut deque = VecDeque::new();
        if let Some(root) = self.root.as_mut() {
            deque.push_front(root.iter_mut());
        }
        IterMut(deque)
    }
}

impl<T> Node<T> {
    pub fn iter_mut(&mut self) -> NodeIterMut<'_, T> {
        NodeIterMut {
            elem: Some(&mut self.elem),
            left: self.left.as_deref_mut(),
            right: self.right.as_deref_mut(),
        }
    }
}

impl<'a, T> Iterator for NodeIterMut<'a, T> {
    type Item = State<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.left.take().map(State::Node).or_else(|| {
            self.elem
                .take()
                .map(State::Elem)
                .or_else(|| self.right.take().map(State::Node))
        })
    }
}

impl<'a, T> DoubleEndedIterator for NodeIterMut<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.right.take().map(State::Node).or_else(|| {
            self.elem
                .take()
                .map(State::Elem)
                .or_else(|| self.left.take().map(State::Node))
        })
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.0.front_mut().and_then(Iterator::next) {
                Some(State::Elem(elem)) => return Some(elem),
                Some(State::Node(node)) => self.0.push_front(node.iter_mut()),
                None => {
                    self.0.pop_front()?;
                }
            }
        }
    }
}

impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            match self.0.back_mut().and_then(DoubleEndedIterator::next_back) {
                Some(State::Elem(elem)) => return Some(elem),
                Some(State::Node(node)) => self.0.push_back(node.iter_mut()),
                None => {
                    self.0.pop_back()?;
                }
            }
        }
    }
}

fn main() {
    let mut tree = Tree::new();

    // Constructing a tree:
    //         10
    //        /  \
    //       5    15
    //      / \
    //     2   7
    for val in [10, 5, 15, 2, 7] {
        tree.insert(val);
    }

    println!("--- Initial In-Order Traversal ---");
    for val in tree.iter_mut() {
        print!("{val} ");
    }
    println!();

    println!("\n--- Modifying values in-place (*val *= 10) ---");
    for val in tree.iter_mut() {
        *val *= 10;
    }

    println!("\n--- Forward Traversal (after modification) ---");
    for val in tree.iter_mut() {
        print!("{val} ");
    }
    println!();

    println!("\n--- Reverse Traversal (DoubleEndedIterator) ---");
    for val in tree.iter_mut().rev() {
        print!("{val} ");
    }
    println!();
}

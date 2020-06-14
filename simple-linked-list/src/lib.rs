mod node;

use crate::node::Node;
use std::iter::FromIterator;

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> Default for SimpleLinkedList<T> {
    fn default() -> Self {
        SimpleLinkedList { head: None }
    }
}

impl<T: Copy> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        let mut cur = self.head.as_ref();
        let mut count = 0;
        while let Some(x) = cur {
            cur = x.next.as_ref();
            count += 1;
        }
        count
    }

    pub fn push(&mut self, element: T) {
        self.head = Some(Box::new(Node::new(element, self.head.take())));
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.head.is_some() {
            let node = self.head.take().unwrap();
            self.head = node.next;
            return Some(node.data);
        }
        None
    }

    pub fn peek(&self) -> Option<&T> {
        if self.head.is_some() {
            let node = &self.head.as_ref().unwrap();
            return Some(&node.data);
        }
        None
    }

    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut list = Self::default();
        let mut node = self.head;

        while node.is_some() {
            let cur = node.take().unwrap();
            list.push(cur.data);
            node = cur.next;
        }

        list
    }
}

impl<T: Copy> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = Self::default();
        iter.into_iter().for_each(|item| list.push(item));
        list
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T: Copy> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(mut self) -> Vec<T> {
        let mut vec = Vec::new();
        while let Some(x) = &self.pop() {
            vec.push(*x);
        }
        vec.into_iter().rev().collect()
    }
}

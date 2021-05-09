use std::fmt::{Debug, Display};
use std::ops::{Deref, Index};
use std::option::Option::Some;

//https://rust-unofficial.github.io/too-many-lists/
type Link<T> = Option<Box<Node<T>>>;

pub struct List<T> {
    head: Link<T>,
}

struct Node<T> {
    element: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push_front(&mut self, element: T) {
        let new_node = Node {
            element,
            next: self.head.take(),
        };

        self.head = Some(Box::new(new_node));
    }

    pub fn push_back(&mut self, element: T) {
        let new_node = Some(Box::new(Node {
            element,
            next: None,
        }));

        match self.head {
            Some(ref mut head) => {
                let mut node = head;

                while let Some(ref mut next_node) = node.next {
                    node = next_node;
                }
                node.next = new_node;
            }
            None => {
                self.head = new_node;
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.element
        })
    }

    pub fn pop_back(&mut self) -> Option<T> {
        let mut current_node = &mut self.head;

        loop {
            let next = current_node.as_ref()?.next.as_ref();

            if next.is_none() {
                return current_node.take().map(|node| node.element);
            }
            current_node = &mut current_node.as_mut()?.next;
        }
    }

    pub fn peek_front(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.element)
    }

    pub fn peek_front_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.element)
    }

    pub fn peek_back(&self) -> Option<&T> {
        self.iter().last()
    }

    pub fn peek_back_mut(&mut self) -> Option<&mut T> {
        self.iter_mut().last()
    }
}

impl<T> List<T> {
    pub fn reverse(&mut self) {
        let mut current_node = self.head.take();
        let mut prev_node: Option<Box<Node<T>>> = None;

        while let Some(ref mut boxed_node) = current_node {
            let next_node = boxed_node.next.take();
            boxed_node.next = prev_node.take();

            prev_node = current_node;
            current_node = next_node;
        }

        self.head = prev_node;
    }

    pub fn remove_by_index(&mut self, index: usize) -> Option<T> {
        let mut current_index: usize = 0;
        let mut current_node = &mut self.head;

        while current_index != index {
            current_node = match current_node {
                Some(node) => &mut node.next,
                None => {
                    return None;
                }
            };
            current_index += 1;
        }

        current_node.take().map(|node| *node).map(|node| {
            *current_node = node.next;
            node.element
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();

        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

pub struct IntoIter<T>(List<T>);

impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_deref(),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.element
        })
    }
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.element
        })
    }
}

#[cfg(test)]
mod test {
    use super::List;
    use std::ops::Deref;

    #[test]
    fn basics() {
        let mut list = List::new();

        assert_eq!(list.pop_front(), None);

        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));

        list.push_front(4);
        list.push_front(5);

        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), Some(4));

        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn push_back() {
        let mut list = List::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(3));
    }

    #[test]
    fn pop_back() {
        let mut list = List::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn peek() {
        let mut list = List::new();

        assert_eq!(list.peek_front(), None);
        assert_eq!(list.peek_front_mut(), None);

        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(list.peek_front(), Some(&3));
        assert_eq!(list.peek_front_mut(), Some(&mut 3));

        assert_eq!(list.peek_back(), Some(&1));
        assert_eq!(list.peek_back_mut(), Some(&mut 1));

        list.peek_front_mut().map(|value| *value = 42);
        list.peek_back_mut().map(|value| *value = 32);

        assert_eq!(list.peek_front(), Some(&42));
        assert_eq!(list.pop_front(), Some(42));

        assert_eq!(list.peek_back(), Some(&32));
        assert_eq!(list.peek_back_mut(), Some(&mut 32));
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();

        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        let mut iter = list.into_iter();

        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        list.iter_mut()
            .map(|value| {
                *value *= 4;
            })
            .for_each(drop);

        let mut iter = list.iter_mut();

        assert_eq!(iter.next(), Some(&mut 12));
        assert_eq!(iter.next(), Some(&mut 8));
        assert_eq!(iter.next(), Some(&mut 4));
    }

    #[test]
    fn reverse() {
        let mut list = List::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));

        list.reverse();

        let mut iter = list.into_iter();

        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn remove_by_index() {
        let mut list = List::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        let v = list.remove_by_index(1);
        assert_eq!(v, Some(2));

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&1));
    }
}

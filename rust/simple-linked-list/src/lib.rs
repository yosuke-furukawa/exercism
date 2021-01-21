use std::iter::FromIterator;

#[derive(PartialEq, Clone)]
pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
    length: usize,
}

#[derive(PartialEq, Clone)]
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList {
            head: None,
            length: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn push(&mut self, _element: T) {
        let node = Node {
            data: _element,
            next: self.head.take(),
        };
        self.head = Some(Box::new(node));
        self.length += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        self.length -= 1;
        self.head.take().map(|n| {
            self.head = n.next;
            n.data
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|n| &n.data)
    }

    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut result = SimpleLinkedList::new();
        for v in self {
            result.push(v);
        }
        result
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        let mut sl = SimpleLinkedList::new();
        for i in _iter {
            sl.push(i);
        }
        sl
    }
}

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut vec = vec![];
        for v in self {
            vec.push(v);
        }
        vec.reverse();
        vec
    }
}

impl<T> Iterator for SimpleLinkedList<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
    }
}

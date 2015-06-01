use std::sync::Arc;
use std::fmt;

pub struct List<A> {
    node: Arc<Node<A>>
}

enum Node<A> {
    Cons(A, Arc<Node<A>>),
    Nil
}

impl<A> List<A> {
    pub fn prepend(&self, next: A) -> List<A> {
        List { node: Arc::new(Node::Cons(next, self.node.clone())) }
    }

    pub fn empty() -> List<A> {
        List { node: Arc::new(Node::Nil) }
    }

    pub fn is_empty(&self) -> bool {
        self.node.is_empty()
    }

    pub fn length(&self) -> u32 {
        self.node.length()
    }

    pub fn head(&self) -> Option<&A> {
        match *self.node {
            Node::Nil => None,
            Node::Cons(ref head, _) => Some(head)
        }
    }

    pub fn tail(&self) -> Option<List<A>> {
        match *self.node {
            Node::Nil => None,
            Node::Cons(_, ref rest) => Some(List { node: rest.clone() })
        }
    }

    pub fn iter <'a> (&'a self) -> ListIterator<'a, A> {
        ListIterator { node: &self.node }
    }
}

impl<A> Node<A> {
    fn length(&self) -> u32 {
        let mut cur = self;
        let mut length = 0;

        while !cur.is_empty() {
            length = length + 1;
            cur = &cur.tail();
        }

        length
    }

    fn is_empty(&self) -> bool {
        match *self {
            Node::Nil => true,
            Node::Cons(_, _) => false
        }
    }

    fn tail(&self) -> &Node<A> {
        match *self {
            Node::Cons(_, ref rest) => rest,
            Node::Nil => panic!("don't do that!")
        }
    }
}

pub struct ListIterator<'a, A:'a> {
    node: &'a Node<A>
}

impl<'a, A> Iterator for ListIterator<'a, A> {
    type Item = &'a A;

    fn next(&mut self) -> Option<&'a A> {
        match *self.node {
            Node::Nil => None,
            Node::Cons(ref next, ref rest) => {
                self.node = rest;
                Some(next)
            }
        }
    }
}

impl<'a, A> IntoIterator for &'a List<A> {
    type Item = &'a A;
    type IntoIter = ListIterator<'a, A>;

    fn into_iter(self) -> ListIterator<'a, A> {
        self.iter()
    }
}

impl<A: fmt::Display> fmt::Display for List<A> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut first = true;
        try!(write!(f, "List("));

        for elem in self.iter() {
            if first {
                first = false;
                try!(write!(f, "{}", elem));
            } else {
                try!(write!(f, ", {}", elem));
            }
        }

        write!(f, ")")
    }
}

impl<A: fmt::Debug> fmt::Debug for List<A> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut first = true;
        try!(write!(f, "List("));

        for elem in self.iter() {
            if first {
                first = false;
                try!(write!(f, "{:?}", elem));
            } else {
                try!(write!(f, ", {:?}", elem));
            }
        }

        write!(f, ")")
    }
}

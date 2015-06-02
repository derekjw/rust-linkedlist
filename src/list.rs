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
    pub fn prepend(&self, head: A) -> List<A> {
        List { node: Arc::new(Node::Cons(head, self.node.clone())) }
    }
 
    pub fn empty() -> List<A> {
        List { node: Arc::new(Node::Nil) }
    }

    pub fn is_empty(&self) -> bool {
        match *self.node {
            Node::Nil => true,
            Node::Cons(_, _) => false
        }
    }
 
    pub fn len(&self) -> usize {
        self.iter().count()
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
            Node::Cons(_, ref tail) => Some(List { node: tail.clone() })
        }
    }
 
    pub fn iter <'a> (&'a self) -> ListIterator<'a, A> {
        ListIterator { node: &self.node }
    }
 
    pub fn reverse(&self) -> List<&A> {
        self.iter().fold(List::empty(), |list, elem| list.prepend(elem))
    }

    pub fn head_and_tail(&self) -> Option<(&A, List<A>)> {
        match *self.node {
            Node::Nil => None,
            Node::Cons(ref head, ref tail) => Some((head, List { node: tail.clone() }))
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
            Node::Cons(ref next, ref tail) => {
                self.node = tail;
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
        let mut iter = self.iter();
 
        let first = iter
            .next()
            .iter()
            .fold(write!(f, "List("), |result, elem| result.and(write!(f, "{}", elem)));
 
        iter.fold(first, |result, elem| result.and(write!(f, ", {}", elem)))
            .and(write!(f, ")"))
    }
}
 
impl<A: fmt::Debug> fmt::Debug for List<A> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut iter = self.iter();
 
        let first = iter
            .next()
            .iter()
            .fold(write!(f, "List("), |result, elem| result.and(write!(f, "{:?}", elem)));
 
        iter.fold(first, |result, elem| result.and(write!(f, ", {:?}", elem)))
            .and(write!(f, ")"))
    }
}
 
impl<A> Default for List<A> {
    fn default() -> List<A> {
        List::empty()
    }
}
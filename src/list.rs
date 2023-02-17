use core::{iter::FusedIterator, mem::replace, ptr::NonNull};

use crate::node_mut::NodeMut;

// TODO: Make self.0 pub?
pub struct StackListMut<'a, 'b, T>(Option<&'b mut NodeMut<'a, T>>);

impl<'a, 'b, T> StackListMut<'a, 'b, T> {
    pub(crate) fn new(top_node: Option<&'b mut NodeMut<'a, T>>) -> Self {
        StackListMut(top_node)
    }

    pub(crate) fn take(self) -> Option<&'b mut NodeMut<'a, T>> {
        self.0
    }
}

impl<'a, 'b, T> StackListMut<'a, 'b, T> {
    pub fn iter_mut<'c>(&'c mut self) -> StackListMutIter<'a, 'b, 'c, T> {
        StackListMutIter(&mut self.0)
    }

    // TODO: Impl `iter()`
}

pub struct StackListMutIter<'a, 'b, 'c, T>(&'c mut Option<&'b mut NodeMut<'a, T>>);

// TODO: Implemented fused?
impl<'a, 'b, 'c, T> FusedIterator for StackListMutIter<'a, 'b, 'c, T> {}

impl<'a, 'b, 'c, T> Iterator for StackListMutIter<'a, 'b, 'c, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        let ptrs = self.0.as_mut().map(|inner| {
            let content: (&mut Option<&mut NodeMut<T>>, &mut T) = inner.pair();
            let value_ptr = content.1 as *mut T;
            let previous_ptr = content.0 as *mut Option<&mut NodeMut<T>>;
            let previous_ptr = previous_ptr as *mut Option<NonNull<NodeMut<T>>>; // Need to sever the inner lifetime too
            (value_ptr, previous_ptr)
        });

        if let Some((value_ptr, previous_ptr)) = ptrs {
            let previous_ptr_lifetime = previous_ptr as *mut Option<&mut NodeMut<T>>;

            // SAFETY: More problems with recursive types. Both value_ptr and previous_ptr live for `'a`.
            //         so it's safe to both replace `self` with another value that lives for `'a` and
            //         return a reference to the value. `nodemut.value` and `nodemut.previous` don't alias.
            let new = StackListMutIter(unsafe { &mut *previous_ptr_lifetime });
            let _ = replace(self, new);
            Some(unsafe { &mut *value_ptr })
        } else {
            None
        }
    }
}

#[test]
fn test_iter_mut() {
    let mut a = NodeMut::new(None, 1i32);
    let mut b = NodeMut::new(Some(&mut a), 2);
    let mut c = NodeMut::new(Some(&mut b), 3);
    let mut d = NodeMut::new(Some(&mut c), 4);

    let f = &mut d;
    let head = &mut d;
    let mut list = StackListMut(Some(head));
    let mut list = StackListMut(Some(head));

    {
        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 4));

        let value = iter.next();
        assert_eq!(value, Some(&mut 3));
        if let Some(x) = value {
            *x = 6;
        }

        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None); // Just to make sure nothing weird is going on.
    }

    let mut iter = list.iter_mut();
    assert_eq!(iter.next(), Some(&mut 4));
    assert_eq!(iter.next(), Some(&mut 6));
    assert_eq!(iter.next(), Some(&mut 2));
    assert_eq!(iter.next(), Some(&mut 1));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None); // Just to make sure nothing weird is going on.
}

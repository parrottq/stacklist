use core::mem::replace;

use crate::node_mut::NodeMut;

// TODO: Make self.0 pub?
pub struct StackListMut<'a, T>(Option<&'a mut NodeMut<'a, T>>);

impl<'a, T> StackListMut<'a, T> {
    pub fn iter_mut(&'a mut self) -> StackListMutIter<'a, T>
    {
        StackListMutIter(&mut self.0)
        // StackListMutIter(self.0)
    }
}

// pub struct StackListMutIter<'a, 'b, T>(&'b mut Option<&'a mut NodeMut<'a, T>>);

// impl<'a, 'b, T> Iterator for StackListMutIter<'a, 'b, T>
// where
//     'b: 'a,
// {

// TODO: Implemented fused?
pub struct StackListMutIter<'a, T>(&'a mut Option<&'a mut NodeMut<'a, T>>);

impl<'a, T> Iterator for StackListMutIter<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        let ptrs = self.0.as_mut().map(|inner| {
            let content: (&mut Option<&mut NodeMut<T>>, &mut T) = inner.pair();
            let value_ptr = content.1 as *mut T;
            let previous_ptr = content.0 as *mut Option<&mut NodeMut<T>>;
            let previous_ptr = previous_ptr as *mut Option<*mut NodeMut<T>>; // Need to sever the inner lifetime too
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

    let head = &mut d;
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

    // let mut list = StackListMut(Some(&mut d));
    // let mut iter = list.iter_mut();
    // assert_eq!(iter.next(), Some(&mut 4));
    // assert_eq!(iter.next(), Some(&mut 3));
    // assert_eq!(iter.next(), Some(&mut 2));
    // assert_eq!(iter.next(), Some(&mut 1));
    // assert_eq!(iter.next(), None);
    // assert_eq!(iter.next(), None); // Just to make sure nothing weird is going on.
}

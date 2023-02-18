use core::{marker::PhantomData, ptr::NonNull};

use crate::node_ref::NodeRef;

#[repr(C)]
pub struct NodeMut<'a, T> {
    // Creating recursive &mut with nested lifetimes is not possible as far as I can tell
    // (it is for references). I think this `&mut T` being invariant over `T`.
    previous: Option<NonNull<()>>, // `Option<&'a mut NodeMut<'a, T>>` actual type
    pub value: T,
    _node: PhantomData<&'a mut T>,
}

impl<'a, T> NodeMut<'a, T> {
    pub fn new<'b>(previous: Option<&'a mut NodeMut<'b, T>>, value: T) -> Self
    where
        'b: 'a, // Make sure `'b` outlives `'a` so that lifetime erasing is sound.
    {
        Self {
            previous: previous.map(|x| {
                // SAFETY: `x` is a reference so it is already non-null.
                unsafe { NonNull::new_unchecked((x as *mut NodeMut<T>) as *mut ()) }
            }),
            value,
            _node: PhantomData,
        }
    }

    #[inline]
    pub fn pair_mut(&mut self) -> (&mut Option<&mut NodeMut<'a, T>>, &mut T) {
        (
            // SAFETY: The pointer in self.previous is always a reference to a `Option<&mut NodeMut<T>>` since only Self::new
            // can create this structure. Given `&'a mut NodeMut<'b, _>`, `'b` must outlive `'a` (Self::new ensures this).
            unsafe {
                let previous_ref: &mut Option<NonNull<()>> = &mut self.previous;
                let previous_raw = previous_ref as *mut Option<NonNull<()>>;
                let previous = previous_raw as *mut Option<&mut NodeMut<T>>;

                &mut *previous
            },
            &mut self.value,
        )
    }

    #[inline]
    pub fn pair_ref(&self) -> (Option<&NodeRef<'a, T>>, &T) {
        (
            {
                // SAFETY: The pointer in self.previous is always a reference to a `Option<&mut NodeMut<T>>` since only Self::new
                // can create this structure. Given `&'a mut NodeMut<'b, _>`, `'b` must outlive `'a` (Self::new ensures this).
                let previous = unsafe {
                    let previous_ref: &Option<NonNull<()>> = &self.previous;
                    let previous_raw = previous_ref as *const Option<NonNull<()>>;
                    let previous = previous_raw as *const Option<&NodeMut<T>>;

                    *previous
                };

                previous.map(AsRef::as_ref)
            },
            &self.value,
        )
    }

    #[inline]
    pub fn previous_node(&mut self) -> &mut Option<&mut NodeMut<'a, T>> {
        self.pair_mut().0
    }
}

impl<'a, T> AsRef<NodeRef<'a, T>> for NodeMut<'a, T> {
    fn as_ref(&self) -> &NodeRef<'a, T> {
        // SAFETY: The conversion from `NodeMut` to `NodeRef` is safe because both structures have the same
        // layout and size due to `repr(C)` and having the same contents. Also a mutable reference is being
        // turned into an immutable reference.
        unsafe {
            let ptr = self as *const NodeMut<'a, T>;
            let ptr = ptr as *const NodeRef<'a, T>;
            &*ptr
        }
    }
}

#[test]
fn test_lifetime() {
    let mut a = NodeMut::new(None, 1i32);
    let mut b = NodeMut::new(Some(&mut a), 2);
    let mut c = NodeMut::new(Some(&mut b), 3);
    let d = NodeMut::new(Some(&mut c), 4);

    drop(d); // Make sure that dropping `d` makes `c` accessible

    assert_eq!(3, c.value);
    let b = c
        .previous_node()
        .as_mut()
        .expect("There should be a previous");
    assert_eq!(2, b.value);
    let a = b
        .previous_node()
        .as_mut()
        .expect("There should be a previous");
    assert_eq!(1, a.value);
    assert!(
        a.previous_node().as_mut().is_none(),
        "Last node should be empty"
    );
}

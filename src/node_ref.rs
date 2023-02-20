#[repr(C)]
pub struct NodeRef<'a, T> {
    previous: Option<&'a NodeRef<'a, T>>,
    value: T,
}

impl<'a, T> NodeRef<'a, T> {
    pub fn new<'b>(previous: Option<&'a NodeRef<'b, T>>, value: T) -> Self {
        Self { previous, value }
    }

    #[inline]
    pub fn pair(&self) -> (&Option<&NodeRef<'a, T>>, &T) {
        (&self.previous, &self.value)
    }

    #[inline]
    pub fn previous_node(&self) -> &Option<&NodeRef<'a, T>> {
        self.pair().0
    }
}

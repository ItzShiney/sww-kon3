pub trait PushLast {
    type Item;

    fn push_last(&mut self, value: Self::Item) -> &mut Self::Item;
}

impl<T> PushLast for Vec<T> {
    type Item = T;

    fn push_last(&mut self, value: Self::Item) -> &mut Self::Item {
        self.push(value);
        unsafe { self.last_mut().unwrap_unchecked() }
    }
}

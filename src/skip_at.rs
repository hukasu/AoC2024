use std::iter::Enumerate;

pub struct SkipAt<T: Iterator> {
    iter: Enumerate<T>,
    index: usize,
}

impl<T: Iterator> SkipAt<T> {
    pub fn new(iter: T, index: usize) -> Self {
        Self {
            iter: iter.enumerate(),
            index,
        }
    }
}

impl<T: Iterator> Iterator for SkipAt<T> {
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .and_then(|(i, item)| {
                if i == self.index {
                    self.iter.next()
                } else {
                    Some((i, item))
                }
            })
            .map(|(_, item)| item)
    }
}

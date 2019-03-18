use std::option::Option;
use std::vec::Vec;

pub trait SafeVec<T> {
    fn safe_remove(&mut self, index: usize) -> Option<T>;
}

impl<T> SafeVec<T> for Vec<T> {
    fn safe_remove(&mut self, index: usize) -> Option<T> {
        if index >= self.len() {
            None
        } else {
            Some(self.remove(index))
        }
    }
}

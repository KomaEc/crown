pub trait DestructiveReadable {
    type Output;
    fn destructive_read(&mut self) -> Self::Output;
}

impl<T: Default> DestructiveReadable for T {
    type Output = T;
    fn destructive_read(&mut self) -> Self::Output {
        let mut ret = T::default();
        std::mem::swap(&mut ret, self);
        ret
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub mod list_node;

#[cfg(test)]
pub mod test_util {
    use std::fmt::Debug;

    use pretty_assertions::assert_eq;

    /// validator for test-case
    pub fn any_order<T: Ord + Debug>(mut expected: Vec<T>) -> impl FnMut(Vec<T>) {
        move |mut actual: Vec<T>| {
            actual.sort_unstable();
            expected.sort_unstable();
            assert_eq!(expected, actual)
        }
    }
}

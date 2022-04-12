pub trait Sorter<T> {
    fn sort(&self, slice: &mut [T])
    where
        T: Ord;
}

mod bubble_sort;
mod insertion_sort;
mod shell_sort;

pub use bubble_sort::BubbleSort;
pub use insertion_sort::InsertionSort;
pub use shell_sort::ShellSort;

pub struct StdSorter;
impl<T> Sorter<T> for StdSorter {
    fn sort(&self, slice: &mut [T])
    where
        T: Ord,
    {
        slice.sort();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn std_works() {
        let mut things = vec![4, 2, 3, 1];
        StdSorter.sort(&mut things);
        assert_eq!(things, &[1, 2, 3, 4]);
    }
}

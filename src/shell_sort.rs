use super::Sorter;

pub struct ShellSort;

impl<T> Sorter<T> for ShellSort {
    fn sort(&self, slice: &mut [T])
    where
        T: Ord,
    {
        let gaps = vec![701, 301, 132, 57, 23, 10, 4, 1]; // Ciura gap sequence
        for gap in gaps {
            let mut offset = 0;
            while offset < gap {
                let mut i = offset;
                while i < slice.len() {
                    let mut j = i;

                    while j >= gap && &slice[j - gap] > &slice[j] {
                        slice.swap(j, j - 1);
                        j -= gap;
                    }
                    i += gap;
                }

                offset += 1;
            }
        }
    }
}

#[test]
fn it_works() {
    let mut things = vec![4, 2, 3, 1];
    ShellSort.sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 4]);
}

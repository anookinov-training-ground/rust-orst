use super::Sorter;

pub struct InsertionSort;

impl Sorter for InsertionSort {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord,
    {
        // [ sorted | not sorted ]
    }
}

#[test]
fn it_works() {
    let mut things = vec![4, 2, 5, 3, 1];
    InsertionSort::sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 4, 5]);
}

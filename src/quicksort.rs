use super::Sorter;

pub struct QuickSort;

impl<T> Sorter<T> for QuickSort {
    fn sort(&self, slice: &mut [T])
    where
        T: Ord,
    {
        let n = slice.len();
        quick(slice, 0, n);
    }
}

fn quick<T>(slice: &mut [T], l: usize, r: usize)
where
    T: Ord,
{
    if r - l <= 1 {
        return;
    }
    let rand = rand::random::<usize>();
    let pivot_index = l + rand % (r - l);
    slice.swap(r - 1, pivot_index);
    let mut pos = l;
    for i in l..(r - 1) {
        if slice[i] < slice[r - 1] {
            slice.swap(i, pos);
            pos += 1;
        }
    }
    slice.swap(r - 1, pos);
    quick(slice, l, pos);
    quick(slice, pos + 1, r);
}

#[test]
fn it_works() {
    let mut things = vec![4, 2, 5, 3, 1];
    QuickSort.sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 4, 5]);
}

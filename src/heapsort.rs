use super::Sorter;

pub struct HeapSort;

impl<T> Sorter<T> for HeapSort {
    fn sort(&self, slice: &mut [T])
    where
        T: Ord,
    {
        let n = slice.len();
        for i in (0..=(n / 2)).rev() {
            heapify(slice, i, n);
        }
        for i in (0..n).rev() {
            slice.swap(i, 0);
            heapify(slice, 0, i);
        }
    }
}

fn heapify<T>(slice: &mut [T], curr: usize, boundary: usize)
where
    T: Ord,
{
    let mut curr = curr;
    while curr < boundary {
        let l = 2 * curr + 1;
        let r = 2 * curr + 2;
        let mut pos = curr;
        if l < boundary && slice[l] > slice[pos] {
            pos = l;
        }
        if r < boundary && slice[r] > slice[pos] {
            pos = r;
        }
        if pos == curr {
            return;
        }
        slice.swap(pos, curr);
        curr = pos;
    }
}

#[test]
fn it_works() {
    let mut things = vec![4, 2, 5, 3, 1];
    HeapSort.sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 4, 5]);
}

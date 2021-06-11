use crate::WindowMedian;
use std::cmp::Ordering;
use std::collections::VecDeque;

pub struct VecWindow<T: Ord + Copy> {
    cap: usize,
    items: VecDeque<T>,
    sorted: Vec<T>,
}

impl<T: Ord + Copy> VecWindow<T> {
    /// Constructs a new, empty `VecWindow<T>` with the specified
    /// capacity.
    pub fn new(cap: usize) -> VecWindow<T> {
        VecWindow {
            cap: cap,
            items: VecDeque::with_capacity(cap),
            sorted: Vec::with_capacity(cap),
        }
    }
}

impl<T: Ord + Copy> WindowMedian<T> for VecWindow<T> {
    /// Inserts an element into the window, evicting the oldest element
    /// if the window is at full capacity.
    fn insert(&mut self, x: T) {
        let ipos = match self.sorted.binary_search(&x) {
            Ok(p) => p,
            Err(p) => p,
        };

        if self.items.len() < self.cap {
            self.sorted.insert(ipos, x);
        } else {
            let old = self.items.pop_front().unwrap();
            let rpos = match self.sorted.binary_search(&old) {
                Ok(p) => p,
                Err(_) => unreachable!("element from item history is not in sorted list"),
            };

            match rpos.cmp(&ipos) {
                Ordering::Equal => {
                    //     rpos, ipos
                    //     |
                    // a b c e f g
                    self.sorted[rpos] = x;
                }
                Ordering::Less => {
                    //     rpos    ipos
                    //     |       |
                    // a b c d e f h
                    //      <~~~~^
                    // insert(g)
                    self.sorted.copy_within(rpos + 1..ipos, rpos);
                    self.sorted[ipos - 1] = x;
                }
                Ordering::Greater => {
                    //     ipos  rpos
                    //     |     |
                    // a b d e f g h
                    //     ^~~~~>
                    // insert(c)
                    self.sorted.copy_within(ipos..rpos, ipos + 1);
                    self.sorted[ipos] = x;
                }
            }
        }

        self.items.push_back(x);
    }

    /// Returns the median element of the window.
    /// Returns the greater element when the window size is even.
    fn median(&self) -> Option<T> {
        let size = self.sorted.len();

        if size == 0 {
            None
        } else {
            Some(self.sorted[size / 2])
        }
    }

    /// Removes all elements from the window.
    fn clear(&mut self) {
        self.items.clear();
        self.sorted.clear();
    }

    // Returns the number of elements in the window.
    fn len(&self) -> usize {
        self.items.len()
    }
}

#[cfg(test)]
mod tests {
    use super::VecWindow;
    use crate::WindowMedian;
    use rand::prelude::*;

    fn assert_sorted(w: &VecWindow<u32>) {
        for i in 1..w.sorted.len() {
            assert!(w.sorted[i - 1] <= w.sorted[i]);
        }
    }

    #[test]
    fn empty_median() {
        let w = VecWindow::<u32>::new(5);
        assert_eq!(None, w.median());
    }

    #[test]
    fn median() {
        let mut w = VecWindow::<u32>::new(6);
        w.insert(6);
        assert_eq!(Some(6), w.median());

        w.insert(1);
        assert_eq!(Some(6), w.median());

        w.insert(5);
        assert_eq!(Some(5), w.median());

        w.insert(3);
        assert_eq!(Some(5), w.median());

        w.insert(2);
        assert_eq!(Some(3), w.median());

        w.insert(4);
        assert_eq!(Some(4), w.median());
    }

    #[test]
    fn insert_ascending() {
        let mut w = VecWindow::<u32>::new(10);

        for i in 0..100 {
            w.insert(i);
            assert_sorted(&w);
        }
    }

    #[test]
    fn insert_descending() {
        let mut w = VecWindow::<u32>::new(10);

        for i in 100..0 {
            w.insert(i);
            assert_sorted(&w);
        }
    }

    #[test]
    fn insert_random() {
        let mut rng: StdRng = SeedableRng::seed_from_u64(678943567895);
        let mut w = VecWindow::<u32>::new(10);

        for _ in 0..100 {
            w.insert(rng.next_u32());
            assert_sorted(&w);
        }
    }

    #[test]
    fn clear() {
        let mut w = VecWindow::<u32>::new(10);

        for i in 2..7 {
            w.insert(i);
        }
        assert_eq!(Some(4), w.median());

        w.clear();
        assert_eq!(None, w.median());

        for i in 12..17 {
            w.insert(i);
        }
        assert_eq!(Some(14), w.median());
    }
}

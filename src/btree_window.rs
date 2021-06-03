use crate::WindowMedian;
use std::cmp::Ordering;
use std::collections::{BTreeSet, VecDeque};

pub struct BTreeWindow<T: Ord + Copy> {
    cap: usize,
    items: VecDeque<T>,
    left: BTreeSet<T>,
    right: BTreeSet<T>,
}

impl<T: Ord + Copy> BTreeWindow<T> {
    /// Constructs a new, empty `BTreeWindow<T>`.
    pub fn new(cap: usize) -> BTreeWindow<T> {
        BTreeWindow {
            cap: cap,
            items: VecDeque::with_capacity(cap),
            left: BTreeSet::new(),
            right: BTreeSet::new(),
        }
    }

    /// Remove the oldest element in the window.
    fn remove(&mut self) {
        if self.items.is_empty() {
            return;
        }

        let old = self.items.pop_front().unwrap();
        if !self.left.remove(&old) {
            self.right.remove(&old);
        }
        self.rebalance();
    }

    /// Rebalance the internal sets such that their sizes differ by at most 1.
    fn rebalance(&mut self) {
        let lsize = self.left.len();
        let rsize = self.right.len();
        let diff = isize::abs(lsize as isize - rsize as isize) as usize / 2;

        if diff == 0 {
            return;
        } else if lsize > rsize {
            let mv: Vec<T> = self.left.iter().rev().take(diff).map(|x| *x).collect();
            for e in mv {
                self.left.remove(&e);
                self.right.insert(e);
            }
        } else if rsize > lsize {
            let mv: Vec<T> = self.right.iter().take(diff).map(|x| *x).collect();
            for e in mv {
                self.right.remove(&e);
                self.left.insert(e);
            }
        }
    }
}

impl<T: Ord + Copy> WindowMedian<T> for BTreeWindow<T> {
    /// Inserts an element into the window, evicting the oldest element
    /// if the window is at full capacity.
    fn insert(&mut self, x: T) {
        if self.items.len() == self.cap {
            self.remove();
        }

        self.items.push_back(x);

        if !self.left.is_empty() {
            let lmax = self.left.iter().next_back().unwrap();
            if x < *lmax {
                self.left.insert(x);
            } else {
                self.right.insert(x);
            }
        } else if !self.right.is_empty() {
            let rmin = self.right.iter().next().unwrap();
            if x > *rmin {
                self.right.insert(x);
            } else {
                self.left.insert(x);
            }
        } else {
            self.left.insert(x);
        }

        self.rebalance();
    }

    /// Returns the median element of the window.
    fn median(&self) -> Option<T> {
        let lsize = self.left.len();

        // since the sets differ by at most 1, the larger set contains the median
        match lsize.cmp(&self.right.len()) {
            Ordering::Less => Some(*self.right.iter().next().unwrap()),
            Ordering::Greater => Some(*self.left.iter().next_back().unwrap()),
            Ordering::Equal => {
                if lsize == 0 {
                    None
                } else {
                    Some(*self.right.iter().next().unwrap())
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::BTreeWindow;
    use crate::WindowMedian;
    use rand::prelude::*;

    fn check_sets(w: &BTreeWindow<u32>) {
        let lsize = w.left.len() as isize;
        let rsize = w.right.len() as isize;

        assert!(isize::abs(lsize - rsize) <= 1);

        if lsize > 0 && rsize > 0 {
            let lmax = w.left.iter().next_back().unwrap();
            let rmin = w.right.iter().next().unwrap();
            assert!(*lmax <= *rmin);
        }
    }

    #[test]
    fn empty_median() {
        let w = BTreeWindow::<u32>::new(5);
        assert_eq!(None, w.median());
    }

    #[test]
    fn median() {
        let mut w = BTreeWindow::<u32>::new(6);
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
        let mut w = BTreeWindow::<u32>::new(10);

        for i in 0..100 {
            w.insert(i);
            check_sets(&w);
        }
    }

    #[test]
    fn insert_descending() {
        let mut w = BTreeWindow::<u32>::new(10);

        for i in 100..0 {
            w.insert(i);
            check_sets(&w);
        }
    }

    #[test]
    fn insert_random() {
        let mut rng: StdRng = SeedableRng::seed_from_u64(678943567895);
        let mut w = BTreeWindow::<u32>::new(10);

        for _ in 0..100 {
            w.insert(rng.next_u32());
            check_sets(&w);
        }
    }
}

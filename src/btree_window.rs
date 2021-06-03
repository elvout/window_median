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

    /// Inserts an element into the window, evicting the oldest element
    /// if the window is at full capacity.
    pub fn insert(&mut self, x: T) {
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
    pub fn median(&self) -> Option<T> {
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
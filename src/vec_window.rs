use crate::WindowMedian;
use std::cmp::Ordering;
use std::collections::VecDeque;

pub struct VecWindow<T: Ord + Copy> {
    cap: usize,
    items: VecDeque<T>,
    sorted: Vec<T>,
}

impl<T: Ord + Copy> VecWindow<T> {
    /// Constructs a new, empty `VecWindow<T>`.
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
                    self.sorted[rpos] = x;
                }
                Ordering::Less => {
                    self.sorted.copy_within(rpos + 1..ipos, rpos);
                    self.sorted[ipos - 1] = x;
                }
                Ordering::Greater => {
                    self.sorted.copy_within(ipos..rpos, ipos + 1);
                    self.sorted[ipos] = x;
                }
            }
        }

        self.items.push_back(x);
    }

    /// Returns the median element of the window.
    fn median(&self) -> Option<T> {
        let size = self.sorted.len();

        if size == 0 {
            None
        } else {
            Some(self.sorted[size / 2])
        }
    }
}

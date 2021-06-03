mod btree_window;
mod vec_window;

pub trait WindowMedian<T: Ord + Copy> {
    fn insert(&mut self, x: T);
    fn median(&self) -> Option<T>;
}

pub use btree_window::BTreeWindow;
pub use vec_window::VecWindow;

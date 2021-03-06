mod btree_window;
mod vec_window;

pub trait WindowMedian<T: Ord + Copy> {
    /// Inserts an element into the window, evicting the oldest element
    /// if the window is at full capacity.
    fn insert(&mut self, x: T);

    /// Returns the median element of the window.
    fn median(&self) -> Option<T>;

    /// Removes all elements from the window.
    fn clear(&mut self);

    // Returns the number of elements in the window.
    fn len(&self) -> usize;
}

pub use btree_window::BTreeWindow;
pub use vec_window::VecWindow;

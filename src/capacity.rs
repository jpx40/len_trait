
//! Traits involving the capacity of a collection.
use super::len::Len;

/// A trait for describing the capacity of a collection.
///
/// Because frequently allocating memory can be inefficient, collections often store room for more
/// data than they actually hold. This "extra length" is represented by capacity, stating the
/// maximum length that can be occupied without any memory allocations.
///
/// Obtaining the capacity of the collection must take a constant amount of time and space.
pub trait Capacity: Len {
    /// Returns the capacity of the collection.
    ///
    /// If this collection stores zero-sized types, then it effectively has infinite capacity. For
    /// this reason, those collections should have a capacity of `usize::MAX`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use len_trait::Capacity;
    ///
    /// fn check_capacity<C: Capacity>(collection: C, zero_sized: bool) {
    ///     if zero_sized {
    ///         assert_eq!(collection.capacity(), usize::max_value());
    ///     } else {
    ///         assert!(collection.capacity() >= collection.len());
    ///     }
    /// }
    ///
    /// check_capacity(vec![()], true);
    /// check_capacity(vec![1, 2, 3], false);
    /// check_capacity("Hello, world!".to_string(), false);
    /// ```
    fn capacity(&self) -> usize;
}

/// A trait for creating an empty collection with a given, pre-allocated capacity.
///
/// If a user knows in advance a minimum bound for how much capacity they will need, they can use
/// the `with_capacity` trait to pre-allocate memory in advance. The resulting collection is
/// guaranteed to have at least the capacity given.
///
/// If the given capacity is zero, then no capacity should be allocated at all. The behaviour of the
/// `Default` implementation should act the same as `with_capacity(0)`.
///
/// Creating a collection with a given capacity should take linear time and space with respect to
/// the requested capacity. Creating a collection of zero-size types should always take constant
/// time and space.
pub trait WithCapacity: Capacity + Default {
    /// Creates a value of the given capacity.
    ///
    /// If a value of zero is given, this should have the same effect as calling `Default::default`,
    /// and should not allocate any memory.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use len_trait::WithCapacity;
    ///
    /// fn check_capacity<C: WithCapacity>(zero_sized: bool) {
    ///     let default = C::default();
    ///     let with_cap = C::with_capacity(10);
    ///     if zero_sized {
    ///         assert_eq!(default.capacity(), usize::max_value());
    ///         assert_eq!(with_cap.capacity(), usize::max_value());
    ///     } else {
    ///         assert_eq!(default.capacity(), 0);
    ///         assert!(with_cap.capacity() >= 10);
    ///     }
    /// }
    ///
    /// check_capacity::<Vec<()>>(true);
    /// check_capacity::<Vec<usize>>(false);
    /// check_capacity::<String>(false);
    /// ```
    fn with_capacity(capacity: usize) -> Self;
}

/// A trait for modifying the capacity of a collection.
///
/// These methods are mostly hints to
///
/// Clearing a collection must take at most a linear amount of time and space with respect to the
/// number of elements which are dropped.
pub trait CapacityMut: WithCapacity {
    /// Ensures that the capacity is at least the current length plus `additional`.
    ///
    /// Usually, this is used to avoid multiple allocations when you know exactly how much capacity
    /// is needed to store data.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use len_trait::CapacityMut;
    ///
    /// fn check_capacity<C: CapacityMut>(mut collection: C) {
    ///     collection.reserve(100);
    ///     assert!(collection.capacity() >= collection.len() + 100)
    /// }
    ///
    /// check_capacity(vec![1, 2, 3]);
    /// check_capacity("Hello, world!".to_string());
    /// ```
    fn reserve(&mut self, additional: usize);

    /// Similar to `reserve`, adding a strong hint to not reserve capacity above what's needed.
    ///
    /// By default, this method just delegates to `reserve` unless the implementation has a more
    /// efficient version.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::collections::HashMap;
    /// use len_trait::CapacityMut;
    ///
    /// fn check_capacity<C: CapacityMut>(mut collection: C, exact: bool) {
    ///     collection.reserve_exact(100);
    ///     if exact {
    ///         assert_eq!(collection.capacity(), collection.len() + 100)
    ///     } else {
    ///         assert!(collection.capacity() > collection.len() + 100)
    ///     }
    /// }
    ///
    /// check_capacity(vec![1, 2, 3], true);
    /// check_capacity("Hello, world!".to_string(), true);
    /// check_capacity(
    ///     {
    ///         let mut map = HashMap::new();
    ///         map.insert('a', 1);
    ///         map.insert('b', 2);
    ///         map.insert('c', 3);
    ///         map
    ///     },
    ///     false,
    /// );
    /// ```
    fn reserve_exact(&mut self, additional: usize) {
        CapacityMut::reserve(self, additional)
    }

    /// Reduces the capacity down as close as possible to the current length.
    ///
    /// If the length of the collection is zero, this method will set its capacity to zero. By
    /// default, this method does nothing unless the capacity is zero.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use len_trait::{Capacity, WithCapacity, CapacityMut};
    ///
    /// let mut v: Vec<usize> = WithCapacity::with_capacity(10);
    ///
    /// v.extend(&[1, 2, 3, 4, 5, 6][..]);
    /// assert_eq!(v.capacity(), 10);
    ///
    /// CapacityMut::shrink_to_fit(&mut v);
    /// assert_eq!(v.capacity(), 6);
    ///
    /// v.clear();
    /// CapacityMut::shrink_to_fit(&mut v);
    /// assert_eq!(v.capacity(), 0);
    /// ```
    fn shrink_to_fit(&mut self) {
        if <dyn Len>::is_empty(self) {
            *self = Default::default();
        }
    }
}
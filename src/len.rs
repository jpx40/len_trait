
//! Traits involving the length of a collection.

/// A trait for describing whether a collection is empty, i.e., its length is zero.
///
/// Checking whether a collection is empty should take a constant amount of time and space.
pub trait Empty {
    /// Returns whether the collection is empty.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use len_trait::Empty;
    ///
    /// fn check_empty<C: ?Sized + Empty>(collection: &C) {
    ///     assert!(collection.is_empty());
    /// }
    ///
    /// check_empty("");
    /// check_empty(&[1, 2, 3][..0]);
    /// check_empty(&"".to_string());
    /// ```
    fn is_empty(&self) -> bool;
}


/// A trait for describing the length of a collection.
///
/// The amount of data stored in a collection, i.e. the amount of space it requires in memory, is
/// directly proportional to its length. For this reason, `str` and other types measure their
/// lengths in code values (e.g. `u8`), not code points (e.g. `char`).
///
/// Obtaining the length of the collection must take a constant amount of time and space.

// TODO: https://github.com/Manishearth/rust-clippy/issues/1740
pub trait Len: Empty {
    /// Returns the length of the collection.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use len_trait::Len;
    ///
    /// fn print_len<C: ?Sized + Len>(collection: &C) {
    ///     println!("{} units long", collection.len());
    /// }
    ///
    /// print_len("中文");         // 6 units long
    /// print_len(&[1, 2, 3][..]); // 3 units long
    /// ```
    fn len(&self) -> usize;
}

/// A trait for clearing collections.
///
/// A collection is cleared by dropping all of its data. After `clear` is called, the collection is
/// guaranteed to be empty.
///
/// Clearing a collection must take at most a linear amount of time and space.
pub trait Clear: Len {
    /// Clears the collection, dropping its elements and returning its length to zero.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use len_trait::Clear;
    ///
    /// struct Value(u32);
    /// impl Drop for Value {
    ///     fn drop(&mut self) {
    ///         println!("Dropped a {}", self.0)
    ///     }
    /// }
    ///
    /// fn check_clear<C: Clear>(mut collection: C) {
    ///     collection.clear();
    ///     assert!(collection.is_empty());
    /// }
    ///
    /// check_clear(vec![Value(12)]); // Prints "Dropped a 12"
    /// check_clear("Hello, world!".to_string());
    /// ```
    fn clear(&mut self);
}

/// A trait for modifying the length of a collection.
///
/// These methods must take at most a linear amount of time and space with respect to the number of
/// elements which are moved or dropped.
pub trait LenMut: Default + Clear {
    /// Truncates the collection to be no greater than `len` long, dropping elements as needed.
    ///
    /// If the collection is less than `len` long, do nothing.
    ///
    /// # Panics
    ///
    /// Panics if `len` is not valid according to the collection. For example, the implementation
    /// for `String` will panic if `len` does not lie on a character boundary.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use len_trait::LenMut;
    ///
    /// fn check_truncate<C: LenMut>(mut collection: C) {
    ///     let old_len = collection.len();
    ///     collection.truncate(5);
    ///     if old_len >= 5 {
    ///         assert_eq!(collection.len(), 5);
    ///     } else {
    ///         assert_eq!(collection.len(), old_len);
    ///     }
    /// }
    ///
    /// check_truncate("Hello, world!".to_string());
    /// check_truncate(vec![1, 2, 3]);
    /// ```
    fn truncate(&mut self, len: usize);

    /// Splits off the collection at the given index, returning the data past the index.
    ///
    /// # Panics
    ///
    /// Panics if `index > len`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use len_trait::LenMut;
    ///
    /// fn check_split_off<C: LenMut>(mut collection: C) {
    ///     let old_len = collection.len();
    ///     let split = collection.split_off(5);
    ///     assert_eq!(collection.len(), 5);
    ///     assert_eq!(split.len(), old_len - 5);
    /// }
    ///
    /// check_split_off("Hello, world!".to_string());
    /// check_split_off(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    /// ```
    fn split_off(&mut self, index: usize) -> Self;
}
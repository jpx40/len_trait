
//! Traits involving collections which index over a particular type.
 use std::ops;


use self::ops::{Index, IndexMut, Range, RangeFrom, RangeFull, RangeTo};


mod private {
    pub trait Sealed {}
    impl Sealed for str {}
    impl<T> Sealed for [T] {}
}


/// Shorthand trait for collections which offer consistent, immutable slicing.
///
/// Here, "consistent" means that indexing is implemented for all of the `Range` types from the
/// standard library, and the type returned by each of the slices is always the same.
///
/// Currently, because inclusive ranges are unstable, and ranges starting with an exclusive bound
/// don't exist, this trait can't be implemented for anything other than `str` and `[T]`. Hopefully,
/// this will change in the future.
pub trait IndexRange<Idx>
    : Index<Range<Idx>, Output = <Self as Index<RangeFull>>::Output>
    + Index<RangeTo<Idx>, Output = <Self as Index<RangeFull>>::Output>
    + Index<RangeFrom<Idx>, Output = <Self as Index<RangeFull>>::Output>
    + Index<RangeFull>
    + private::Sealed {
}
impl<T: ?Sized, Idx> IndexRange<Idx> for T
where
    T: Index<
        Range<Idx>,
        Output = <T as Index<RangeFull>>::Output,
    >,
    T: Index<
        RangeTo<Idx>,
        Output = <T as Index<RangeFull>>::Output,
    >,
    T: Index<
        RangeFrom<Idx>,
        Output = <T as Index<RangeFull>>::Output,
    >,
    T: Index<RangeFull>,
    T: private::Sealed,
{
}

/// Shorthand trait for collections which offer consistent, mutable slicing.
///
/// Here, "consistent" means that indexing is implemented for all of the `Range` types from the
/// standard library, and the type returned by each of the slices is always the same.
///
/// Currently, because inclusive ranges are unstable, and ranges starting with an exclusive bound
/// don't exist, this trait can't be implemented for anything other than `str` and `[T]`. Hopefully,
/// this will change in the future.
pub trait IndexRangeMut<Idx>
    : IndexMut<Range<Idx>>
    + IndexMut<RangeTo<Idx>>
    + IndexMut<RangeFrom<Idx>>
    + IndexMut<RangeFull>
    + IndexRange<Idx> {
}

impl<T: ?Sized, Idx> IndexRangeMut<Idx> for T
where
    T: IndexMut<Range<Idx>>,
    T: IndexMut<RangeTo<Idx>>,
    T: IndexMut<RangeFrom<Idx>>,
    T: IndexMut<RangeFull>,
    T: IndexRange<Idx>,
{
}

/// A trait for splitting a collection into two pieces at a given index.
///
/// Splitting a collection must take a constant amount of time and space.
pub trait SplitAt<Idx>: IndexRange<Idx> {
    /// Splits the collection into two pieces at the given index.
    ///
    /// # Panics
    ///
    /// Panics if `index` is invalid according to the collection.
    fn split_at(
        &self,
        index: Idx,
    ) -> (&<Self as Index<RangeFull>>::Output, &<Self as Index<RangeFull>>::Output);
}

/// A trait for splitting a collection into two mutable pieces at a given index.
///
/// Splitting a collection must take a constant amount of time and space.
pub trait SplitAtMut<Idx>: IndexRangeMut<Idx> + SplitAt<Idx> {
    /// Splits the collection into two mutable pieces at the given index.
    ///
    /// # Panics
    ///
    /// Panics if `index` is invalid according to the collection.
    fn split_at_mut(
        &mut self,
        index: Idx,
    ) -> (&mut <Self as Index<RangeFull>>::Output, &mut <Self as Index<RangeFull>>::Output);
}
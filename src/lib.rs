
//! This crate generalises traits over the `len` and `capacity` methods found in most collections.
//!
//! Methods that are included:
//!
//! * [`capacity`]
//! * [`clear`]
//! * [`is_empty`]
//! * [`len`]
//! * [`reserve_exact`]
//! * [`reserve`]
//! * [`shrink_to_fit`]
//! * [`split_at_mut`]
//! * [`split_at`]
//! * [`split_off`]
//! * [`truncate`]
//! * [`with_capacity`]
//!
//! Additionally, the traits [`IndexRange<Idx>`] and [`IndexRangeMut<Idx>`] are provided for
//! "consistent slicing," i.e. slicing over all range types.
//!
//! # Modules
//!
//! The `len` module provides:
//!
//! * [`LenMut`], which requires `Clear`
//! * [`Clear`], which requires `Len`
//! * [`Len`], which requires `Empty`
//! * [`Empty`]
//!
//! The `capacity` module provides:
//!
//! * [`CapacityMut`], which requires `WithCapacity`
//! * [`WithCapacity`], which requires `Capacity`
//! * [`Capacity`], which requires `Len`.
//!
//! The `index` module provides:
//!
//! * [`IndexRange<Idx>`], automatically implemented from `Index<Idx>`
//! * [`IndexRangeMut<Idx>`], automatically implemented from `IndexMut<Idx>`
//! * [`SplitAt<Idx>`], which requires `IndexRange<Idx>`
//! * [`SplitAtMut<Idx>`], which requires `IndexRangeMut<Idx>`
//!
//! # Features
//!
//! The `alloc` and `std` features offer different tiers of implementations for different
//! collections. The `std` feature automatically enables `alloc`. Although the `std` feature is the
//! default, disabling it will enable `no_std`.
//!
//! [`LenMut`]: len/trait.LenMut.html
//! [`Clear`]: len/trait.Clear.html
//! [`Len`]: len/trait.Len.html
//! [`Empty`]: len/trait.Empty.html
//! [`CapacityMut`]: capacity/trait.CapacityMut.html
//! [`WithCapacity`]: capacity/trait.WithCapacity.html
//! [`Capacity`]: capacity/trait.Capacity.html
//! [`SplitAt<Idx>`]: index/trait.SplitAt.html
//! [`SplitAtMut<Idx>`]: index/trait.SplitAtMut.html
//! [`IndexRange<Idx>`]: index/trait.IndexRange.html
//! [`IndexRangeMut<Idx>`]: index/trait.IndexRangeMut.html
//! [`capacity`]: capacity/trait.Capacity.html#tymethod.capacity
//! [`clear`]: len/trait.Clear.html#tymethod.clear
//! [`is_empty`]: len/trait.Empty.html#tymethod.is_empty
//! [`len`]: len/trait.Len.html#tymethod.len
//! [`reserve_exact`]: capacity/trait.CapacityMut.html#method.reserve_exact
//! [`reserve`]: capacity/trait.CapacityMut.html#tymethod.reserve
//! [`shrink_to_fit`]: capacity/trait.CapacityMut.html#method.shrink_to_fit
//! [`split_at_mut`]: index/trait.SplitAtMut.html#tymethod.split_at_mut
//! [`split_at`]:  index/trait.SplitAt.html#tymethod.split_at
//! [`split_off`]: len/trait.LenMut.html#tymethod.split_off
//! [`truncate`]: len/trait.LenMut.html#tymethod.truncate
//! [`with_capacity`]: capacity/trait.WithCapacity.html#tymethod.with_capacity

#![doc(html_root_url = "https://docs.charr.xyz/len-trait/")]
#![cfg_attr(test, deny(warnings))]



extern crate alloc;

pub mod capacity;
pub mod index;
pub mod len;

pub use capacity::*;
pub use index::*;
pub use len::*;

mod impls;
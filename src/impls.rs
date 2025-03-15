
macro_rules! do_impl {
    (Empty for $name:ident; $($gen:tt)*) => {
        do_impl!(Empty for $name, $name; $($gen)*);
    };
    (Empty for $name:ident, $impl_for:ty; $($gen:tt)*) => {
        impl<$($gen)*> $crate::len::Empty for $impl_for {
            fn is_empty(&self) -> bool {
                $name::is_empty(self)
            }
        }
    };
    (Len for $name:ident; $($gen:tt)*) => {
        do_impl!(Len for $name, $name; $($gen)*);
    };
    (Len for $name:ident, $impl_for:ty; $($gen:tt)*) => {
        do_impl!(Empty for $name, $impl_for; $($gen)*);
        impl<$($gen)*> $crate::len::Len for $impl_for {
            fn len(&self) -> usize {
                $name::len(self)
            }
        }
    };
    (Clear for $name:ident; $($gen:tt)*) => {
        do_impl!(Clear for $name, $name; $($gen)*);
    };
    (Clear for $name:ident, $impl_for:ty; $($gen:tt)*) => {
        impl<$($gen)*> $crate::len::Clear for $impl_for {
            fn clear(&mut self) {
                $name::clear(self)
            }
        }
    };
    (LenMut for $name:ident; $($gen:tt)*) => {
        do_impl!(LenMut for $name, $name; $($gen)*);
    };
    (LenMut for $name:ident, $impl_for:ty; $($gen:tt)*) => {
        do_impl!(Clear for $name, $impl_for; $($gen)*);
        impl<$($gen)*> $crate::len::LenMut for $impl_for {
            fn truncate(&mut self, len: usize) {
                $name::truncate(self, len)
            }
            fn split_off(&mut self, index: usize) -> Self {
                $name::split_off(self, index)
            }
        }
    };
    (Capacity for $name:ident; $($gen:tt)*) => {
        do_impl!(Capacity for $name, $name; $($gen)*);
    };
    (Capacity for $name:ident, $impl_for:ty; $($gen:tt)*) => {
        impl<$($gen)*> $crate::capacity::Capacity for $impl_for {
            fn capacity(&self) -> usize {
                $name::capacity(self)
            }
        }
    };
    (WithCapacity for $name:ident; $($gen:tt)*) => {
        do_impl!(WithCapacity for $name, $name; $($gen)*);
    };
    (WithCapacity for $name:ident, $impl_for:ty; $($gen:tt)*) => {
        do_impl!(Capacity for $name, $impl_for; $($gen)*);
        impl<$($gen)*> $crate::capacity::WithCapacity for $impl_for {
            fn with_capacity(capacity: usize) -> Self {
                $name::with_capacity(capacity)
            }
        }
    };
    (CapacityMut for $name:ident; $($gen:tt)*) => {
        do_impl!(CapacityMut for $name, $name; $($gen)*);
    };
    (CapacityMut for $name:ident, $impl_for:ty; $($gen:tt)*) => {
        do_impl!(WithCapacity for $name, $impl_for; $($gen)*);
        impl<$($gen)*> $crate::capacity::CapacityMut for $impl_for {
            fn reserve(&mut self, additional: usize) {
                $name::reserve(self, additional)
            }
            fn reserve_exact(&mut self, additional: usize) {
                $name::reserve_exact(self, additional)
            }
            fn shrink_to_fit(&mut self) {
                $name::shrink_to_fit(self)
            }
        }
    };
    (inexact CapacityMut for $name:ident, $impl_for:ty; $($gen:tt)*) => {
        do_impl!(WithCapacity for $name, $impl_for; $($gen)*);
        impl<$($gen)*> $crate::capacity::CapacityMut for $impl_for {
            fn reserve(&mut self, additional: usize) {
                $name::reserve(self, additional)
            }
            fn shrink_to_fit(&mut self) {
                $name::shrink_to_fit(self)
            }
        }
    };
    (noshrink CapacityMut for $name:ident, $impl_for:ty; $($gen:tt)*) => {
        do_impl!(WithCapacity for $name, $impl_for; $($gen)*);
        impl<$($gen)*> $crate::capacity::CapacityMut for $impl_for {
            fn reserve(&mut self, additional: usize) {
                $name::reserve(self, additional)
            }
            fn reserve_exact(&mut self, additional: usize) {
                $name::reserve_exact(self, additional)
            }
        }
    };
    (SplitAt for $name:ident; $($gen:tt)*) => {
        do_impl!(SplitAt for $name, $name; $($gen)*);
    };
    (SplitAt for $name:ident, $impl_for:ty; $($gen:tt)*) => {
        impl<$($gen)*> $crate::index::SplitAt<usize> for $impl_for {
            fn split_at(&self, index: usize) -> (&Self, &Self) {
                $name::split_at(self, index)
            }
        }
    };
    (SplitAtMut for $name:ident; $($gen:tt)*) => {
        do_impl!(SplitAtMut for $name, $name; $($gen)*);
    };
    (SplitAtMut for $name:ident, $impl_for:ty; $($gen:tt)*) => {
        do_impl!(SplitAt for $name, $impl_for; $($gen)*);
        impl<$($gen)*> $crate::index::SplitAtMut<usize> for $impl_for {
            fn split_at_mut(&mut self, index: usize) -> (&mut Self, &mut Self) {
                $name::split_at_mut(self, index)
            }
        }
    };
}
type Slice<T> = [T];

do_impl!(Len for Slice, [T]; T);
do_impl!(SplitAtMut for Slice, [T]; T);
    type Str = str;


do_impl!(Len for Str, str; );
do_impl!(SplitAtMut for Str, str; );
use std::boxed::Box;
             use std::rc::Rc;
             use std::sync::Arc;
             use std::collections;
    
    
        use self::collections::{BTreeMap, BTreeSet, BinaryHeap, LinkedList, VecDeque};


        impl<T: ?Sized + super::len::Empty> super::len::Empty for Box<T> {
            fn is_empty(&self) -> bool {
                T::is_empty(self)
            }
        }
        impl<T: ?Sized + super::len::Len> super::len::Len for Box<T> {
            fn len(&self) -> usize {
                T::len(self)
            }
        }
        impl<T: ?Sized + super::capacity::Capacity> super::capacity::Capacity for Box<T> {
            fn capacity(&self) -> usize {
                T::capacity(self)
            }
        }

        impl<T: ?Sized + super::len::Empty> super::len::Empty for Rc<T> {
            fn is_empty(&self) -> bool {
                T::is_empty(self)
            }
        }
        impl<T: ?Sized + super::len::Len> super::len::Len for Rc<T> {
            fn len(&self) -> usize {
                T::len(self)
            }
        }
        impl<T: ?Sized + super::capacity::Capacity> super::capacity::Capacity for Rc<T> {
            fn capacity(&self) -> usize {
                T::capacity(self)
            }
        }

        impl<T: ?Sized + super::len::Empty> super::len::Empty for Arc<T> {
            fn is_empty(&self) -> bool {
                T::is_empty(self)
            }
        }
        impl<T: ?Sized + super::len::Len> super::len::Len for Arc<T> {
            fn len(&self) -> usize {
                T::len(self)
            }
        }
        impl<T: ?Sized + super::capacity::Capacity> super::capacity::Capacity for Arc<T> {
            fn capacity(&self) -> usize {
                T::capacity(self)
            }
        }


        do_impl!(Len for BTreeMap, BTreeMap<K, V>; K: Ord, V);
        do_impl!(Clear for BTreeMap, BTreeMap<K, V>; K: Ord, V);

        do_impl!(Len for BTreeSet, BTreeSet<T>; T: Ord);
        do_impl!(Clear for BTreeSet, BTreeSet<T>; T: Ord);

        do_impl!(Len for BinaryHeap, BinaryHeap<T>; T: Ord);
        do_impl!(Clear for BinaryHeap, BinaryHeap<T>; T: Ord);
        do_impl!(CapacityMut for BinaryHeap, BinaryHeap<T>; T: Ord);

        do_impl!(Len for LinkedList, LinkedList<T>; T);
        do_impl!(Clear for LinkedList, LinkedList<T>; T);

        do_impl!(Len for str, String; );
        do_impl!(LenMut for String; );
        do_impl!(CapacityMut for String; );

        do_impl!(Len for Self, Vec<T>; T);
        do_impl!(LenMut for Vec, Vec<T>; T);
        do_impl!(CapacityMut for Vec, Vec<T>; T);

        do_impl!(Len for VecDeque, VecDeque<T>; T);
        do_impl!(LenMut for VecDeque, VecDeque<T>; T);
        do_impl!(CapacityMut for VecDeque, VecDeque<T>; T);

   use std::ffi;


        impl super::len::Empty for ffi::CStr {
            fn is_empty(&self) -> bool {
                self.to_bytes().is_empty()
            }
        }
        impl super::len::Len for ffi::CStr {
            fn len(&self) -> usize {
                self.to_bytes().len()
            }
        }

        impl super::len::Empty for ffi::CString {
            fn is_empty(&self) -> bool {
                self.as_bytes().is_empty()
            }
        }
        impl super::len::Len for ffi::CString {
            fn len(&self) -> usize {
                self.as_bytes().len()
            }
        }
        // TODO: Clear for CString
        // TODO: LenMut for CString

        use self::ffi::{OsStr, OsString};
        do_impl!(Len for OsStr; );
        do_impl!(Len for OsStr, OsString; );
        do_impl!(Clear for OsString; );
        // TODO: LenMut for OsString
        do_impl!(CapacityMut for OsString; );

        use std::hash::Hash;
        use std::collections::HashMap;
        do_impl!(Len for HashMap, HashMap<K, V>; K: Eq + Hash, V);
        do_impl!(Clear for HashMap, HashMap<K, V>; K: Eq + Hash, V);
        do_impl!(inexact CapacityMut for HashMap, HashMap<K, V>; K: Eq + Hash, V);

        use std::collections::HashSet;
        do_impl!(Len for HashSet, HashSet<T>; T: Eq + Hash);
        do_impl!(Clear for HashSet, HashSet<T>; T: Eq + Hash);
        do_impl!(inexact CapacityMut for HashSet, HashSet<T>; T: Eq + Hash);
// cfg_if::cfg_if! {
//             if #[cfg(feature = indexmap)] {
//             do_impl!(Len for indexmap::HashSet, indexmap::HashSet<T>; T: Eq + Hash);
//             do_impl!(Clear for indexmap::HashSet, indexmap::HashSet<T>; T: Eq + Hash);
//             do_impl!(inexact CapacityMut for indexmap::HashSet, indexmap::HashSet<T>; T: Eq + Hash);
//             do_impl!(Len for indexmap::IndexMap, indexmap::IndexMap<K, V>; K: Eq + Hash, V);
//             do_impl!(Clear for indexmap::IndexMap, indexmap::IndexMap<K, V>; K: Eq + Hash, V);
//             do_impl!(inexact CapacityMut for indexmap::IndexMap, indexmap::IndexMap<K, V>; K: Eq + Hash, V);  
//         }
// }
// cfg_if::cfg_if! {
//     if #[cfg(feature = hashbrown)] {
//     do_impl!(Len for hashbrown::HashSet, hashbrown::HashSet<T>; T: Eq + Hash);
//     do_impl!(Clear for hashbrown::HashSet, hashbrown::HashSet<T>; T: Eq + Hash);
//     do_impl!(inexact CapacityMut for hashbrown::HashSet, hashbrown::HashSet<T>; T: Eq + Hash);
//     do_impl!(Len for hashbrown::HashMap, hashbrown::HashMap<K, V>; K: Eq + Hash, V);
//     do_impl!(Clear for hashbrown::HashMap, hashbrown::HashMap<K, V>; K: Eq + Hash, V);
//     do_impl!(inexact CapacityMut for hashbrown::HashMap, hashbrown::HashMap<K, V>; K: Eq + Hash, V);  
// }
// }
// cfg_if::cfg_if! {
//     if #[cfg(feature = rangemap)] {
//     do_impl!(Len for rangemap::RangeSet, rangemap::RangeSet<T>; T: Eq + Hash);
//     do_impl!(Clear for rangemap::RangeSet, rangemap::RangeSet<T>; T: Eq + Hash);
//     do_impl!(inexact CapacityMut for rangemap::RangeSet,  rangemap::RangeSet<T>; T: Eq + Hash);
//     do_impl!(Len for rangemap:: RangeInclusiveSet, rangemap:: RangeInclusiveSet<T>; T: Eq + Hash);
//     do_impl!(Clear for rangemap:: RangeInclusiveSet, rangemap:: RangeInclusiveSet<T>; T: Eq + Hash);
//     do_impl!(inexact CapacityMut for rangemap:: RangeInclusiveSet,  rangemap:: RangeInclusiveSet<T>; T: Eq + Hash);
//     do_impl!(Len for rangemap::RangeMap, rangemap::RangeMap<K, V>; K: Eq + Hash, V);
//     do_impl!(Clear for rangemap::RangeMap, rangemap::RangeMap<K, V>; K: Eq + Hash, V);
//     do_impl!(inexact CapacityMut for rangemap::RangeMap, rangemap::RangeMap<K, V>; K: Eq + Hash, V);
//     do_impl!(Len for rangemap::RangeInclusiveMap, rangemap::RangeInclusiveMap<K, V>; K: Eq + Hash, V);
//     do_impl!(Clear for rangemap::RangeInclusiveMap, rangemap::RangeInclusiveMap<K, V>; K: Eq + Hash, V);
//     do_impl!(inexact CapacityMut for rangemap::RangeInclusiveMap, rangemap::RangeInclusiveMap<K, V>; K: Eq + Hash, V);
// }
// }
// cfg_if::cfg_if! {
//     if #[cfg(feature = compact_str)] {
        
        
//         do_impl!(Len for compact_str::CompactString;);
//         do_impl!(Clear for compact_str::CompactString;);
//         do_impl!(inexact CapacityMut for compact_str::CompactString;);
   

// }
// }

// cfg_if::cfg_if! {
//     if #[cfg(feature = ndarray)] {
//     do_impl!(Len for ndarray::ArrayBase, ndarray::ArrayBase<S, D>; );
//     do_impl!(Clear for ndarray::ArrayBase, ndarray::ArrayBase<S, D>; );
//     do_impl!(inexact CapacityMut for ndarray::ArrayBase, ndarray::ArrayBase<S, D>;);
    
//     }
// }
// cfg_if::cfg_if! {
//     if #[cfg(feature =  smallvec)] {
//     do_impl!(Len for  smallvec::SmallVec, SmallVec<A: Array>; );
//     do_impl!(Clear for  smallvec::SmallVec,  SmallVec<A: Array>; );
//     do_impl!(inexact CapacityMut for  smallvec::SmallVec,  SmallVec<A: Array>;);
    
//     }
// }
// cfg_if::cfg_if! {
//     if #[cfg(feature =  bstr)] {
//     do_impl!(Len for  bstr::BString; );
//     do_impl!(Clear for   bstr::BString; );
//     do_impl!(inexact CapacityMut for  bstr::BString;);
    
//     }
// }
// cfg_if::cfg_if! {
//     if #[cfg(feature = heapless)] {
//     do_impl!(Len for heapless::Vec, heapless::Vec<T>; );
//     do_impl!(Clear for heapless::Vec, heapless::Vec<T>; );
//     do_impl!(inexact CapacityMut for heapless::Vec, heapless::Vec<T>;);
//     do_impl!(Len for heapless::Deque, heapless::Deque<T>; );
//     do_impl!(Clear for heapless::Deque, heapless::Deque<T>; );
//     do_impl!(inexact CapacityMut for heapless::Deque, heapless::Deque<T>;);
//     do_impl!(Len for heapless::LinearMap, heapless::LinearMap<K, V>; K: Eq + Hash, V);
//     do_impl!(Clear for heapless::LinearMap, heapless::LinearMap<K, V>; K: Eq + Hash, V);
//     do_impl!(inexact CapacityMut for heapless::LinearMap, heapless::LinearMap<K, V>; K: Eq + Hash, V);
//     do_impl!(Len for heapless::IndexMap, heapless::IndexMap<K, V>; K: Eq + Hash, V);
//     do_impl!(Clear for heapless::IndexMap, heapless::IndexMap<K, V>; K: Eq + Hash, V);
//     do_impl!(inexact CapacityMut for heapless::IndexMap, heapless::IndexMap<K, V>; K: Eq + Hash, V);
//     do_impl!(Len for heapless::IndexSet, heapless::IndexSet<T>; K: Eq + Hash, V);
//     do_impl!(Clear for heapless::IndexSet, heapless::IndexSet<T>; K: Eq + Hash, V);
//     do_impl!(inexact CapacityMut for heapless::IndexSet, heapless::IndexSet<T>; K: Eq + Hash, V);
//     do_impl!(Len for heapless::String;);
//     do_impl!(Clear for heapless::String;);
//     do_impl!(inexact CapacityMut for heapless::String;);

// }
// }

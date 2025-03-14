//  IMPLS.rs
//    by Lut99
//
//  Created:
//    06 Mar 2025, 10:34:44
//  Last edited:
//    06 Mar 2025, 15:24:12
//  Auto updated?
//    Yes
//
//  Description:
//!   Implements the main [`Railroad`]- and [`RailroadDelim`]-traits for
//!   standard library types.
//

use std::cell::{Ref, RefMut};
use std::collections::HashSet;
use std::rc::Rc;
use std::sync::{Arc, MutexGuard, RwLockReadGuard, RwLockWriteGuard};

use super::{Diagram, Railroad, RailroadDelim};


/***** HELPER MACROS *****/
/// Implements a transparent implementation for [`Railroad`]/[`RailroadDelim`] that defers to the
/// wrapped type.
///
/// # Syntax
/// Call it with a type to implement it for (including the generic `T`).
///
/// Optionally, precede it with the keyword `lifetime` to also add a lifetime `'a` to all impls.
macro_rules! transparent_impl {
    ($type:ty) => {
        impl<T: Railroad> Railroad for $type {
            type Output = <T as Railroad>::Output;
            type InlineOutput = <T as Railroad>::InlineOutput;

            #[inline]
            fn to_railroad_node() -> Self::Output { <T as Railroad>::to_railroad_node() }

            #[inline]
            fn to_railroad_node_inline() -> Self::InlineOutput { <T as Railroad>::to_railroad_node_inline() }

            #[inline]
            fn to_railroad_diagram() -> Diagram<'static> { <T as Railroad>::to_railroad_diagram() }
        }
        impl<T: RailroadDelim> RailroadDelim for $type {
            type DelimOutput = <T as RailroadDelim>::DelimOutput;

            #[inline]
            fn to_railroad_delim_node<'n>(node: impl 'static + railroad::Node) -> Self::DelimOutput {
                <T as RailroadDelim>::to_railroad_delim_node(node)
            }
        }
    };

    (lifetime $type:ty) => {
        impl<'a, T: Railroad> Railroad for $type {
            type Output = <T as Railroad>::Output;
            type InlineOutput = <T as Railroad>::InlineOutput;

            #[inline]
            fn to_railroad_node() -> Self::Output { <T as Railroad>::to_railroad_node() }

            #[inline]
            fn to_railroad_node_inline() -> Self::InlineOutput { <T as Railroad>::to_railroad_node_inline() }

            #[inline]
            fn to_railroad_diagram() -> Diagram<'static> { <T as Railroad>::to_railroad_diagram() }
        }
        impl<'a, T: RailroadDelim> RailroadDelim for $type {
            type DelimOutput = <T as RailroadDelim>::DelimOutput;

            #[inline]
            fn to_railroad_delim_node<'n>(node: impl 'static + railroad::Node) -> Self::DelimOutput {
                <T as RailroadDelim>::to_railroad_delim_node(node)
            }
        }
    };
}





/***** IMPLEMENTATIONS *****/
// Pointer-like impls
transparent_impl!(&T);
transparent_impl!(&mut T);
transparent_impl!(Box<T>);
transparent_impl!(Rc<T>);
transparent_impl!(lifetime Ref<'a, T>);
transparent_impl!(lifetime RefMut<'a, T>);
transparent_impl!(Arc<T>);
transparent_impl!(lifetime MutexGuard<'a, T>);
transparent_impl!(lifetime RwLockReadGuard<'a, T>);
transparent_impl!(lifetime RwLockWriteGuard<'a, T>);
#[cfg(feature = "parking_lot")]
transparent_impl!(lifetime parking_lot::MutexGuard<'a, T>);
#[cfg(feature = "parking_lot")]
transparent_impl!(lifetime parking_lot::RwLockReadGuard<'a, T>);
#[cfg(feature = "parking_lot")]
transparent_impl!(lifetime parking_lot::RwLockWriteGuard<'a, T>);


// Propagation for `Option` which will add optional tracks.
impl<T: Railroad> Railroad for Option<T> {
    type Output = railroad::Optional<<T as Railroad>::Output>;
    type InlineOutput = railroad::Optional<<T as Railroad>::InlineOutput>;

    #[inline]
    fn to_railroad_node() -> Self::Output { railroad::Optional::new(<T as Railroad>::to_railroad_node()) }

    #[inline]
    fn to_railroad_node_inline() -> Self::InlineOutput { railroad::Optional::new(<T as Railroad>::to_railroad_node_inline()) }
}
impl<T: RailroadDelim> RailroadDelim for Option<T> {
    type DelimOutput = railroad::Optional<<T as RailroadDelim>::DelimOutput>;

    #[inline]
    fn to_railroad_delim_node<'n>(node: impl 'static + railroad::Node) -> Self::DelimOutput {
        railroad::Optional::new(<T as RailroadDelim>::to_railroad_delim_node(node))
    }
}



// Propagation for `Vec` which will repeat something indefinitely.
impl<T: Railroad> Railroad for Vec<T> {
    type Output = railroad::Repeat<<T as Railroad>::Output, railroad::Empty>;
    type InlineOutput = railroad::Repeat<<T as Railroad>::InlineOutput, railroad::Empty>;

    #[inline]
    fn to_railroad_node() -> Self::Output { railroad::Repeat::new(<T as Railroad>::to_railroad_node(), railroad::Empty) }

    #[inline]
    fn to_railroad_node_inline() -> Self::InlineOutput { railroad::Repeat::new(<T as Railroad>::to_railroad_node_inline(), railroad::Empty) }
}
impl<T: RailroadDelim> RailroadDelim for Vec<T> {
    type DelimOutput = railroad::Repeat<<T as RailroadDelim>::DelimOutput, railroad::Empty>;

    #[inline]
    fn to_railroad_delim_node<'n>(node: impl 'static + railroad::Node) -> Self::DelimOutput {
        railroad::Repeat::new(<T as RailroadDelim>::to_railroad_delim_node(node), railroad::Empty)
    }
}

// Propagation for `HashSet` which will also repeat something indefinitely.
impl<T: Railroad> Railroad for HashSet<T> {
    type Output = railroad::Repeat<<T as Railroad>::Output, railroad::Empty>;
    type InlineOutput = railroad::Repeat<<T as Railroad>::InlineOutput, railroad::Empty>;

    #[inline]
    fn to_railroad_node() -> Self::Output { railroad::Repeat::new(<T as Railroad>::to_railroad_node(), railroad::Empty) }

    #[inline]
    fn to_railroad_node_inline() -> Self::InlineOutput { railroad::Repeat::new(<T as Railroad>::to_railroad_node_inline(), railroad::Empty) }
}
impl<T: RailroadDelim> RailroadDelim for HashSet<T> {
    type DelimOutput = railroad::Repeat<<T as RailroadDelim>::DelimOutput, railroad::Empty>;

    #[inline]
    fn to_railroad_delim_node<'n>(node: impl 'static + railroad::Node) -> Self::DelimOutput {
        railroad::Repeat::new(<T as RailroadDelim>::to_railroad_delim_node(node), railroad::Empty)
    }
}

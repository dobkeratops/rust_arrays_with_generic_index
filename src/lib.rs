
#![feature(collections_range)]
#![feature(drain_filter)]
#![feature(slice_rsplit)]
#![feature(slice_get_slice)]
#![feature(vec_resize_default)] 
#![feature(vec_remove_item)]
#![feature(collections_range)] 
#![feature(slice_rotate)]
#![feature(swap_with_slice)]

use std::collections::range::RangeArgument;
use std::cmp::Ordering;
use std::borrow::Borrow;
use std::vec::{Drain,Splice,DrainFilter};
use std::ops::{Deref,DerefMut,Index,IndexMut};
use std::slice::{Iter,IterMut,Windows,Chunks,ChunksMut,Split,SplitMut,RSplit,RSplitMut,RSplitN,RSplitNMut,SplitN,SplitNMut,SliceIndex};
use std::marker::PhantomData; // this sucks!


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod fixedsizearray;
pub mod dynamicarray;
pub use fixedsizearray::*;
pub use dynamicarray::*;



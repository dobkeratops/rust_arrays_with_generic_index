
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
	use super::*;
    #[test]
    fn it_works() {
		let mut ar=Array::<String,i32>::new();
		ar.push(String::from("foo"));
		ar.push(String::from("bar"));
		ar.push(String::from("baz"));
		ar.push(String::from("qux"));
		let ia=1i32;
		let ib=2i32;
		let mut extracted=String::new();
		for x in ar.iter(){
			extracted.push_str(x);
		}
		assert!(extracted=="foobarbazqux");
		assert!(ar.len()==4);
		assert_eq!(ar[ia]=="bar",ar[ib]=="baz");
    }
}

pub mod fixedsizearray;
pub mod dynamicarray;
pub use fixedsizearray::*;
pub use dynamicarray::*;



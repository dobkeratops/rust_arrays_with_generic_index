
#![feature(collections_range)]
#![feature(drain_filter)]
#![feature(slice_rsplit)]
#![feature(slice_get_slice)]
#![feature(vec_resize_default)] 
#![feature(vec_remove_item)]
#![feature(collections_range)] 
#![feature(slice_rotate)]
#![feature(swap_with_slice)]

pub mod fixed;
pub mod dynamic;
pub use fixed::*;
pub use dynamic::*;


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
		let ar_len:i32 = ar.len();
		let at_borrow:&[String]=ar.as_slice();
		let foo=["apple","banana"];

		assert!(foo[0i32]=="apple");		
		assert!(at_borrow.at(1)=="bar");		
		assert!(extracted=="foobarbazqux");
		assert!(ar_len==4);
		assert!(ar[ia]=="bar" && ar[ib]=="baz");
		assert!(ar[3 as usize]=="qux");
		let a3=Array3(00,10,20);
		assert!(a3[1i32]==10);
		
    }
}
pub trait IndexTrait  {	// TODO - would be better to use official from/into, but it doesn't let us impl
	fn from_usize(x:usize)->Self;
	fn into_usize(self)->usize;
}
macro_rules! impl_index_trait_for{($tname:ty)=>{
	impl IndexTrait for $tname{
		fn from_usize(x:usize)->Self{x as Self}
		fn into_usize(self)->usize{self as usize}
	}
}}
// DONT implement for usize, that is implemented directly.
impl_index_trait_for!(i32);
impl_index_trait_for!(u32);
impl_index_trait_for!(i16);
impl_index_trait_for!(u16);
impl_index_trait_for!(isize);
impl_index_trait_for!(i8);
impl_index_trait_for!(u8);

// get - function interface to indexing that we can inplement for existing types, if a[mytype] fails

trait At<T,I:IndexTrait> :Index<usize>{
	type Elem;
	fn at(&self,index:I)->&Self::Elem;
	fn at_mut(&mut self,index:I)->&mut Self::Elem;
}
impl<T,I:IndexTrait> At<T,I> for [T] {
	type Elem=T;
	fn at(&self, index:I)->&T{
		self.index(index.into_usize())
	}
	fn at_mut(&mut self, index:I)->&mut T{
		self.index_mut(index.into_usize())
	}
}
impl<A,I> At<A,I> for A 
	where
		A:Index<usize>+IndexMut<usize>+Sized,
		I:IndexTrait,
		A::Output : Sized,
{
	type Elem=A::Output;
	fn at(&self, index:I)->&Self::Elem{
		self.index(index.into_usize())
	}
	fn at_mut(&mut self, index:I)->&mut Self::Elem{
		self.index_mut(index.into_usize())
	}
}

/*
todo -whats going on with [T;N]
macro_rules! impl_array_get{
	[=>$count:expr]=>{
		impl<T,I:IndexTrait> MyGet<T,I> for [T;$count] {
			type Elem = T;
			fn get(&self, i:I)->&T{
				self.index(0 as usize/*i.into_usize()*/)
			}
			fn get_mut(&mut self, i:I)->&mut T{
				self.index_mut( 0 as usize/*i.into_usize()*/)
			}
		}
	};
	// given a {..num list..}, call above for each
	[$($i:expr),*]=>{
		$( impl_array_get!(=>$i); )*
	}
}

impl_array_get![1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,32,64,128,256,512,1024];
*/
/*
macro_rules! impl_get_by{($index_type:ty)=>{
	impl Get<T,$index_type> for &[T]{
		type Output=T;
		fn get(&self,index:$index_type)->&T{
			self.index(index.into_usize())
		}
	}
	impl Get<T,$index_type> for &[T]{
		type Output=T;
		fn get(&self,index:$index_type)->&T{
			self.index(index.into_usize())
		}
		fn get_mut(&mut self,index:$index_type)->&T{
			self.index_mut(index.into_usize())
		}
	}
}}
impl_get_by!(i8);
impl_get_by!(u8);
impl_get_by!(i16);
impl_get_by!(u16);
impl_get_by!(i32);
impl_get_by!(u32);
impl_get_by!(i64);
impl_get_by!(u64);
impl_get_by!(isize);
*/




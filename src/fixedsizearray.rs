use super::*;

#[derive(Copy,Clone,Debug)]
pub struct Array1<T>{pub elems:[T;1]}	// could be generated..
#[derive(Copy,Clone,Debug)]
pub struct Array2<T>{pub elems:[T;2]}
#[derive(Copy,Clone,Debug)]
pub struct Array3<T>{pub elems:[T;3]}
#[derive(Copy,Clone,Debug)]
pub struct Array4<T>{pub elems:[T;4]}
#[derive(Copy,Clone,Debug)]
pub struct Array8<T>{pub elems:[T;8]}
// todo 16,..

// todo, macro. 
pub fn Array1<T>(a:T)->Array1<T>{Array1{ elems:[a]}}
pub fn Array2<T>(a:T,b:T)->Array2<T>{Array2{ elems:[a,b]}}
pub fn Array3<T>(a:T,b:T,c:T)->Array3<T>{Array3{ elems:[a,b,c]}}
pub fn Array4<T>(a:T,b:T,c:T,d:T)->Array4<T>{Array4{ elems:[a,b,c,d]}}
pub fn Array8<T>(a:T,b:T,c:T,d:T, e:T,f:T,g:T,h:T)->Array8<T>{Array8{ elems:[a,b,c,d,e,f,g,h]}}

macro_rules! impl_array_index_by_type{($index_type:ty)=>{
	impl<T> Index<$index_type> for Array1<T> {
		type Output=T;
		fn index(&self,i:$index_type)->&T{
			&self.elems[i as usize]
		}
	}
	impl<T> Index<$index_type> for Array2<T> {
		type Output=T;
		fn index(&self,i:$index_type)->&T{
			&self.elems[i as usize]
		}
	}
	impl<T> Index<$index_type> for Array3<T> {
		type Output=T;
		fn index(&self,i:$index_type)->&T{
			&self.elems[i as usize]
		}
	}
	impl<T> Index<$index_type> for Array4<T> {
		type Output=T;
		fn index(&self,i:$index_type)->&T{
			&self.elems[i as usize]
		}
	}
	impl<T> Index<$index_type> for Array8<T> {
		type Output=T;
		fn index(&self,i:$index_type)->&T{
			&self.elems[i as usize]
		}
	}

	impl<T> IndexMut<$index_type> for Array2<T> {
		fn index_mut(&mut self,i:$index_type)->&mut T{
			&mut self.elems[i as usize]
		}
	}
	impl<T> IndexMut<$index_type> for Array3<T> {
		fn index_mut(&mut self,i:$index_type)->&mut T{
			&mut self.elems[i as usize]
		}
	}
	impl<T> IndexMut<$index_type> for Array4<T> {
		fn index_mut(&mut self,i:$index_type)->&mut T{
			&mut self.elems[i as usize]
		}
	}
	impl<T> IndexMut<$index_type> for Array8<T> {
		fn index_mut(&mut self,i:$index_type)->&mut T{
			&mut self.elems[i as usize]
		}
	}
}}

impl_array_index_by_type!(i8);
impl_array_index_by_type!(u8);
impl_array_index_by_type!(i16);
impl_array_index_by_type!(u16);
impl_array_index_by_type!(i32);
impl_array_index_by_type!(u32);
impl_array_index_by_type!(isize);
impl_array_index_by_type!(usize);

// TODO .. all the other accessors..

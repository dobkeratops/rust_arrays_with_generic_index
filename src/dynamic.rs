use super::*;

// dynamic arrays - called 'Array' here to avoid confusion
// but it should be a drop-in replacement for 'Vec'.
//todo, how to handle 'enumerate'.
// would we have to impl 'my_enumerate' or something?

// wrapper for Vec<T> with indexing defaulting to i32
// todo , real vector impl, with smallvec  stuff


// grrr. can't impl theirs this way round?!
//trait MyInto {
//}

//TODO - wrapper or macro to roll a 'strongly typed index'
// e.g. I32<Polygon>

/*
impl Into<usize> for i32{
	fn into(self)->usize{ self as usize }
}
impl Into<usize> for u32{
	fn into(self)->usize{ self as usize }
}
impl Into<usize> for i16{
	fn into(self)->usize{ self as usize }
}
impl Into<usize> for u32{
	fn into(self)->usize{ self as usize }
}
impl Into<usize> for i8{
	fn into(self)->usize{ self as usize }
}
impl Into<usize> for u8{
	fn into(self)->usize{ self as usize }
}
impl Into<usize> for isize{
	fn into(self)->usize{ self as usize }
}
*/



#[derive(Debug)]
pub struct Array<T,I=i32>(pub Vec<T>,PhantomData<I>);

// my array helper fn's
impl<T:Clone,I:IndexTrait+Clone> Array<T,I>{
	/// TODO - better name. preserves ordering of vec![v;count].
	pub fn from_val_n(val:T, n:i32)->Self{
		let v=vec![val; n as usize];
		Array(v,PhantomData)
	}
	pub fn from_fn<F:Fn(I)->T>(count:I,f:F)->Self{
		let mut v=Vec::new();
		v.reserve(count.clone().into_usize());
		for x in 0..count.into_usize() {v.push(f(I::from_usize(x)))}
		Array(v,PhantomData)
	}
	pub fn map<B,F:Fn(&T)->B>(&self,f:F)->Array<B,I>{
		let mut out=Array::<B,I>::new();
		out.reserve(self.len());
		for x in self.iter(){
			out.push(f(x))
		}
		out
	}
}

impl<T,I:IndexTrait+Clone> Array<T,I>{
	pub fn new()->Self{ Array(Vec::new(),PhantomData) }
	pub fn reserve(&mut self, additional: I){
		self.0.reserve(additional.into_usize());
	}	
	pub fn push(&mut self,val:T){self.0.push(val)}
	pub fn shrink_to_fit(&mut self){self.0.shrink_to_fit()}
	pub fn truncate(&mut self, len: I){
		self.0.truncate(len.into_usize());
	}
	pub fn as_slice(&self) -> &[T]{
		self.0.as_slice()
	}
	pub fn as_mut_slice(&mut self) -> &mut [T]{
		self.0.as_mut_slice()
	}
	pub fn swap_remove(&mut self, index: I) -> T{
		self.0.swap_remove(index.into_usize())
	}
	pub fn insert(&mut self, index: I, element: T){
		self.0.insert(index.into_usize(),element)
	}
	pub fn remove(&mut self, index: I) -> T{
		self.0.remove(index.into_usize())
	}
	// aka filter in place
	pub fn retain<F:FnMut(&T)->bool>(&mut self, f: F) {
		self.0.retain(f)
	}
	pub fn dedup_by_key<F:FnMut(&mut T)->K, K:PartialEq<K>>(&mut self, key: F) {
		self.0.dedup_by_key(key)
	}
	pub fn dedup_by<F:FnMut(&mut T,&mut T)->bool>(&mut self, same_bucket: F) {
		self.0.dedup_by(same_bucket)
	}
	#[cfg(nightly_vector)]
	pub fn place_back(&mut self) -> PlaceBack<T>{
		self.0.place_back()
	}
	pub fn pop(&mut self) -> Option<T>{
		self.0.pop()
	}
	pub fn append(&mut self, other: &mut Vec<T>){
		self.0.append(other)
	}
	#[cfg(UseRangeArgument)]
	pub fn drain<R:RangeArgument<I>>(&mut self, range: R) -> Drain<T> 
	{
		self.0.drain(range)
	}
	pub fn clear(&mut self){
		self.0.clear()
	}
//	pub fn len(&self)->I{
//		self.0.len() as Index
//	}
//	pub fn is_empty(&self)->bool{ self.0.is_empty()}
	pub fn split_off(&mut self,at:I)->Array<T>{
		Array(self.0.split_off(at.into_usize()),PhantomData)
	}
}
impl<T:Clone,I:IndexTrait> Array<T,I>{
	pub fn resize(&mut self, new_len:I, value:T){
		self.0.resize(new_len.into_usize(),value)
	}
	pub fn extend_from_slice(&mut self, other:&[T]){
		self.0.extend_from_slice(other)
	}
}

impl<T:Default,I:IndexTrait> Array<T,I>{
	pub fn resize_default(&mut self, new_len:I){
		self.0.resize_default(new_len.into_usize())
	}
}

impl<T:PartialEq<T>,I:IndexTrait> Array<T,I>{
	pub fn dedup(&mut self){
		self.0.dedup()
	}
	pub fn remove_item(&mut self, item:&T)->Option<T>{
		self.0.remove_item(item)
	}
}

impl<T,INDEX:IndexTrait> Array<T,INDEX>{
	/// TODO - figure out how to convert RangeArguemnt indices
	pub fn splice<I:IntoIterator<Item=T>,R:RangeArgument<usize>>(&mut self, range:R, replace_with:I)-> Splice<<I as IntoIterator>::IntoIter>
	{
		self.0.splice(range,replace_with)
	}
	pub fn drain_filter<F:FnMut(&mut T)->bool>(&mut self, filter: F) -> DrainFilter<T, F> {
		self.0.drain_filter(filter)
	}
}
/*
impl<T,INDEX:IndexTrait> Deref for Array<T,INDEX>{
	type Target=[T];
	fn deref(&self)->&Self::Target { self.0.deref() }
}
*/
impl<T,INDEX:IndexTrait> Array<T,INDEX>{
	pub fn len(&self)->INDEX{INDEX::from_usize(self.0.len())}
	pub fn is_empty(&self)->bool{self.0.is_empty()}
	pub fn first(&self)->Option<&T>{self.0.first()}
	pub fn first_mut(&mut self)->Option<&mut T>{self.0.first_mut()}
	pub fn split_first(&self)->Option<(&T,&[T])>{self.0.split_first()}
	pub fn split_first_mut(&mut self)->Option<(&mut T, &mut [T])>{ self.0.split_first_mut() }
	pub fn split_last(&self)->Option<(&T,&[T])>{self.0.split_last()}
	pub fn split_last_mut(&mut self)->Option<(&mut T, &mut[T])>{self.0.split_last_mut()}
	pub fn last(&self)->Option<&T>{self.0.last()}
	pub fn last_mut(&mut self)->Option<&mut T>{self.0.last_mut()}
	pub fn get<I>(&self, index:I)->Option<&<I as SliceIndex<[T]> >::Output>
		where I:SliceIndex<[T]>
	{
		self.0.get(index)
	}
	pub fn get_mut<I>(&mut self, index:I)->Option<&mut <I as SliceIndex<[T]>>::Output>
		where I:SliceIndex<[T]>
	{
		self.0.get_mut(index)
	}
	pub unsafe fn get_unchecked<I>(&self, index: I) -> &<I as SliceIndex<[T]>>::Output 
		where I: SliceIndex<[T]> {self.0.get_unchecked(index)}
	unsafe fn get_unchecked_mut<I>(
	    &mut self, 
		index: I
	) -> &mut <I as SliceIndex<[T]>>::Output 
		where I: SliceIndex<[T]>{
		self.0.get_unchecked_mut(index)
	}
	pub fn as_ptr(&self)->*const T{self.0.as_ptr()}
	pub fn as_mut_ptr(&mut self)->*mut T{self.0.as_mut_ptr()}
	pub fn swap(&mut self, a:INDEX,b:INDEX){
		self.0.swap(a.into_usize(),b.into_usize())
	}
	pub fn reverse(&mut self){self.0.reverse()}
	pub fn iter(&self)->Iter<T>{self.0.iter()}
	pub fn iter_mut(&mut self)->IterMut<T>{self.0.iter_mut()}
	pub fn windows(&self,size:INDEX)->Windows<T>{self.0.windows(size.into_usize())}
	pub fn chunks(&self,chunk_size:INDEX)->Chunks<T>{self.0.chunks(chunk_size.into_usize())}
	
	pub fn chunks_mut(&mut self,chunk_size:INDEX)->ChunksMut<T>{self.0.chunks_mut(chunk_size.into_usize())}
	pub fn split_at(&self, mid: INDEX) -> (&[T], &[T]){
		self.0.split_at(mid.into_usize())
	}
	pub fn split_at_mut(&mut self, mid: INDEX) -> (&mut [T], &mut [T]){
		self.0.split_at_mut(mid.into_usize())
	}
	pub fn split<F>(&self, pred: F) -> Split<T, F> 
		where F:FnMut(&T)->bool
	{
		self.0.split(pred)
	}
	pub fn split_mut<F>(&mut self, pred: F) -> SplitMut<T, F> 
		where F: FnMut(&T) -> bool
	{
		self.0.split_mut(pred)
	}
	pub fn rsplit<F>(&self, pred: F) -> RSplit<T, F> 
		where F: FnMut(&T) -> bool, 
	{
		self.0.rsplit(pred)
	}
	pub fn rsplit_mut<F>(&mut self, pred: F) -> RSplitMut<T, F>
		where F: FnMut(&T) -> bool	
	{
		self.0.rsplit_mut(pred)
	}
	pub fn splitn<F>(&self, n: INDEX, pred: F) -> SplitN<T, F> 
		where	F: FnMut(&T) -> bool
	{
		self.0.splitn(n.into_usize(),pred)
	}
	pub fn splitn_mut<F>(&mut self, n: INDEX, pred: F) -> SplitNMut<T, F> 
		where F: FnMut(&T) -> bool
	{
		self.0.splitn_mut(n.into_usize(),pred)
	}
	pub fn rsplitn<F>(&self, n: INDEX, pred: F) -> RSplitN<T, F> 
	where F: FnMut(&T) -> bool{
		self.0.rsplitn(n.into_usize(),pred)
	}
	pub fn rsplitn_mut<F>(&mut self, n: INDEX, pred: F) -> RSplitNMut<T, F> 
where
    F: FnMut(&T) -> bool{
		self.0.rsplitn_mut(n.into_usize(),pred)
	}
	pub fn contains(&self, x: &T) -> bool 
where
    T: PartialEq<T>{
		self.0.contains(x)
	}
	pub fn starts_with(&self, needle: &[T]) -> bool 
where
    T: PartialEq<T>{
		self.0.starts_with(needle)
	}
	pub fn ends_with(&self, needle: &[T]) -> bool 
where
    T: PartialEq<T>{
		self.0.ends_with(needle)
	}
	pub fn binary_search(&self, a: &T) -> Result<INDEX, INDEX> 
where
    T: Ord{
		match self.0.binary_search(a){
			Ok(x)=>Ok(INDEX::from_usize(x)),
			Err(x)=>Err(INDEX::from_usize(x))
		}
	}
	pub fn binary_search_by<'a, F>(&'a self, f: F) -> Result<INDEX, INDEX> 
		where F: FnMut(&'a T) -> Ordering{
		match self.0.binary_search_by(f){
			Ok(x)=>Ok(INDEX::from_usize(x)),
			Err(x)=>Err(INDEX::from_usize(x))
		}
	}
	pub fn binary_search_by_key<'a, B, F>(&'a self, b: &B, f: F) -> Result<INDEX, INDEX> 
	where
		B: Ord,
	    F: FnMut(&'a T) -> B,
		T: Ord
	{
		match self.0.binary_search_by_key(b,f){
			Ok(x)=>Ok(INDEX::from_usize(x)),
			Err(x)=>Err(INDEX::from_usize(x))
		}
	}
	pub fn sort(&mut self) where T:Ord{
		self.0.sort()
	}
	pub fn sort_by<F>(&mut self,f:F) where F:FnMut(&T,&T)->Ordering{
		self.0.sort_by(f)
	}
	pub fn sort_by_key<F,B>(&mut self,f:F) where B:Ord,F:FnMut(&T)->B{
		self.0.sort_by_key(f)
	}
	pub fn sort_unstable(&mut self)where T:Ord{self.0.sort_unstable()}
	
	pub fn sort_unstable_by<F>(&mut self,f:F)where T:Ord,F:FnMut(&T,&T)->Ordering{self.0.sort_unstable_by(f)}

	pub fn sort_unstable_by_key<B:Ord,F>(&mut self,f:F)where T:Ord,F:FnMut(&T)->B{self.0.sort_unstable_by_key(f)}
	pub fn rotate(&mut self,mid:INDEX){
		self.0.rotate(mid.into_usize())
	}
	pub fn clone_from_slice(&mut self, src:&[T]) where T:Clone{
		self.0.clone_from_slice(src)
	}
	pub fn copy_from_slice(&mut self, src:&[T]) where T:Copy{
		self.0.copy_from_slice(src)
	}
	pub fn swap_with_slice(&mut self, src:&mut[T]){
		self.0.swap_with_slice(src)
	}
	pub fn to_vec(&self)->Array<T> where T:Clone{
		Array(self.0.to_vec(),PhantomData)
	}
	
}

// access by the arrays dedicated index,
impl<T,INDEX:IndexTrait> Index<INDEX> for Array<T,INDEX>{
	type Output=T;
	fn index(&self,i:INDEX)->&T{
		&self.0.index(i.into_usize())
	}
}
impl<T,INDEX:IndexTrait> IndexMut<INDEX> for Array<T,INDEX>{
	fn index_mut(&mut self,i:INDEX)->&mut T{
		self.0.index_mut(i.into_usize())
	}
}
impl<T,INDEX:IndexTrait> Index<usize> for Array<T,INDEX>{
	type Output=T;
	fn index(&self,i:usize)->&T{
		&self.0.index(i)
	}
}
impl<T,INDEX:IndexTrait> IndexMut<usize> for Array<T,INDEX>{
	fn index_mut(&mut self,i:usize)->&mut T{
		self.0.index_mut(i)
	}
}


impl<T:Clone,INDEX:IndexTrait> Clone for Array<T,INDEX>{
	fn clone(&self)->Self{
		Array(self.0.clone(),PhantomData)
	}
	fn clone_from(&mut self, other:&Self){
		self.0.clone_from(&other.0);
		self.1.clone_from(&other.1);
	}
	
}
impl<T,INDEX:IndexTrait> Default for Array<T,INDEX>{
	fn default()->Self{
		Array(Vec::<T>::default(),PhantomData)
	}
}

impl<T,INDEX:IndexTrait> Borrow<[T]> for Array<T,INDEX>{
	fn borrow(&self) -> &[T]{
		self.0.borrow()
	}
}

impl<T,INDEX:IndexTrait> AsRef<[T]> for Array<T,INDEX>{
	fn as_ref(&self)->&[T]{
		self.0.as_ref()
	}
} 
impl<T,INDEX:IndexTrait> AsRef<Array<T,INDEX>> for Array<T,INDEX>{
	fn as_ref(&self)->&Self{
		self
	}
} 


aim: 
* [1]to make using 32 (or fewer) bit indices more ergonomic in rust 64bit builds (which currently requires continuous manual casting); 
* [2] To avoid signed vs unsigned friction (often indices do require interaction with signed values, eg offsets in convolutions etc) 
* [3] To allow strongly typed indices, e.g. differentiate between a Index32<Vertex>, Index32<Triangle> etc to give more compile time checks on the correct use of index data.

There is hardware vectorized 32bit index support, and on a 16gb machine,pervasive 64bit indices are overkill: 2^30 * 16byte objects will fill memory,yet 32bit mode is insufficient; 

Most applications will never fill memory with a single array. 

Graphical applications often reason about explicit sizes e.g. with knowledge of what fits in GPU memory, optimal chunk sizes for rendering, etc. It is common to cluster/subdivide scenes with certain maximums for individual elements, giving the opportunity to leverage bit-reduction in both indexing and spatial precision within chunks.

approach:-

* 'Vec<T>' is wrapped in Array<T,INDEX>
* there is an IndexTrait which must be implemented for indexable types, which must provide conversion to and from usize , for interop with the underlying rust vector implementation
* There are small 'Array2,Array3, Array4' types to replace [T;2],[T;3],[T;4] with similar support (WIP)
* no solution yet for slices (TODO!)

Possible future work:-
It would be best to re-implement Vec itself with custom Length,Capacity fields, e.g. 'a 32bit indexed vector/64bit build' would take 16bytes for the struct instead of 24. This is lower priority, and it's possible the larger vec gives useful space for a SmallVec optimization later. 

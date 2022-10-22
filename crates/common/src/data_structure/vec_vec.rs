use std::alloc::{Allocator, Global};

/// A vector of non-growable-after-construction vecs: `Vec<Vec<I>>`
#[derive(Clone, Debug)]
pub struct VecVec<I, A: Allocator = Global> {
    indices: Vec<usize, A>,
    data: Vec<I, A>,
}

impl<I, A: Allocator> std::ops::Index<usize> for VecVec<I, A> {
    type Output = [I];

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        let end = self.indices[index + 1];
        let start = self.indices[index];
        &self.data[start..end]
    }
}

impl<I, A: Allocator> std::ops::IndexMut<usize> for VecVec<I, A> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let end = self.indices[index + 1];
        let start = self.indices[index];
        &mut self.data[start..end]
    }
}

impl<I> VecVec<I> {
    pub fn with_capacity(indices_capacity: usize, data_capacity: usize) -> VecVecConstruction<I> {
        let mut indices = Vec::with_capacity(indices_capacity + 1);
        indices.push(0);
        let data = Vec::with_capacity(data_capacity);
        let vec_array = VecVec { indices, data };
        VecVecConstruction {
            vec_vec: vec_array,
            start_index: 0,
            n_cur_items: 0,
        }
    }

    pub fn with_indices_capacity(size: usize) -> VecVecConstruction<I> {
        let mut indices = Vec::with_capacity(size + 1);
        indices.push(0);
        let data = Vec::new();
        let vec_array = VecVec { indices, data };
        VecVecConstruction {
            vec_vec: vec_array,
            start_index: 0,
            n_cur_items: 0,
        }
    }

    pub fn with_data_capacity(size: usize) -> VecVecConstruction<I> {
        let mut indices = Vec::new();
        indices.push(0);
        let data = Vec::with_capacity(size);
        let vec_array = VecVec { indices, data };
        VecVecConstruction {
            vec_vec: vec_array,
            start_index: 0,
            n_cur_items: 0,
        }
    }

    #[inline]
    pub fn repack<U, F>(self, f: F) -> VecVec<U>
    where
        F: Fn(I) -> U,
    {
        let indices = self.indices;
        let data = self.data.into_iter().map(f).collect();
        VecVec { indices, data }
    }
}

impl<I, A: Allocator + Copy> VecVec<I, A> {
    pub fn new_in(len: usize, alloc: A) -> VecVecConstruction<I, A> {
        let mut indices = Vec::with_capacity_in(len + 1, alloc);
        indices.push(0);
        let data = Vec::new_in(alloc);
        let vec_array = VecVec { indices, data };
        VecVecConstruction {
            vec_vec: vec_array,
            start_index: 0,
            n_cur_items: 0,
        }
    }

    #[inline]
    pub fn everything(&self) -> &[I] {
        &self.data
    }

    #[inline]
    pub fn everything_mut(&mut self) -> &mut [I] {
        &mut self.data
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.indices.len() - 1
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &[I]> {
        self.indices
            .array_windows()
            .map(|&[start, end]| &self.data[start..end])
    }
}

#[derive(Debug)]
pub struct VecVecConstruction<I, A: Allocator = Global> {
    vec_vec: VecVec<I, A>,
    start_index: usize,
    n_cur_items: usize,
}

impl<I, A: Allocator> VecVecConstruction<I, A> {
    #[inline]
    pub fn push_inner(&mut self, item: I) {
        self.vec_vec.data.push(item);
        self.n_cur_items += 1;
    }

    #[inline]
    pub fn push_vec(&mut self, items: impl Iterator<Item = I>) {
        debug_assert_eq!(self.n_cur_items, 0);
        let old_len = self.vec_vec.data.len();
        self.vec_vec.data.extend(items);
        self.start_index += self.vec_vec.data.len() - old_len;
        self.vec_vec.indices.push(self.start_index);
    }

    #[inline]
    pub fn push(&mut self) {
        self.start_index += std::mem::take(&mut self.n_cur_items);
        self.vec_vec.indices.push(self.start_index);
    }

    #[inline]
    pub fn done(self) -> VecVec<I, A> {
        self.vec_vec
    }

    #[inline]
    pub fn get_constructed(&self, index: usize) -> &[I] {
        if index + 1 >= self.vec_vec.indices.len() {
            panic!("the entry for {index} is still under construction")
        }
        &self.vec_vec[index]
    }
}

use std::alloc::{Allocator, Global};

// use lending_iterator::{gat, prelude::LendingIteratorඞItem, LendingIterator};

/// A vector of non-growable arrays `Vec<Array<I>>`
#[derive(Debug)]
pub struct VecArray<I, A: Allocator = Global> {
    indices: Vec<usize, A>,
    data: Vec<I, A>,
}

impl<I, A: Allocator> std::ops::Index<usize> for VecArray<I, A> {
    type Output = [I];

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        let end = self.indices[index + 1];
        let start = self.indices[index];
        &self.data[start..end]
    }
}

impl<I, A: Allocator> std::ops::IndexMut<usize> for VecArray<I, A> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let end = self.indices[index + 1];
        let start = self.indices[index];
        &mut self.data[start..end]
    }
}

impl<I> VecArray<I> {
    pub fn with_indices_capacity(size: usize) -> VecArrayConstruction<I> {
        let mut indices = Vec::with_capacity(size + 1);
        indices.push(0);
        let data = Vec::new();
        let frozen_vec_vec = VecArray { indices, data };
        VecArrayConstruction {
            vec_array: frozen_vec_vec,
            start_index: 0,
            n_cur_items: 0,
        }
    }

    pub fn with_data_capacity(size: usize) -> VecArrayConstruction<I> {
        let mut indices = Vec::new();
        indices.push(0);
        let data = Vec::with_capacity(size);
        let frozen_vec_vec = VecArray { indices, data };
        VecArrayConstruction {
            vec_array: frozen_vec_vec,
            start_index: 0,
            n_cur_items: 0,
        }
    }

    #[inline]
    pub fn repack<U, F>(self, f: F) -> VecArray<U>
    where
        F: Fn(I) -> U,
    {
        let indices = self.indices;
        let data = self.data.into_iter().map(f).collect();
        VecArray { indices, data }
    }
}

impl<I, A: Allocator + Copy> VecArray<I, A> {
    pub fn new_in(len: usize, alloc: A) -> VecArrayConstruction<I, A> {
        let mut indices = Vec::with_capacity_in(len + 1, alloc);
        indices.push(0);
        let data = Vec::new_in(alloc);
        let vec_array = VecArray { indices, data };
        VecArrayConstruction {
            vec_array,
            start_index: 0,
            n_cur_items: 0,
        }
    }

    #[inline]
    pub fn everything(&self) -> &[I] {
        &self.data
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

    // #[inline]
    // pub fn iter_mut(&mut self) -> VecArrayIterMut<'_, I> {
    //     //impl Iterator<Item = &mut [I]> {
    //     VecArrayIterMut {
    //         indices: self.indices.array_windows(),
    //         data: &mut self.data,
    //     }
    // }
}

// pub struct VecArrayIterMut<'a, I> {
//     indices: std::slice::ArrayWindows<'a, usize, 2>,
//     data: &'a mut [I],
// }

// #[gat]
// impl<'a, I> LendingIterator for VecArrayIterMut<'a, I> {
//     type Item<'next> = &'next mut [I];

//     fn next(self: &'_ mut Self) -> Option<Self::Item<'_>> {
//         let &[start, end] = self.indices.next()?;
//         Some(&mut self.data[start..end])
//     }
// }

#[derive(Debug)]
pub struct VecArrayConstruction<I, A: Allocator = Global> {
    vec_array: VecArray<I, A>,
    start_index: usize,
    n_cur_items: usize,
}

impl<I, A: Allocator> VecArrayConstruction<I, A> {
    #[inline]
    pub fn add_item_to_array(&mut self, item: I) {
        self.vec_array.data.push(item);
        self.n_cur_items += 1;
    }

    #[inline]
    pub fn push_array(&mut self, items: impl Iterator<Item = I>) {
        debug_assert_eq!(self.n_cur_items, 0);
        let old_len = self.vec_array.data.len();
        self.vec_array.data.extend(items);
        self.start_index += self.vec_array.data.len() - old_len;
        self.vec_array.indices.push(self.start_index);
    }

    #[inline]
    pub fn done_with_array(&mut self) {
        self.start_index += std::mem::take(&mut self.n_cur_items);
        self.vec_array.indices.push(self.start_index);
    }

    #[inline]
    pub fn done(self) -> VecArray<I, A> {
        self.vec_array
    }

    #[inline]
    pub fn get_constructed(&self, index: usize) -> &[I] {
        if index + 1 >= self.vec_array.indices.len() {
            panic!("the entry for {index} is still under construction")
        }
        &self.vec_array[index]
    }
}

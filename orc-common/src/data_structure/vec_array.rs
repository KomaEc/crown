/// A vector of non-growable arrays `Vec<Array<I>>`
#[derive(Debug)]
pub struct VecArray<I> {
    l1_indexing: Vec<usize>,
    l2_indexing: Vec<I>,
}

impl<I> std::ops::Index<usize> for VecArray<I> {
    type Output = [I];

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        let end = self.l1_indexing[index + 1];
        let start = self.l1_indexing[index];
        &self.l2_indexing[start..end]
    }
}

impl<I> std::ops::IndexMut<usize> for VecArray<I> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let end = self.l1_indexing[index + 1];
        let start = self.l1_indexing[index];
        &mut self.l2_indexing[start..end]
    }
}

impl<I> VecArray<I> {
    pub fn new(len: usize) -> VecArrayConstruction<I> {
        let mut l1_indexing = Vec::with_capacity(len + 1);
        l1_indexing.push(0);
        let l2_indexing = Vec::new();
        let frozen_vec_vec = VecArray {
            l1_indexing,
            l2_indexing,
        };
        VecArrayConstruction {
            vec_array: frozen_vec_vec,
            l1_index: 0,
            n_cur_items: 0,
        }
    }

    #[inline]
    pub fn everything(&self) -> &[I] {
        &self.l2_indexing
    }

    #[inline]
    pub fn array_count(&self) -> usize {
        self.l1_indexing.len() - 1
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &[I]> {
        self.l1_indexing
            .array_windows()
            .map(|&[start, end]| &self.l2_indexing[start..end])
    }
}

#[derive(Debug)]
pub struct VecArrayConstruction<I> {
    vec_array: VecArray<I>,
    l1_index: usize,
    n_cur_items: usize,
}

impl<I> VecArrayConstruction<I> {
    #[inline]
    pub fn add_item_to_array(&mut self, item: I) {
        self.vec_array.l2_indexing.push(item);
        self.n_cur_items += 1;
    }

    #[inline]
    pub fn push_array(&mut self, items: impl Iterator<Item = I>) {
        debug_assert_eq!(self.n_cur_items, 0);
        let old_len = self.vec_array.l2_indexing.len();
        self.vec_array.l2_indexing.extend(items);
        self.l1_index += self.vec_array.l2_indexing.len() - old_len;
        self.vec_array.l1_indexing.push(self.l1_index);
    }

    #[inline]
    pub fn done_with_array(&mut self) {
        self.l1_index += std::mem::take(&mut self.n_cur_items);
        self.vec_array.l1_indexing.push(self.l1_index);
    }

    #[inline]
    pub fn done(self) -> VecArray<I> {
        self.vec_array
    }

    #[inline]
    pub fn get_constructed(&self, l1: usize) -> &[I] {
        if l1 + 1 >= self.vec_array.l1_indexing.len() {
            panic!("the entry for {l1} is still under construction")
        }
        &self.vec_array[l1]
    }
}

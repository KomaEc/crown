#[derive(Clone, Copy)]
pub struct NonNullRawMut<T> {
    ptr: *mut T,
    _marker: core::marker::PhantomData<*mut T>,
}

impl<T> NonNullRawMut<T> {
    pub fn as_mut_ptr(self) -> *mut T {
        self.ptr
    }

    pub fn as_ptr(self) -> *const T {
        self.ptr as *const _
    }

    pub unsafe fn as_mut(&mut self) -> &mut T {
        &mut *self.ptr
    }

    pub unsafe fn as_ref(&self) -> &T {
        &*self.ptr
    }
}
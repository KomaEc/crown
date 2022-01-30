#![feature(min_specialization)]
#![feature(maybe_uninit_extra)]

use core::marker::PhantomData;
use std::mem::MaybeUninit;

pub struct Cached;
pub struct Vacant;

pub struct Cache<Data, State> {
    cache: MaybeUninit<Data>,
    _state: PhantomData<State>,
}

impl<Data> Cache<Data, Vacant> {
    pub fn new() -> Self {
        Cache {
            cache: MaybeUninit::uninit(),
            _state: PhantomData,
        }
    }

    pub fn cache(mut self, data: Data) -> Cache<Data, Cached> {
        self.cache.write(data);
        todo!() // unsafe { std::mem::transmute(self) }
    }
}

impl<Data> AsRef<Data> for Cache<Data, Cached> {
    fn as_ref(&self) -> &Data {
        unsafe { self.cache.assume_init_ref() }
    }
}

impl<Data, State> Drop for Cache<Data, State> {
    default fn drop(&mut self) {}
}

impl<Data> Drop for Cache<Data, Cached> {
    fn drop(&mut self) {
        unsafe { self.cache.assume_init_drop() }
    }
}

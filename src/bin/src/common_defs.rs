use std::fs;
use std::path::Path;

fn nonnull_raw_mut_def() -> &'static str {
    "
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
}"
}

pub fn add_defs_to_dir(dir_path: &Path) {
    let aux_path = dir_path.join("crustr_ptr.rs");
    for entry in dir_path.read_dir().expect("read_dir call failed") {
        if let Ok(entry) = entry {
            if entry.path() == aux_path {
                // eprintln!("Duplicated file: {}", aux_path.as_path().to_str().unwrap());
                log::warn!("Overwrite existing crustr_ptr.rs")
            }
        }
    }
    // let aux_file = File::create(aux_path).expect("add aux_file");
    fs::write(aux_path, nonnull_raw_mut_def()).expect("write file failed");
}

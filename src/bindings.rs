extern crate libc;
pub use libc::{c_void, size_t};

#[repr(C)]
pub struct Tensor {
    pub data: *mut c_void,
    pub size: size_t
}

extern "C" {
    pub fn allocateTensor(t: *mut Tensor, data: *const c_void, size: size_t, typeSize: size_t);
    pub fn freeTensor(t: *mut Tensor);
}

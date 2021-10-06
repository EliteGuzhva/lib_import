use crate::bindings::{
    c_void,
    size_t
};
use crate::bindings::Tensor;
use crate::bindings::{
    allocateTensor,
    freeTensor
};


pub struct FooTensor<'a, T> {
    _tensor: Tensor,
    pub data: &'a mut [T],
    pub size: usize
}

impl<T> FooTensor<'_, T> {
    pub fn new(data: &[T]) -> FooTensor<T> {
        let mut tensor = Tensor {
            data: std::ptr::null_mut(),
            size: 0
        };

        unsafe {
            allocateTensor(&mut tensor,
                           data.as_ptr() as *const c_void,
                           data.len() as size_t,
                           std::mem::size_of::<T>())
        };

        let size = tensor.size;

        let data = unsafe {
            std::slice::from_raw_parts_mut(tensor.data as *mut T, size)
        };

        FooTensor {
            _tensor: tensor,
            data,
            size
        }
    }
}

impl<T> Drop for FooTensor<'_, T> {
    fn drop(&mut self) {
        unsafe {
            freeTensor(&mut self._tensor);
        }
    }
}

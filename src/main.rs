mod bindings;
mod wrappers;

use wrappers::FooTensor;


fn main() {
    let data = vec![1., 2., 3.];
    let t = FooTensor::new(&data);
    t.data[0] = 7.;
    println!("Tensor size is {}", t.size);
    println!("Tensor data is {:#?}", t.data);
}

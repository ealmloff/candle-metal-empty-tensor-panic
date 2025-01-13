use candle_core::{Device, Tensor};

fn main() {
    let device = Device::new_metal(0).unwrap();
    let tensors = [("testing", Tensor::from_iter([] as [u32; 0], &device).unwrap())];
    safetensors::serialize(tensors, &None).unwrap();
}

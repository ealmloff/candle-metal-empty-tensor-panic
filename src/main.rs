// [package]
// name = "candle-panic"
// version = "0.1.0"
// edition = "2021"

// [dependencies]
// candle-core = { version = "0.8.2", features = ["metal"] }
// safetensors = "0.4.0"

use candle_core::{Device, Tensor};

fn main() {
    let device = Device::new_metal(0).unwrap();
    let tensors = [("testing", Tensor::from_iter([] as [u32; 0], &device).unwrap())];
    safetensors::serialize(tensors, &None).unwrap();
}

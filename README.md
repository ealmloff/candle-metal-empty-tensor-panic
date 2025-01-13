This repo panics when run on a metal device (tested on M2 mac) with `cargo run`:
```
thread 'main' panicked at /Users/evanalmloff/.cargo/registry/src/index.crates.io-6f17d22bba15001f/foreign-types-shared-0.3.1/src/lib.rs:72:9:
assertion failed: !ptr.is_null()
stack backtrace:
   0: rust_begin_unwind
             at /rustc/9fc6b43126469e3858e2fe86cafb4f0fd5068869/library/std/src/panicking.rs:665:5
   1: core::panicking::panic_fmt
             at /rustc/9fc6b43126469e3858e2fe86cafb4f0fd5068869/library/core/src/panicking.rs:76:14
   2: core::panicking::panic
             at /rustc/9fc6b43126469e3858e2fe86cafb4f0fd5068869/library/core/src/panicking.rs:148:5
   3: foreign_types_shared::ForeignTypeRef::from_ptr
             at /Users/evanalmloff/.cargo/registry/src/index.crates.io-6f17d22bba15001f/foreign-types-shared-0.3.1/src/lib.rs:72:9
   4: <metal::buffer::Buffer as core::ops::deref::Deref>::deref
             at /Users/evanalmloff/.cargo/registry/src/index.crates.io-6f17d22bba15001f/metal-0.27.0/src/lib.rs:169:9
   5: candle_core::metal_backend::MetalStorage::to_cpu
             at /Users/evanalmloff/.cargo/registry/src/index.crates.io-6f17d22bba15001f/candle-core-0.8.2/src/metal_backend/mod.rs:1866:35
   6: <candle_core::metal_backend::MetalStorage as candle_core::backend::BackendStorage>::to_cpu_storage
             at /Users/evanalmloff/.cargo/registry/src/index.crates.io-6f17d22bba15001f/candle-core-0.8.2/src/metal_backend/mod.rs:100:46
   7: candle_core::tensor::Tensor::to_vec1
             at /Users/evanalmloff/.cargo/registry/src/index.crates.io-6f17d22bba15001f/candle-core-0.8.2/src/tensor.rs:1626:58
   8: candle_core::safetensors::convert_back
             at /Users/evanalmloff/.cargo/registry/src/index.crates.io-6f17d22bba15001f/candle-core-0.8.2/src/safetensors.rs:236:47
   9: candle_core::safetensors::<impl safetensors::tensor::View for candle_core::tensor::Tensor>::data
             at /Users/evanalmloff/.cargo/registry/src/index.crates.io-6f17d22bba15001f/candle-core-0.8.2/src/safetensors.rs:60:20
  10: safetensors::tensor::serialize
             at /Users/evanalmloff/.cargo/registry/src/index.crates.io-6f17d22bba15001f/safetensors-0.4.5/src/tensor.rs:235:23
  11: candle_segfault::main
             at ./src/main.rs:16:5
  12: core::ops::function::FnOnce::call_once
             at /Users/evanalmloff/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

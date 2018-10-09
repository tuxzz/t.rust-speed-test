# t.rust-speed-test
A simple test that show the strange performance behaviour of rustc

Run `cargo run --release`

### Test result

`i7-4790 3.60GHz` + `Windows 10 17763` + `rustc 1.31.0-nightly (2bd5993ca 2018-10-02)`

|          Test          | Time(s) |
|------------------------|---------|
|      ndarray_iter      |  2.684  |
|       slice_iter       |  0.875  |
|    ndarray_zip_with    |  0.307  |
|   slice_loop_safe_bad  |  0.519  |
|  slice_loop_safe_good  |  0.298  |
| slice_loop_unsafe_good |  0.133  |
# t.rust-speed-test
A simple test that show the strange performance behaviour of rustc

Run `cargo run --release`

### Test result

`i7-4790 3.60GHz` + `Windows 10 17763` + `MSVC 2017`

|          Test          | Time(s) |
|------------------------|---------|
|     cpp_method_bad     |  0.432  |
|    cpp_method_good     |  0.236  |

`i7-4790 3.60GHz` + `Windows 10 17763` + `rustc 1.31.0-nightly (2bd5993ca 2018-10-02) x86_64-pc-windows-msvc`

|          Test          | Time(s) |
|------------------------|---------|
|      ndarray_iter      |  2.684  |
|       slice_iter       |  0.875  |
|    ndarray_zip_with    |  0.307  |
|   slice_loop_safe_bad  |  0.519  |
|  slice_loop_safe_good  |  0.298  |
| slice_loop_unsafe_good |  0.133  |


`i7-4790 3.60GHz` + `Windows 10 17763` + `rustc 1.31.0-nightly (2bd5993ca 2018-10-02) x86_64-pc-windows-gnu` + LTO

|          Test          | Time(s) |
|------------------------|---------|
|      ndarray_iter      |  1.157  |
|       slice_iter       |  0.835  |
|    ndarray_zip_with    |  0.297  |
|   slice_loop_safe_bad  |  0.408  |
|  slice_loop_safe_good  |  0.358  |
| slice_loop_unsafe_good |  0.189  |

`i7-4790 3.60GHz` + `Windows 10 17763` + `rustc 1.31.0-nightly (2bd5993ca 2018-10-02) x86_64-pc-windows-gnu` + LTO - incremental - overflow-checks

|          Test          | Time(s) |
|------------------------|---------|
|      ndarray_iter      |  0.968  |
|       slice_iter       |  0.799  |
|    ndarray_zip_with    |  0.282  |
|   slice_loop_safe_bad  |  0.281  |
|  slice_loop_safe_good  |  0.306  |
| slice_loop_unsafe_good |  0.178  |
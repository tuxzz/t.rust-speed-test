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

`i7-4790 3.60GHz` + `Windows 10 17763` + `rustc 1.31.0-nightly (2bd5993ca 2018-10-02) x86_64-pc-windows-msvc` + `Thin LTO`

|          Test          | Time(s) |
|------------------------|---------|
|      ndarray_iter      |  1.310  |
|       slice_iter       |  0.776  |
|    ndarray_zip_with    |  0.268  |
|   slice_loop_safe_bad  |  0.418  |
|  slice_loop_safe_good  |  0.288  |
| slice_loop_unsafe_good |  0.126  |

`i7-4790 3.60GHz` + `Windows 10 17763` + `rustc 1.31.0-nightly (2bd5993ca 2018-10-02) x86_64-pc-windows-msvc` + `Thin LTO` - `incremental` - `overflow-checks`

|          Test          | Time(s) |
|------------------------|---------|
|      ndarray_iter      |  0.968  |
|       slice_iter       |  0.788  |
|    ndarray_zip_with    |  0.266  |
|   slice_loop_safe_bad  |  0.510  |
|  slice_loop_safe_good  |  0.288  |
| slice_loop_unsafe_good |  0.126  |

`i7-4790 3.60GHz` + `Windows 10 17763` + `rustc 1.31.0-nightly (2bd5993ca 2018-10-02) x86_64-pc-windows-msvc` + `Thin LTO` - `incremental` - `overflow-checks` + `target-cpu=native`

|          Test          | Time(s) |
|------------------------|---------|
|      ndarray_iter      |  1.148  |
|       slice_iter       |  0.829  |
|    ndarray_zip_with    |  0.279  |
|   slice_loop_safe_bad  |  0.510  |
|  slice_loop_safe_good  |  0.354  |
| slice_loop_unsafe_good |  0.185  |

`i7-4790 3.60GHz` + `Windows 10 17763` + `rustc 1.31.0-nightly (2bd5993ca 2018-10-02) x86_64-pc-windows-gnu` + `Thin LTO` + `target-cpu=native`

|          Test          | Time(s) |
|------------------------|---------|
|      ndarray_iter      |  1.157  |
|       slice_iter       |  0.835  |
|    ndarray_zip_with    |  0.297  |
|   slice_loop_safe_bad  |  0.408  |
|  slice_loop_safe_good  |  0.358  |
| slice_loop_unsafe_good |  0.189  |

`i7-4790 3.60GHz` + `Windows 10 17763` + `rustc 1.31.0-nightly (2bd5993ca 2018-10-02) x86_64-pc-windows-gnu` + `Thin LTO` - `incremental` - `overflow-checks` + `target-cpu=native`

|          Test          | Time(s) |
|------------------------|---------|
|      ndarray_iter      |  0.968  |
|       slice_iter       |  0.799  |
|    ndarray_zip_with    |  0.282  |
|   slice_loop_safe_bad  |  0.281  |
|  slice_loop_safe_good  |  0.306  |
| slice_loop_unsafe_good |  0.178  |

`i7-4790 3.60GHz` + `Windows 10 17763` + `rustc 1.31.0-nightly (2bd5993ca 2018-10-02) x86_64-pc-windows-gnu` + `Thin LTO` - `incremental` - `overflow-checks`

|          Test          | Time(s) |
|------------------------|---------|
|      ndarray_iter      |  1.096  |
|       slice_iter       |  0.794  |
|    ndarray_zip_with    |  0.281  |
|   slice_loop_safe_bad  |  0.549  |
|  slice_loop_safe_good  |  0.315  |
| slice_loop_unsafe_good |  0.211  |
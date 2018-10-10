#![feature(duration_as_u128)]

#[macro_use]
extern crate ndarray;
use ndarray::{Array1, ArrayView1, ArrayViewMut1};

// iterator over 1d ndarray
fn fn_ndarray_method_1(x: &ArrayView1<f32>, out: &mut ArrayViewMut1<f32>) {
	out.iter_mut().step_by(2).zip(x.iter().skip(1).step_by(2)).for_each(|(a, b)| *a = *b * 123.0);
	out.iter_mut().skip(1).step_by(2).zip(x.iter().step_by(2)).for_each(|(a, b)| *a = *b * 456.0);
}

// use high order methods(zip_with, map) of ndarray
fn fn_ndarray_method_2(x: &ArrayView1<f32>, out: &mut ArrayViewMut1<f32>) {
	out.slice_mut(s![..;2]).zip_mut_with(&x.slice(s![1..;2]), |a, b| *a = *b * 123.0);
	out.slice_mut(s![1..;2]).zip_mut_with(&x.slice(s![..;2]), |a, b| *a = *b * 456.0);
}

// loop over ndarray, bad implemention
fn fn_ndarray_method_3_1(x: &ArrayView1<f32>, out: &mut ArrayViewMut1<f32>) {
	let n_x = x.len();

	assert_eq!(n_x, out.len());

	for i in (0..n_x).step_by(2) {
		out[i] = x[i + 1] * 123.0;
	}

	for i in (1..n_x).step_by(2) {
		out[i] = x[i - 1] * 456.0;
	}
}

// loop over ndarray, good implemention
fn fn_ndarray_method_3_2(x: &ArrayView1<f32>, out: &mut ArrayViewMut1<f32>) {
	let n_x = x.len();

	assert_eq!(n_x, out.len());

	for h_i in 0..n_x / 2 {
		let i = h_i * 2;
		out[i] = x[i + 1] * 123.0;
		out[i + 1] = x[i] * 456.0;
	}
}

// unchecked loop over ndarray, good implemention
fn fn_ndarray_method_3_3(x: &ArrayView1<f32>, out: &mut ArrayViewMut1<f32>) {
	unsafe {
		let n_x = x.len();

		assert_eq!(n_x, out.len());

		for h_i in 0..n_x / 2 {
			let i = h_i * 2;
			*out.uget_mut(i) = *x.uget(i + 1) * 123.0;
			*out.uget_mut(i + 1) = *x.uget(i) * 456.0;
		}
	}
}

// iterator over 1d slice
fn fn_slice_method_1(x: &[f32], out: &mut [f32]) {
	out.iter_mut().step_by(2).zip(x.iter().skip(1).step_by(2)).for_each(|(a, b)| *a = *b * 123.0);
	out.iter_mut().skip(1).step_by(2).zip(x.iter().step_by(2)).for_each(|(a, b)| *a = *b * 456.0);
}

// loop over 1d slice, bad implemention
fn fn_slice_method_3_1(x: &[f32], out: &mut [f32]) {
	let n_x = x.len();

	assert_eq!(n_x, out.len());

	for i in (0..n_x).step_by(2) {
		out[i] = x[i + 1] * 123.0;
	}

	for i in (1..n_x).step_by(2) {
		out[i] = x[i - 1] * 456.0;
	}
}

// loop over 1d slice, good implemention
fn fn_slice_method_3_2(x: &[f32], out: &mut [f32]) {
	let n_x = x.len();

	assert_eq!(n_x, out.len());

	for h_i in 0..n_x / 2 {
		let i = h_i * 2;
		out[i] = x[i + 1] * 123.0;
		out[i + 1] = x[i] * 456.0;
	}
}

// unchecked loop over 1d slice, good implemention
fn fn_slice_method_3_3(x: &[f32], out: &mut [f32]) {
	unsafe {
		let n_x = x.len();

		assert_eq!(n_x, out.len());

		for h_i in 0..n_x / 2 {
			let i = h_i * 2;
			*out.get_unchecked_mut(i) = *x.get_unchecked(i + 1) * 123.0;
			*out.get_unchecked_mut(i + 1) = *x.get_unchecked(i) * 456.0;
		}
	}
}

fn do_test(prefix: &str) {
	{
		let now = std::time::Instant::now();
		let data_1d = Array1::range(0f32, 65536f32, 1f32);
		let mut out = Array1::zeros(65536);
		for _ in 0..8192 {
			fn_ndarray_method_1(&data_1d.view(), &mut out.view_mut());
		}
		println!("{}NDArray 1d_1d iterator: {}", prefix, now.elapsed().as_millis() as f64 / 1000.0);
	}

	{
		let now = std::time::Instant::now();
		let mut data_1d = Vec::<f32>::new();
		data_1d.resize(65536, 0f32);
		data_1d.iter_mut().enumerate().for_each(|(i, x)| *x = i as f32);
		let mut out = Vec::<f32>::new();
		out.resize(65536, 0f32);
		for _ in 0..8192 {
			fn_slice_method_1(&data_1d[..], &mut out[..]);
		}
		println!("{}Slice 1d_1d iterator: {}", prefix, now.elapsed().as_millis() as f64 / 1000.0);
	}

	{
		let now = std::time::Instant::now();
		let data_1d = Array1::range(0f32, 65536f32, 1f32);
		let mut out = Array1::zeros(65536);
		for _ in 0..8192 {
			fn_ndarray_method_2(&data_1d.view(), &mut out.view_mut());
		}
		println!("{}NDArray 1d_1d high order: {}", prefix, now.elapsed().as_millis() as f64 / 1000.0);
	}

	{
		let now = std::time::Instant::now();
		let mut data_1d = Vec::<f32>::new();
		data_1d.resize(65536, 0f32);
		data_1d.iter_mut().enumerate().for_each(|(i, x)| *x = i as f32);
		let mut out = Vec::<f32>::new();
		out.resize(65536, 0f32);
		for _ in 0..8192 {
			fn_slice_method_3_1(&data_1d[..], &mut out[..]);
		}
		println!("{}Slice 1d_1d safe loop_bad: {}", prefix, now.elapsed().as_millis() as f64 / 1000.0);
	}

	{
		let now = std::time::Instant::now();
		let data_1d = Array1::range(0f32, 65536f32, 1f32);
		let mut out = Array1::zeros(65536);
		for _ in 0..8192 {
			fn_ndarray_method_3_1(&data_1d.view(), &mut out.view_mut());
		}
		println!("{}NDArray 1d_1d safe loop_bad: {}", prefix, now.elapsed().as_millis() as f64 / 1000.0);
	}

	{
		let now = std::time::Instant::now();
		let mut data_1d = Vec::<f32>::new();
		data_1d.resize(65536, 0f32);
		data_1d.iter_mut().enumerate().for_each(|(i, x)| *x = i as f32);
		let mut out = Vec::<f32>::new();
		out.resize(65536, 0f32);
		for _ in 0..8192 {
			fn_slice_method_3_2(&data_1d[..], &mut out[..]);
		}
		println!("{}Slice 1d_1d safe loop_good: {}", prefix, now.elapsed().as_millis() as f64 / 1000.0);
	}

	{
		let now = std::time::Instant::now();
		let data_1d = Array1::range(0f32, 65536f32, 1f32);
		let mut out = Array1::zeros(65536);
		for _ in 0..8192 {
			fn_ndarray_method_3_2(&data_1d.view(), &mut out.view_mut());
		}
		println!("{}NDArray 1d_1d safe loop_good: {}", prefix, now.elapsed().as_millis() as f64 / 1000.0);
	}

	{
		let now = std::time::Instant::now();
		let mut data_1d = Vec::<f32>::new();
		data_1d.resize(65536, 0f32);
		data_1d.iter_mut().enumerate().for_each(|(i, x)| *x = i as f32);
		let mut out = Vec::<f32>::new();
		out.resize(65536, 0f32);
		for _ in 0..8192 {
			fn_slice_method_3_3(&data_1d[..], &mut out[..]);
		}
		println!("{}Slice 1d_1d unsafe loop_good: {}", prefix, now.elapsed().as_millis() as f64 / 1000.0);
	}

	{
		let now = std::time::Instant::now();
		let data_1d = Array1::range(0f32, 65536f32, 1f32);
		let mut out = Array1::zeros(65536);
		for _ in 0..8192 {
			fn_ndarray_method_3_3(&data_1d.view(), &mut out.view_mut());
		}
		println!("{}NDArray 1d_1d unsafe loop_good: {}", prefix, now.elapsed().as_millis() as f64 / 1000.0);
	}
}

fn main() {
	do_test("Warm up: ");
	println!("**********************");
	do_test("");
}

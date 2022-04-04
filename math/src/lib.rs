#![cfg_attr(not(std), no_std)]

pub mod mat;
pub mod quat;
pub mod vec;

extern "C" {
	fn sqrt(x: f64) -> f64;
	fn sqrtf(x: f32) -> f32;
	fn abs(x: f64) -> f64;
	fn absf(x: f32) -> f32;
}

pub trait Float {
	fn sqrt(&self) -> Self;
	fn abs(&self) -> Self;
}

impl Float for f64 {
	fn sqrt(&self) -> Self {
		unsafe { sqrt(*self) }
	}

	fn abs(&self) -> Self {
		unsafe { abs(*self) }
	}
}

impl Float for f32 {
	fn sqrt(&self) -> Self {
		unsafe { sqrtf(*self) }
	}

	fn abs(&self) -> Self {
		unsafe { absf(*self) }
	}
}

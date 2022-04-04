use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::Float;

/// The type that is used for Vec2 & Vec3
type FloatType = f64;

macro_rules! expr {
	($e:expr) => {
		$e
	};
}

macro_rules! impl_operator {
	($name:ident, $function_name:ident, $operator:tt) => {
		impl $name for Vec2 {
			type Output = Self;
			#[inline]
			fn $function_name(self, rhs: Self) -> Self {
				Vec2::new(expr!(self.x $operator rhs.x), expr!(self.y $operator rhs.y))
			}
		}
		impl $name for Vec3 {
			type Output = Self;
			#[inline]
			fn $function_name(self, rhs: Self) -> Self {
				Vec3::new(expr!(self.x $operator rhs.x), expr!(self.y $operator rhs.y), expr!(self.z $operator rhs.z))
			}
		}
	};
}

macro_rules! impl_operator_assign {
	($name:ident, $function_name:ident, $operator:tt) => {
		impl $name for Vec2 {
			#[inline]
			fn $function_name(&mut self, rhs: Self) {
				expr!(self.x $operator rhs.x);
				expr!(self.y $operator rhs.y);
			}
		}
		impl $name for Vec3 {
			#[inline]
			fn $function_name(&mut self, rhs: Self) {
				expr!(self.x $operator rhs.x);
				expr!(self.y $operator rhs.y);
				expr!(self.z $operator rhs.z);
			}
		}
	};
}

macro_rules! impl_operator_float {
	($name:ident, $function_name:ident, $operator:tt) => {
		impl $name<FloatType> for Vec2 {
			type Output = Self;
			#[inline]
			fn $function_name(self, rhs: FloatType) -> Self {
				Vec2::new(expr!(self.x $operator rhs), expr!(self.y $operator rhs))
			}
		}
		impl $name<Vec2> for FloatType {
			type Output = Vec2;
			#[inline]
			fn $function_name(self, rhs: Vec2) -> Vec2 {
				Vec2::new(expr!(self $operator rhs.x), expr!(self $operator rhs.y))
			}
		}
		impl $name<FloatType> for Vec3 {
			type Output = Self;
			#[inline]
			fn $function_name(self, rhs: FloatType) -> Self {
				Vec3::new(expr!(self.x $operator rhs), expr!(self.y $operator rhs), expr!(self.z $operator rhs))
			}
		}
		impl $name<Vec3> for FloatType {
			type Output = Vec3;
			#[inline]
			fn $function_name(self, rhs: Vec3) -> Vec3 {
				Vec3::new(expr!(self $operator rhs.x), expr!(self $operator rhs.y), expr!(self $operator rhs.z))
			}
		}
	};
}

macro_rules! impl_operator_float_assign {
	($name:ident, $function_name:ident, $operator:tt) => {
		impl $name<FloatType> for Vec2 {
			fn $function_name(&mut self, rhs: FloatType) {
				expr!(self.x $operator rhs);
				expr!(self.y $operator rhs);
			}
		}
		impl $name<FloatType> for Vec3 {
			fn $function_name(&mut self, rhs: FloatType) {
				expr!(self.x $operator rhs);
				expr!(self.y $operator rhs);
				expr!(self.z $operator rhs);
			}
		}
	};
}

/// Three component vector type
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[repr(C)]
pub struct Vec3 {
	pub x: FloatType,
	pub y: FloatType,
	pub z: FloatType,
}

/// Two component vector type
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[repr(C)]
pub struct Vec2 {
	pub x: FloatType,
	pub y: FloatType,
}

impl Vec3 {
	/// Creates a new Vec3
	#[inline]
	pub const fn new(x: FloatType, y: FloatType, z: FloatType) -> Self {
		Vec3 { x, y, z }
	}

	/// Returns a Vec3 with 1.0 as all the components
	#[inline]
	pub const fn one() -> Self {
		Vec3::new(1.0, 1.0, 1.0)
	}

	/// Returns a Vec3 with 0.0 as all the components
	#[inline]
	pub const fn zero() -> Self {
		Vec3::new(0.0, 0.0, 0.0)
	}

	/// Calculates a dot product with another vector
	#[inline]
	pub fn dot(&self, other: Self) -> FloatType {
		self.x * other.x + self.y * other.y + self.z * other.z
	}

	/// Calculates a cross product with another vector
	#[inline]
	pub fn cross(&self, other: Self) -> Self {
		Vec3::new(
			self.y * other.z - self.z * other.y,
			self.z * other.x - self.x * other.z,
			self.x * other.y - self.y * other.x,
		)
	}

	/// Returns the square of magnitude of the vector
	#[inline]
	pub fn mag_sq(&self) -> FloatType {
		self.dot(*self)
	}

	/// Returns the magnitude of the vector
	#[inline]
	pub fn mag(&self) -> FloatType {
		self.dot(*self).sqrt()
	}

	/// Normalises the vector
	#[inline]
	pub fn normalise(&mut self) {
		*self /= self.mag();
	}

	/// Returns a normalised version of the vector
	#[inline]
	pub fn normalised(self) -> Self {
		self / self.mag()
	}

	/// Returns a new vector with the absolute value of each component
	#[inline]
	pub fn abs(self) -> Self {
		Vec3::new(self.x.abs(), self.y.abs(), self.z.abs())
	}

	/// Returns the minimum component of the vector
	#[inline]
	pub fn component_min(self) -> FloatType {
		self.x.min(self.y.min(self.z))
	}

	/// Returns the maximum component of the vector
	#[inline]
	pub fn component_max(self) -> FloatType {
		self.x.max(self.y.max(self.z))
	}

	/// Returns a new vector with the minimum component value between the
	/// vectors
	#[inline]
	pub fn min_by_component(self, other: Self) -> Self {
		Vec3::new(
			self.x.min(other.x),
			self.y.min(other.y),
			self.z.min(other.z),
		)
	}

	/// Returns a new vector with the maximum component value between the
	/// vectors
	#[inline]
	pub fn max_by_component(self, other: Self) -> Self {
		Vec3::new(
			self.x.max(other.x),
			self.y.max(other.y),
			self.z.max(other.z),
		)
	}
}

impl Vec2 {
	/// Creates a new Vec2
	#[inline]
	pub const fn new(x: FloatType, y: FloatType) -> Self {
		Vec2 { x, y }
	}

	/// Returns a Vec2 with 1.0 as all the components
	#[inline]
	pub const fn one() -> Self {
		Vec2::new(1.0, 1.0)
	}

	/// Returns a Vec2 with 0.0 as all the components
	#[inline]
	pub const fn zero() -> Self {
		Vec2::new(0.0, 0.0)
	}

	/// Calculates a dot product with another vector
	#[inline]
	pub fn dot(&self, other: Self) -> FloatType {
		self.x * other.x + self.y * other.y
	}

	/// Returns the square of magnitude of the vector
	#[inline]
	pub fn mag_sq(&self) -> FloatType {
		self.dot(*self)
	}

	/// Returns the magnitude of the vector
	#[inline]
	pub fn mag(&self) -> FloatType {
		self.dot(*self).sqrt()
	}

	/// Normalises the vector
	#[inline]
	pub fn normalise(&mut self) {
		*self /= self.mag();
	}

	/// Returns a normalised version of the vector
	#[inline]
	pub fn normalised(self) -> Self {
		self / self.mag()
	}

	/// Returns a new vector with the absolute value of each component
	#[inline]
	pub fn abs(self) -> Self {
		Vec2::new(self.x.abs(), self.y.abs())
	}

	/// Returns the minimum component of the vector
	#[inline]
	pub fn component_min(self) -> FloatType {
		self.x.min(self.y)
	}

	/// Returns a new vector with the maximum component value between the
	/// vectors
	#[inline]
	pub fn component_max(self) -> FloatType {
		self.x.max(self.y)
	}

	/// Returns a new vector with the minimum component value between the
	/// vectors
	#[inline]
	pub fn min_by_component(self, other: Self) -> Self {
		Vec2::new(self.x.min(other.x), self.y.min(other.y))
	}

	/// Returns a new vector with the maximum component value between the
	/// vectors
	#[inline]
	pub fn max_by_component(self, other: Self) -> Self {
		Vec2::new(self.x.max(other.x), self.y.max(other.y))
	}
}

impl_operator!(Add, add, +);
impl_operator_assign!(AddAssign, add_assign, +=);
impl_operator_float!(Add, add, +);
impl_operator_float_assign!(AddAssign, add_assign, +=);

impl_operator!(Sub, sub, -);
impl_operator_assign!(SubAssign, sub_assign, -=);
impl_operator_float!(Sub, sub, -);
impl_operator_float_assign!(SubAssign, sub_assign, -=);

impl_operator!(Mul, mul, *);
impl_operator_assign!(MulAssign, mul_assign, *=);
impl_operator_float!(Mul, mul, *);
impl_operator_float_assign!(MulAssign, mul_assign, *=);

impl_operator!(Div, div, /);
impl_operator_assign!(DivAssign, div_assign, /=);
impl_operator_float!(Div, div, /);
impl_operator_float_assign!(DivAssign, div_assign, /=);

impl Neg for Vec3 {
	type Output = Self;
	#[inline]
	fn neg(self) -> Self {
		Vec3::new(-self.x, -self.y, -self.z)
	}
}

impl Neg for Vec2 {
	type Output = Self;
	#[inline]
	fn neg(self) -> Self {
		Vec2::new(-self.x, -self.y)
	}
}

impl core::fmt::Display for Vec3 {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(f, "({}, {}, {})", self.x, self.y, self.z)
	}
}

impl core::fmt::Display for Vec2 {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(f, "({}, {})", self.x, self.y)
	}
}

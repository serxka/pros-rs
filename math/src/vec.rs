use crate::FloatMath;

use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use paste::paste;

macro_rules! size_match {
	($base:expr, repeat $rep:expr, take 2) => {
		$base($rep, $rep)
	};
	($base:expr, repeat $rep:expr, take 3) => {
		$base($rep, $rep, $rep)
	};
	($base:expr, repeat $rep:expr, take 4) => {
		$base($rep, $rep, $rep, $rep)
	};
}

macro_rules! impl_vec {
	($name:ident, $type:ty, $num:tt, $($field:ident),*) => {
			/// A Vector type
			#[derive(Copy, Clone, Debug, PartialEq, Default)]
			pub struct $name {
				$(
					pub $field: $type,
				)*
			}

			impl $name {
				/// Creates a new Vector.
				#[inline]
				pub const fn new($( $field: $type, )*) -> Self {
					Self { $($field,)* }
				}
				/// Creates a new Vector with all components as zero.
				#[inline]
				pub const fn zero($( $field: $type, )*) -> Self {
					size_match!(Self::new, repeat 0.0, take $num)
				}
				/// Creates a new Vector with all components as one.
				#[inline]
				pub const fn one($( $field: $type, )*) -> Self {
					size_match!(Self::new, repeat 1.0, take $num)
				}
				/// Calculates a dot product with another vector.
				#[inline]
				pub fn dot(&self, other: Self) -> $type {
					$(
						self.$field * other.$field +
					)* 0.0 // match off last addition
				}
				/// Returns the square of magnitude of the vector.
				#[inline]
				pub fn mag_sq(&self) -> $type {
					self.dot(*self)
				}
				/// Returns the magnitude of the vector.
				#[inline]
				pub fn mag(&self) -> $type {
					self.dot(*self).sqrt()
				}
				/// Normalises the vector in place.
				#[inline]
				pub fn normalise(&mut self) {
					*self /= self.mag();
				}
				/// Returns a normalised version of the vector.
				#[inline]
				pub fn normalised(self) -> Self {
					self / self.mag()
				}
				/// Returns a new vector with the absolute value of each component.
				#[inline]
				pub fn abs(self) -> Self {
					Self::new($( self.$field.abs(), )*)
				}
				/// Returns the minimum component of the vector.
				#[inline]
				pub fn component_min(self) -> $type {
					[ $( self.$field, )* ].into_iter().reduce(<$type>::min).unwrap()
				}
				/// Returns the maximum component of the vector.
				#[inline]
				pub fn component_max(self) -> $type {
					[ $( self.$field, )* ].into_iter().reduce(<$type>::max).unwrap()
				}
				/// Returns a new vector with the minimum component value between the two
				/// vectors.
				#[inline]
				pub fn min_by_component(self, other: Self) -> Self {
					Self::new( $( self.$field.min(other.$field), )* )
				}
				/// Returns a new vector with the maximum component value between the two
				/// vectors.
				#[inline]
				pub fn max_by_component(self, other: Self) -> Self {
					Self::new( $( self.$field.max(other.$field), )* )
				}
			}

			impl Neg for $name {
				type Output = Self;
				#[inline]
				fn neg(self) -> Self {
					Self::new( $( -self.$field, )*)
				}
			}

			impl_vec!($name, $type, $($field,)* |impl| Add +);
			impl_vec!($name, $type, $($field,)* |impl| Sub -);
			impl_vec!($name, $type, $($field,)* |impl| Mul *);
			impl_vec!($name, $type, $($field,)* |impl| Div /);
			impl_vec!($name, $type, $($field,)* |impl shorthand| Mul *);
			impl_vec!($name, $type, $($field,)* |impl shorthand| Div /);

			impl_vec!($name, display: $num, $($field,)*);
	};
	($name:ident, $type:ty, $($field:ident),*, |impl| $op:ident $op_tok:tt) => {
		paste! {
			impl $op for $name {
				type Output = Self;
				#[inline]
				fn [<$op:lower>](self, rhs: Self) -> Self {
					Self::new(
						$(
							self.$field $op_tok rhs.$field,
						)*
					)
				}
			}
			impl [<$op Assign>] for $name {
				#[inline]
				fn [<$op:lower _assign>](&mut self, rhs: Self) {
					$(
						self.$field = self.$field $op_tok rhs.$field;
					)*
				}
			}
		}
	};
	($name:ident, $type:ty, $($field:ident),*, |impl shorthand| $op:ident $op_tok:tt) => {
		paste !{
			impl $op<$type> for $name {
				type Output = Self;
				#[inline]
				fn [<$op:lower>](self, rhs: $type) -> Self {
					Self::new(
						$(
							self.$field $op_tok rhs,
						)*
					)
				}
			}
			impl [<$op Assign>]<$type> for $name {
				#[inline]
				fn [<$op:lower _assign>](&mut self, rhs: $type) {
					$(
						self.$field = self.$field $op_tok rhs;
					)*
				}
			}
		}
	};
	// :( replace this bullshittery with proper proc macros B|
	($name:ident, display: 2, $($field:ident),*,) => {
		impl core::fmt::Display for $name {
			fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
				write!(f, "({}, {})", $(self.$field,)* )
			}
		}
	};
	($name:ident, display: 3, $($field:ident),*,) => {
		impl core::fmt::Display for $name {
			fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
				write!(f, "({}, {}, {})", $(self.$field,)* )
			}
		}
	};
	($name:ident, display: 4, $($field:ident),*,) => {
		impl core::fmt::Display for $name {
			fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
				write!(f, "({}, {}, {}, {})", $(self.$field,)* )
			}
		}
	};
}

impl_vec!(DVec2, f64, 2, x, y);
impl_vec!(Vec2, f32, 2, x, y);
impl_vec!(DVec3, f64, 3, x, y, z);
impl_vec!(Vec3, f32, 3, x, y, z);
impl_vec!(DVec4, f64, 4, x, y, z, w);
impl_vec!(Vec4, f32, 4, x, y, z, w);

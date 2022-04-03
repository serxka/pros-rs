use crate::bindings;
use crate::rtos::{
	action::{Action, NextSleep, Poll},
	tasks::Task,
};

use core::ops::{Add, AddAssign, Sub};
use core::time::Duration;

/// A sample of a monotonically nondecreasing clock running from the start of
/// program execution. Used to represent a point in time of the programs
/// operation. Stored internally with a 1 microsecond precision.
///
/// This type is similar to the on in `std` however it less strict and is more
/// aimed for marking times throughout the programs execution.
#[derive(Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
pub struct Instant(u64);

impl Instant {
	/// Get the current time in the programs execution.
	pub fn now() -> Instant {
		let micros = unsafe { bindings::micros() };
		Instant(micros)
	}

	/// Create a new instant as measured from microseconds from the beginning on
	/// the programs execution.
	#[inline]
	pub const fn from_micros(micros: u64) -> Self {
		Self(micros)
	}

	/// Create a new instant as measured from milliseconds from the beginning on
	/// the programs execution.
	///
	/// # Panics
	/// Will panic if the amount of milliseconds will overflow u64::MAX
	/// microseconds.
	#[inline]
	pub const fn from_millis(millis: u64) -> Self {
		Self(
			millis
				.checked_mul(1000)
				.expect("overflow when creating instant from milliseconds"),
		)
	}

	/// Get this `Instant` as its microsecond representation.
	#[inline]
	pub fn as_micros(&self) -> u64 {
		self.0
	}

	/// Get this `Instant` as its millisecond representation.
	#[inline]
	pub fn as_millis(&self) -> u64 {
		self.0 / 1000
	}

	/// Returns the fractional part of this `Instant`, in milliseconds.
	///
	/// This does not return the time in milliseconds but rather the fractional
	/// part of the current second.
	///
	/// # Examples
	/// ```
	/// let time = Instant::from_millis(1700);
	/// assert_eq!(700, time.frac_millis())
	#[inline]
	pub fn frac_millis(&self) -> u64 {
		self.0 % 1000000 / 1000
	}

	/// Returns the fractional part of this `Instant`, in microseconds.
	///
	/// This does not return the time in microseconds but rather the fractional
	/// part of the current second.
	///
	/// # Examples
	/// ```
	/// let time = Instant::from_micros(1005400);
	/// assert_eq!(5400, time.frac_micros())
	#[inline]
	pub fn frac_micros(&self) -> u64 {
		self.0 % 1000000
	}

	/// Perform a checked addition of the `rhs` Duration onto this `Instant`.
	/// This function will return `None` if the addition would have resulted in
	/// an overflow.
	pub fn checked_add(&self, rhs: Duration) -> Option<Self> {
		Some(Self(self.0.checked_add(rhs.as_micros().try_into().ok()?)?))
	}

	/// Perform a checked subtraction of the `rhs` Duration onto this `Instant`.
	/// This function will return `None` if the subtraction would have resulted
	/// in an underflow.
	pub fn checked_sub(&self, rhs: Duration) -> Option<Self> {
		Some(Self(self.0.checked_sub(rhs.as_micros().try_into().ok()?)?))
	}

	/// Perform a checked subtraction of the `rhs` Instant onto this `Instant`.
	/// This function will return `None` if the subtraction would have resulted
	/// in an underflow.
	pub fn checked_sub_instant(&self, rhs: Self) -> Option<Self> {
		Some(Self(self.0.checked_sub(rhs.0)?))
	}

	/// Return how long has elapsed since the time recorded in this `Instant`. A
	/// panic will occur if `self` measures a time in the future that has not
	/// yet occurred.
	pub fn elapsed(&self) -> Duration {
		Duration::from_micros((Instant::now() - *self).as_millis())
	}

	/// Return the duration between this `Instant` and an earlier `Instant`.
	pub fn duration_since(&self, earlier: Instant) -> Duration {
		Duration::from_micros(
			self.checked_sub_instant(earlier)
				.expect("supplied instant is later then self")
				.0,
		)
	}
}

impl Add<Duration> for Instant {
	type Output = Instant;

	fn add(self, rhs: Duration) -> Self::Output {
		self.checked_add(rhs)
			.expect("overflow when adding duration to instant")
	}
}

impl AddAssign<Duration> for Instant {
	fn add_assign(&mut self, rhs: Duration) {
		*self = *self + rhs;
	}
}

impl Sub<Duration> for Instant {
	type Output = Instant;

	fn sub(self, rhs: Duration) -> Self::Output {
		self.checked_sub(rhs)
			.expect("underflow when subtracting duration from instant")
	}
}

impl Sub<Instant> for Instant {
	type Output = Instant;

	fn sub(self, rhs: Instant) -> Self::Output {
		self.checked_sub_instant(rhs)
			.expect("underflow when subtracting instant from instant")
	}
}

pub struct Interval {
	period: Duration,
	last: Instant,
}

impl Interval {
	pub fn new(period: Duration) -> Self {
		Interval {
			period,
			last: Instant::now(),
		}
	}

	pub fn delay(&mut self) {
		// Check if we do actually need to delay for our next period
		if let Some(t) = (self.last + self.period).checked_sub_instant(Instant::now()) {
			Task::delay(Duration::from_micros(t.as_micros()));
		}
		self.last += self.period;
	}

	pub fn action(&'_ mut self) -> impl Action + '_ {
		struct IntervalAction<'a>(&'a mut Interval);

		impl<'a> Action for IntervalAction<'a> {
			type Output = ();

			// If we have reached the time for out next interval to be triggered, we should
			// return that we are now ready. Otherwise we aren't ready and we should sleep
			// until we will be ready next.
			fn poll(&mut self) -> Poll<Self::Output> {
				if (self.0.last + self.0.period)
					.checked_sub_instant(Instant::now())
					.is_some()
				{
					Poll::Waiting
				} else {
					Poll::Complete(())
				}
			}

			// If this gets called it is assumed that we are not yet complete so we must
			// have some time that we need to wait for.
			fn next(&self) -> NextSleep {
				NextSleep::Timestamp(
					(self.0.last + self.0.period)
						.checked_sub_instant(Instant::now())
						.unwrap(),
				)
			}
		}

		IntervalAction(self)
	}
}

//! TriPort expander, allows more TriPorts connected to a V5 Brain.

use crate::ports::{Port, TriPort};

/// A struct which holds a list of all the TriPorts created on a TriPort
/// expander. They can be taken out of their options and used else where at
/// anytime afterwards.
///
/// # Examples
/// ```
/// let mut port_a = expander.a.take().unwrap();
/// port_a.into_digital_out(true);
/// ```
pub struct TriPortExpander {
	pub port: Port,
	pub a: Option<TriPort>,
	pub b: Option<TriPort>,
	pub c: Option<TriPort>,
	pub d: Option<TriPort>,
	pub e: Option<TriPort>,
	pub f: Option<TriPort>,
	pub g: Option<TriPort>,
	pub h: Option<TriPort>,
}

impl TriPortExpander {
	/// Create a new TriPort expander on this port.
	///
	/// # Panics
	/// This function will panic if a TriPort expander is not connected on this
	/// port.
	///
	/// # Safety
	/// There must only ever be a single reference to this object. It is up to
	/// the caller to make sure there does not exists another device object with
	/// the same port. If there is another device object with the same port this
	/// will result in undefined behaviour and/or panics.
	pub unsafe fn new(port: Port) -> Self {
		TriPortExpander {
			a: Some(TriPort::new(1, Some(port.clone()))),
			b: Some(TriPort::new(2, Some(port.clone()))),
			c: Some(TriPort::new(3, Some(port.clone()))),
			d: Some(TriPort::new(4, Some(port.clone()))),
			e: Some(TriPort::new(5, Some(port.clone()))),
			f: Some(TriPort::new(6, Some(port.clone()))),
			g: Some(TriPort::new(7, Some(port.clone()))),
			h: Some(TriPort::new(8, Some(port.clone()))),
			port,
		}
	}
}

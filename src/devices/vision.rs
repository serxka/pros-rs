use smallvec::SmallVec;

use crate::bindings::*;
use crate::devices::DeviceError;
use crate::ports::Port;
use crate::util::PROS_ERR_VISION_OBJECT_SIG;

/// A struct which holds and presents a connected VEX Vision sensor connected
/// to the V5 Brain.
pub struct Vision {
	pub port: Port,
}

impl Vision {
	pub const FOV_WIDTH: f64 = 316.0;
	pub const FOV_HEIGHT: f64 = 212.0;

	/// Create a new vision sensor object. This will initialise the vision
	/// sensor with the zero point set to [`ZeroPoint::default()`].
	///
	/// # Errors
	/// This function will return an error if the supplied port is not a
	/// vision sensor.
	///
	/// # Safety
	/// There must only ever be a single reference to this sensor. It is up to
	/// the caller to make sure there does not exists another device object with
	/// the same port. If there is another device object with the same port this
	/// will result in undefined behaviour and/or panics.
	pub unsafe fn new(port: Port) -> Result<Self, DeviceError> {
		let mut vision = Vision { port };
		vision.set_zero_point(ZeroPoint::default())?;
		Ok(vision)
	}

	#[inline]
	fn get_port(&self) -> u8 {
		self.port.get()
	}

	/// Set the `(0,0)` coordinate to for the field of view. This will affect
	/// the coordinates provided in [`Object`] structures. It is recommended to
	/// call this function once before using the sensor and maintaining a
	/// consistent zero point.
	///
	/// # Errors
	/// This function will return [`DeviceError::PortNotVisionSensor`] if this
	/// device is no longer a vision sensor.
	///
	/// # Examples
	/// Setting the zero point before use:
	/// ```rust
	/// let mut camera = device.take_port(2)?.into_vision()?;
	/// camera.set_zero_point(ZeroPoint::TopLeft)?;
	/// /* other setup */
	/// ```
	pub fn set_zero_point(&mut self, zero: ZeroPoint) -> Result<(), DeviceError> {
		pros_unsafe_err!(
			vision_set_zero_point,
			err = DeviceError::errno_vision(),
			self.get_port(),
			zero.into()
		)?;
		Ok(())
	}

	pub fn create_colour_code(
		&mut self,
		sig1: SignatureId,
		sig2: SignatureId,
		sig3: Option<SignatureId>,
		sig4: Option<SignatureId>,
		sig5: Option<SignatureId>,
	) -> Result<ColourCode, DeviceError> {
		let conv = |sig: Option<SignatureId>| sig.map(|s| s as _).unwrap_or(0);
		match unsafe {
			vision_create_color_code(
				self.get_port(),
				sig1 as _,
				sig2 as _,
				conv(sig3),
				conv(sig4),
				conv(sig5),
			)
		} {
			x if x == PROS_ERR_VISION_OBJECT_SIG as _ => Err(DeviceError::errno_vision()),
			x => Ok(ColourCode(x)),
		}
	}

	/// Upload the specified [`Signature`] onto the vision sensor under the
	/// specified signature ID. This signature is saved in volatile memory
	/// on the sensor, it will be lost as soon as the sensor is loses power.
	pub fn set_signature(
		&mut self,
		id: SignatureId,
		signature: Signature,
	) -> Result<(), DeviceError> {
		pros_unsafe_err!(
			vision_set_signature,
			err = DeviceError::errno_vision(),
			self.get_port(),
			id as _,
			&signature as *const _ as *mut _ // this pointer is meant to be const
		)?;
		Ok(())
	}

	/// Enable of disable Wi-Fi streaming of the video from the vision sensor.
	pub fn set_wifi_mode(&mut self, enabled: bool) -> Result<(), DeviceError> {
		pros_unsafe_err!(
			vision_set_wifi_mode,
			err = DeviceError::errno_vision(),
			self.get_port(),
			enabled as u8
		)?;
		Ok(())
	}

	/// Set the white balance of the vision sensor manually. This will disable
	/// the automatic white balancing on the sensor. To re-enable automatic
	/// white balancing call [`Vision::enable_auto_white_balance()`].
	pub fn set_white_balance(&mut self, rgb: i32) -> Result<(), DeviceError> {
		pros_unsafe_err!(
			vision_set_white_balance,
			err = DeviceError::errno_vision(),
			self.get_port(),
			rgb
		)?;
		Ok(())
	}

	/// Set the exposure of the vision sensor. `exposure` must be value in the
	/// range of `[0, 150]`. This is enforced by a debug assertion and clamped
	/// at runtime.
	pub fn set_exposure(&mut self, exposure: u8) -> Result<(), DeviceError> {
		debug_assert!(exposure <= 150);
		pros_unsafe_err!(
			vision_set_white_balance,
			err = DeviceError::errno_vision(),
			self.get_port(),
			exposure.clamp(0, 150) as _
		)?;
		Ok(())
	}

	pub fn set_led(&mut self, rgb: i32) -> Result<(), DeviceError> {
		pros_unsafe_err!(
			vision_set_led,
			err = DeviceError::errno_vision(),
			self.get_port(),
			rgb
		)?;
		Ok(())
	}

	pub fn clear_led(&self) -> Result<(), DeviceError> {
		pros_unsafe_err!(
			vision_clear_led,
			err = DeviceError::errno_vision(),
			self.get_port()
		)?;
		Ok(())
	}

	/// Enable automatic white balancing on the vision sensor.
	pub fn enable_auto_white_balance(&mut self) -> Result<(), DeviceError> {
		pros_unsafe_err!(
			vision_set_auto_white_balance,
			err = DeviceError::errno_vision(),
			self.get_port(),
			true as _
		)?;
		Ok(())
	}

	/// For the given signature ID return the [`Signature`] object stored on
	/// the vision sensor. This will return `None` if an error occurs or the
	/// signature ID does not have a vision signature associated with it.
	pub fn get_signature(&self, id: SignatureId) -> Option<Signature> {
		match pros_unsafe_err_sig!(
			vision_get_signature,
			err = DeviceError::errno_vision(),
			self.get_port(),
			id as _
		) {
			Err(_) => None,
			Ok(sig) => Some(Signature(sig)),
		}
	}

	/// Get the white balance of the sensor.
	pub fn get_white_balance(&self) -> Result<i32, DeviceError> {
		pros_unsafe_err!(
			vision_get_white_balance,
			err = DeviceError::errno_vision(),
			self.get_port()
		)
	}

	/// Get the exposure of the sensor. Values will be returned in the range
	/// of `[0, 150]`.
	pub fn get_exposure(&self) -> Result<u8, DeviceError> {
		pros_unsafe_err!(
			vision_get_exposure,
			err = DeviceError::errno_vision(),
			self.get_port()
		)
		.map(|i| i as _)
	}

	/// Get the count of objects currently detected by the vision sensor.
	pub fn get_object_count(&self) -> Result<u32, DeviceError> {
		pros_unsafe_err!(
			vision_get_object_count,
			err = DeviceError::errno_vision(),
			self.get_port()
		)
		.map(|i| i as _)
	}

	pub fn get_by_sig(
		&self,
		size_idx: u32,
		signature_id: SignatureId,
	) -> Result<Object, DeviceError> {
		let raw = unsafe { vision_get_by_sig(self.get_port(), size_idx, signature_id as _) };
		match Object::from_raw(raw) {
			None => Err(DeviceError::errno_vision()),
			Some(obj) => Ok(obj),
		}
	}

	pub fn get_by_size(&self, size_idx: u32) -> Result<Object, DeviceError> {
		let raw = unsafe { vision_get_by_size(self.get_port(), size_idx) };
		match Object::from_raw(raw) {
			None => Err(DeviceError::errno_vision()),
			Some(obj) => Ok(obj),
		}
	}

	pub fn get_by_code(
		&self,
		size_idx: u32,
		colour_code: ColourCode,
	) -> Result<Object, DeviceError> {
		let raw = unsafe { vision_get_by_code(self.get_port(), size_idx, colour_code.as_raw()) };
		match Object::from_raw(raw) {
			None => Err(DeviceError::errno_vision()),
			Some(obj) => Ok(obj),
		}
	}

	pub fn read_by_sig(
		&self,
		size_id: u32,
		sig_id: u32,
		object_count: u32,
	) -> Result<SmallVec<[Object; 4]>, DeviceError> {
		let mut vec: SmallVec<[vision_object_s_t; 4]> =
			SmallVec::with_capacity(object_count as usize);

		let num_detected = pros_unsafe_err!(
			vision_read_by_sig,
			err = DeviceError::errno_vision(),
			self.get_port(),
			size_id,
			sig_id,
			object_count,
			vec.as_mut_ptr()
		)?;

		// we have to set the length of the SmallVec manually becuase the
		// C bindings will add the objects but not incrememnt the length
		unsafe { vec.set_len(num_detected as usize) }

		let vec = vec
			.into_iter()
			.map(Object::from_raw)
			.collect::<Option<SmallVec<_>>>();

		// this should never fail since pros should always return a valid
		// vision_object_s_t
		if let Some(vec) = vec {
			Ok(vec)
		} else {
			unreachable!()
		}
	}
}

#[derive(Clone)]
pub struct Object {
	/// Object signature
	pub signature: SignatureId,
	/// Object type
	pub obj_type: ObjectType,
	/// Left boundary coordinate of the object
	pub left_coord: i16,
	/// Top boundary coordinate of the object
	pub top_coord: i16,
	/// Width of the object
	pub width: i16,
	/// Height of the object
	pub height: i16,
	/// Angle of a colour code object in 0.1 degree units (e.g. 10 -> 1 degree)
	pub angle: u16,

	/// Coordinates of the middle of the object, computed from values above
	pub middle_coord: (i16, i16),
}

impl Object {
	/// Convert a plain vision object from the PROS API into a proper Rust type.
	/// Returns an option in the case where an object is invalid.
	pub fn from_raw(raw: vision_object_s_t) -> Option<Self> {
		Some(Object {
			signature: SignatureId::from_u8(raw.signature.try_into().ok()?)?,
			obj_type: ObjectType::from_u8(raw.type_)?,
			left_coord: raw.left_coord,
			top_coord: raw.top_coord,
			width: raw.width,
			height: raw.height,
			angle: raw.angle,
			middle_coord: (raw.x_middle_coord, raw.y_middle_coord),
		})
	}
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ObjectType {
	Normal,
	ColourCode,
	Line,
}

impl ObjectType {
	pub(crate) fn from_u8(i: u8) -> Option<Self> {
		#[allow(non_upper_case_globals)]
		match i as _ {
			vision_object_type_E_VISION_OBJECT_NORMAL => Some(Self::Normal),
			vision_object_type_E_VISION_OBJECT_COLOR_CODE => Some(Self::ColourCode),
			vision_object_type_E_VISION_OBJECT_LINE => Some(Self::Line),
			_ => None,
		}
	}
}

#[repr(transparent)]
#[derive(Clone)]
pub struct Signature(vision_signature);

impl Signature {
	pub fn create(
		id: SignatureId,
		u_min: i32,
		u_max: i32,
		u_mean: i32,
		v_min: i32,
		v_max: i32,
		v_mean: i32,
		range: f32,
		sig_type: i32,
	) -> Self {
		let sig = unsafe {
			vision_signature_from_utility(
				id as _, u_min, u_max, u_mean, v_min, v_max, v_mean, range, sig_type,
			)
		};
		Self(sig)
	}

	pub fn get_id(&self) -> SignatureId {
		SignatureId::from_u8(self.0.id).unwrap_or_else(|| unreachable!())
	}
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SignatureId {
	Sig1 = 1,
	Sig2,
	Sig3,
	Sig4,
	Sig5,
	Sig6,
	Sig7,
}

impl SignatureId {
	pub(crate) fn from_u8(i: u8) -> Option<Self> {
		match i {
			1 => Some(Self::Sig1),
			2 => Some(Self::Sig2),
			3 => Some(Self::Sig3),
			4 => Some(Self::Sig4),
			5 => Some(Self::Sig5),
			6 => Some(Self::Sig6),
			7 => Some(Self::Sig7),
			_ => None,
		}
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct ColourCode(u16);

impl ColourCode {
	pub fn as_raw(self) -> u16 {
		self.0
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ZeroPoint {
	TopLeft,
	Centre,
}

impl Default for ZeroPoint {
	fn default() -> Self {
		Self::Centre
	}
}

impl From<ZeroPoint> for vision_zero_e_t {
	fn from(x: ZeroPoint) -> Self {
		match x {
			ZeroPoint::TopLeft => vision_zero_E_VISION_ZERO_TOPLEFT,
			ZeroPoint::Centre => vision_zero_E_VISION_ZERO_CENTER,
		}
	}
}

pub trait Robot {
	fn new() -> Self;

	/// Formally `competition_initalize` in pros
	fn initialize(&self);

	fn disabled(&self);

	fn autonomous(&self);

	fn opcontrol(&self);
}

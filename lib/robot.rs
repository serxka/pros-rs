pub trait Robot {
	fn new() -> Self;

	fn competition_init(&mut self);

	fn disabled(&mut self);

	fn autonomous(&mut self);

	fn opcontrol(&mut self);
}

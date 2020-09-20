#[derive(Debug, Default)]
pub struct MiniRwLock(parking_lot::Mutex<()>);

impl MiniRwLock {
	#[inline]
	pub fn read<'a>(&'a self) -> impl Drop + 'a {
		self.0.lock()
	}

	#[inline]
	pub fn write<'a>(&'a self) -> impl Drop + 'a {
		self.0.lock()
	}
}

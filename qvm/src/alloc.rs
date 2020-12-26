/// Allocates `cap` bytes of memory.
pub fn alloc(cap: usize) -> *mut u8 {
	unsafe {
		// SAFETY: ¯\_(ツ)_/¯ 
		libc::malloc(cap) as *mut u8
	}
}

/// Frees memory pointed to by `ptr`.
///
/// SAFETY: Normal `dealloc` things, like don't double free, etc.
pub unsafe fn dealloc(ptr: *mut u8) {
	libc::free(ptr as *mut _)
}

/// Reallocates `ptr` if need be.
///
/// SAFETY: Normal `dealloc` things, like don't double free, etc.
pub unsafe fn realloc(ptr: *mut u8, cap: usize) -> *mut u8 {
	libc::realloc(ptr as *mut _, cap) as *mut u8
}

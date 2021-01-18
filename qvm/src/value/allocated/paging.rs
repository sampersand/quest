pub fn initialize() {}
/*use super::Allocated;
use parking_lot::RwLock;

struct Pages {
	pages: Vec<*mut Allocated>,
	current_page: usize,
	index_in_current_page: usize
}

static mut PAGE_SIZE: usize = 0;
static PAGES: RwLock<Pages> = RwLock::new(Pages {
	pages: Vec::new(),
	current_page: 0,
	index_in_current_page: 0
});

const PAGES_TO_PREALLOCATE: usize = 8192; // arbitrary constant.

pub fn initialize() {
	// SAFETY: we only set page size upon initialization, which is done in one thread, so we know this is thread safe.
	unsafe {
		if PAGE_SIZE != 0 {
			trace!(?PAGE_SIZE, "page size was already initialized");
			return;
		}

		PAGE_SIZE = page_size::get();
		debug!(?PAGE_SIZE, "initialized page size");
	}

	let mut pages = PAGES.write();
	// pages.
}

/// Get a pointer to a place where you can write Allocated data to.
pub fn allocate() -> *mut Allocated {
	todo!()
}


/*/// Allocates `cap` bytes of memory.
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
*/
*/

use std::alloc::{alloc, dealloc, Layout};
use std::collections::HashMap;
use std::sync::Mutex;

/// Global registry: tracks every allocation's layout so we can free it properly.
static ALLOC_MAP: Mutex<Option<HashMap<usize, Layout>>> = Mutex::new(None);

fn registry() -> std::sync::MutexGuard<'static, Option<HashMap<usize, Layout>>> {
    ALLOC_MAP.lock().unwrap()
}

/// Allocate `size` bytes, returns a pointer (or null on failure).
/// Prints a log line so you can see it working.
#[no_mangle]
pub extern "C" fn rust_malloc(size: usize) -> *mut u8 {
    if size == 0 {
        eprintln!("[rust_alloc] rust_malloc(0) -> NULL");
        return std::ptr::null_mut();
    }

    let layout = match Layout::from_size_align(size, 8) {
        Ok(l) => l,
        Err(_) => {
            eprintln!("[rust_alloc] rust_malloc({}) -> bad layout, NULL", size);
            return std::ptr::null_mut();
        }
    };

    let ptr = unsafe { alloc(layout) };
    if ptr.is_null() {
        eprintln!("[rust_alloc] rust_malloc({}) -> alloc failed, NULL", size);
        return ptr;
    }

    let addr = ptr as usize;
    {
        let mut guard = registry();
        let map = guard.get_or_insert_with(HashMap::new);
        map.insert(addr, layout);
    }

    eprintln!("[rust_alloc] rust_malloc({}) -> {:p}", size, ptr);
    ptr
}

/// Free a pointer previously returned by `rust_malloc`.
#[no_mangle]
pub extern "C" fn rust_free(ptr: *mut u8) {
    if ptr.is_null() {
        eprintln!("[rust_alloc] rust_free(NULL) -> no-op");
        return;
    }

    let addr = ptr as usize;
    let layout = {
        let mut guard = registry();
        guard.as_mut().and_then(|map| map.remove(&addr))
    };

    match layout {
        Some(layout) => {
            eprintln!("[rust_alloc] rust_free({:p}) -> {} bytes freed", ptr, layout.size());
            unsafe { dealloc(ptr, layout) };
        }
        None => {
            eprintln!("[rust_alloc] rust_free({:p}) -> UNKNOWN pointer, ignored!", ptr);
        }
    }
}

/// Returns how many live allocations are currently tracked.
#[no_mangle]
pub extern "C" fn rust_alloc_count() -> usize {
    let guard = registry();
    guard.as_ref().map_or(0, |map| map.len())
}

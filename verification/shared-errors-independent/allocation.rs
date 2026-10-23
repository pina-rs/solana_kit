use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

struct CountingAllocator;

static ACTIVE: AtomicBool = AtomicBool::new(false);
static ALLOCATIONS: AtomicUsize = AtomicUsize::new(0);

// The counter does not alter allocation ownership or layout.
unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        record();
        unsafe { System.alloc(layout) }
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        unsafe { System.dealloc(pointer, layout) }
    }

    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        record();
        unsafe { System.realloc(pointer, layout, size) }
    }
}

#[global_allocator]
static ALLOCATOR: CountingAllocator = CountingAllocator;

fn record() {
    if ACTIVE.load(Ordering::Relaxed) {
        ALLOCATIONS.fetch_add(1, Ordering::Relaxed);
    }
}

pub(super) fn count_allocations<T>(operation: impl FnOnce() -> T) -> (T, usize) {
    ALLOCATIONS.store(0, Ordering::Relaxed);
    ACTIVE.store(true, Ordering::Relaxed);
    let result = operation();
    ACTIVE.store(false, Ordering::Relaxed);
    (result, ALLOCATIONS.load(Ordering::Relaxed))
}

#[cfg(all(test, not(target_family = "wasm")))]
mod tests {
    use std::hint::black_box;
    use std::sync::{Arc, Barrier};

    use super::count_allocations;

    #[test]
    fn unrelated_thread_allocations_are_excluded() {
        let barrier = Arc::new(Barrier::new(2));
        let worker_barrier = barrier.clone();
        let worker = std::thread::spawn(move || {
            worker_barrier.wait();
            black_box(Box::new([0_u8; 1024]));
            worker_barrier.wait();
        });
        let (_, allocations) = count_allocations(|| {
            barrier.wait();
            barrier.wait();
        });
        worker.join().unwrap();
        assert_eq!(allocations, 0);
    }
}

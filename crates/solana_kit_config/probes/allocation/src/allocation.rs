use std::{
    alloc::{GlobalAlloc, Layout, System},
    sync::atomic::{AtomicBool, AtomicUsize, Ordering},
};

static ENABLED: AtomicBool = AtomicBool::new(false);
static TOTAL: AtomicUsize = AtomicUsize::new(0);
static MAXIMUM: AtomicUsize = AtomicUsize::new(0);

struct CountingAllocator;

#[global_allocator]
static ALLOCATOR: CountingAllocator = CountingAllocator;

fn record(size: usize) {
    if ENABLED.load(Ordering::Relaxed) {
        TOTAL.fetch_add(size, Ordering::Relaxed);
        MAXIMUM.fetch_max(size, Ordering::Relaxed);
    }
}

// Safety: Every operation forwards the original pointer, layout and size to System.
unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        record(layout.size());

        // Safety: The caller supplies the layout required by GlobalAlloc.
        unsafe { System.alloc(layout) }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        record(layout.size());

        // Safety: The caller supplies the layout required by GlobalAlloc.
        unsafe { System.alloc_zeroed(layout) }
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        // Safety: The original System allocation and layout come from the caller.
        unsafe { System.dealloc(pointer, layout) }
    }

    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        record(size);

        // Safety: The caller supplies the live allocation and valid replacement size.
        unsafe { System.realloc(pointer, layout, size) }
    }
}

pub(super) struct RequestedBytes {
    total: usize,
    maximum: usize,
}

impl RequestedBytes {
    pub(super) fn assert_bounded(self, operation: &str) {
        const CEILING: usize = 1024;
        println!(
            "{operation}: requested total={} maximum={}",
            self.total, self.maximum
        );
        assert!(
            self.total <= CEILING,
            "{operation} requests proportional temporary storage"
        );
        assert!(
            self.maximum <= CEILING,
            "{operation} requests a proportional allocation"
        );
    }
}

pub(super) fn measure<T>(operation: impl FnOnce() -> T) -> (T, RequestedBytes) {
    assert!(!ENABLED.load(Ordering::Relaxed));
    TOTAL.store(0, Ordering::Relaxed);
    MAXIMUM.store(0, Ordering::Relaxed);
    ENABLED.store(true, Ordering::Relaxed);
    let result = operation();
    ENABLED.store(false, Ordering::Relaxed);
    let requested = RequestedBytes {
        total: TOTAL.load(Ordering::Relaxed),
        maximum: MAXIMUM.load(Ordering::Relaxed),
    };

    (result, requested)
}

use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;

struct CountingAllocator;

thread_local! {
    // Constant initialization and no destructor keep allocator access allocation-free.
    static ALLOCATIONS: Cell<Option<usize>> = const { Cell::new(None) };
}

struct Measurement;

impl Drop for Measurement {
    fn drop(&mut self) {
        ALLOCATIONS.set(None);
    }
}

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
    // Allocations during thread teardown cannot access destroyed thread-local state.
    let _ = ALLOCATIONS.try_with(|count| {
        count.set(count.get().map(|value| value.saturating_add(1)));
    });
}

pub(super) fn count_allocations<T>(operation: impl FnOnce() -> T) -> (T, usize) {
    assert!(ALLOCATIONS.get().is_none(), "nested allocation measurement");
    let measurement = Measurement;
    ALLOCATIONS.set(Some(0));
    let result = operation();
    let allocations = ALLOCATIONS.get().unwrap();
    drop(measurement);
    (result, allocations)
}

#[cfg(all(test, not(target_family = "wasm")))]
mod tests {
    use std::hint::black_box;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicU8, Ordering};

    use super::count_allocations;

    fn wait_for(stage: &AtomicU8, expected: u8) {
        while stage.load(Ordering::SeqCst) < expected {
            std::hint::spin_loop();
        }
    }

    #[test]
    fn unrelated_thread_allocations_are_excluded() {
        let stage = Arc::new(AtomicU8::new(0));
        let worker_stage = stage.clone();
        let worker = std::thread::spawn(move || {
            wait_for(&worker_stage, 1);
            black_box(Box::new([0_u8; 1024]));
            worker_stage.store(2, Ordering::SeqCst);
        });
        let (_, allocations) = count_allocations(|| {
            stage.store(1, Ordering::SeqCst);
            wait_for(&stage, 2);
        });
        worker.join().unwrap();
        assert_eq!(allocations, 0);
    }

    #[test]
    fn simultaneous_measurements_count_only_their_own_allocations() {
        let stage = Arc::new(AtomicU8::new(0));
        let worker_stage = stage.clone();
        let worker = std::thread::spawn(move || {
            wait_for(&worker_stage, 1);
            let (_, allocations) = count_allocations(|| {
                worker_stage.store(2, Ordering::SeqCst);
                black_box(Box::new([0_u8; 1024]));
                worker_stage.store(3, Ordering::SeqCst);
                wait_for(&worker_stage, 4);
            });
            worker_stage.store(5, Ordering::SeqCst);
            allocations
        });
        let (_, allocations) = count_allocations(|| {
            stage.store(1, Ordering::SeqCst);
            wait_for(&stage, 3);
            black_box(Box::new([0_u8; 1024]));
            stage.store(4, Ordering::SeqCst);
            wait_for(&stage, 5);
        });
        let worker_allocations = worker.join().unwrap();
        assert_eq!(allocations, 1);
        assert_eq!(worker_allocations, 1);
    }

    #[test]
    fn unwinding_ends_the_measurement() {
        let result = std::panic::catch_unwind(|| {
            count_allocations(|| panic!("measurement panic"));
        });
        assert!(result.is_err());
        assert_eq!(count_allocations(|| ()).1, 0);
    }
}

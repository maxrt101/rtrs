use core::task::{RawWaker, RawWakerVTable, Waker};
use crate::task::TaskControlBlock;

pub(crate) fn create_waker(wake: *const ()) -> Waker {
    unsafe fn waker_clone(data: *const ()) -> RawWaker {
        RawWaker::new(data, &VTABLE)
    }

    unsafe fn waker_wake(data: *const ()) {
        // let c = unsafe { &*(data as *const AtomicBool) };
        // c.store(false, Ordering::Release);

        let c = unsafe { &*(data as *const TaskControlBlock) };

        c.ready()
    }

    unsafe fn waker_drop(_data: *const ()) {
        // nothing to do
    }

    static VTABLE: RawWakerVTable = RawWakerVTable::new(
        waker_clone,
        waker_wake,
        waker_wake,
        waker_drop,
    );

    unsafe { Waker::from_raw(RawWaker::new(wake, &VTABLE)) }
}

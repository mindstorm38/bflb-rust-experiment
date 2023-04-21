//! Asynchronous task executor for the runtime.

use core::task::{Poll, Context, Waker, RawWaker, RawWakerVTable};
use core::future::Future;
use core::pin::Pin;

use alloc::boxed::Box;
use alloc::vec::Vec;


/// Spawn an asynchronous task for the current thread's executor. The
/// thread executor will try to run every task cooperatively. *This
/// function will not block and will only be executed once [`wait`] is
/// called.* This function can be called while [`wait`] is being
/// executed inside running tasks, in such cases the spawned task will
/// be ran as early as possible.
pub fn spawn<F: Future<Output = ()> + 'static>(future: F) {
    // FIXME: In the future, this must be thread-local, so no locking
    // mechanism will be needed.
    unsafe {
        TASKS.push(Task {
            future: Box::pin(future),
        })
    }
}


/// Block the current thread waiting until every spawned task finish.
pub fn wait() {

    loop {

        // Save the initial count
        let tasks_count = unsafe { TASKS.len() };

        for i in 0..tasks_count {

            let task = unsafe { &mut TASKS[i] };
            let future = task.future.as_mut();

            // TODO: The data of the raw waker must be something like
            // an index or a pointer to an atomic variable of the task,
            // for signaling that the task must be resumed.
            let waker = unsafe {
                Waker::from_raw(RawWaker::new(&WAKER_NO_DATA, &WAKER_VTABLE))
            };

            let mut context = Context::from_waker(&waker);

            // When polling the future, other tasks may be spawned into
            // it, these tasks will be appended at the end of the TASKS
            // vector, so it's not problem and we can safely mutate TASKS.
            match future.poll(&mut context) {
                Poll::Ready(()) => {
                    // TODO: The task has finished, we must kill it.
                    todo!()
                }
                Poll::Pending => {
                    // TODO: The task is pending, we should not poll
                    // it until it's woken up.
                    todo!()
                }
            }

        }

    }

}


unsafe fn waker_clone(data: *const ()) -> RawWaker {
    todo!()
}

unsafe fn waker_wake(data: *const ()) {

}

unsafe fn waker_wake_by_ref(data: *const ()) {

}

unsafe fn waker_drop(data: *const ()) {

}


/// List of currently spawned tasks.
static mut TASKS: Vec<Task> = Vec::new();

/// Internal structure to represent a task.
struct Task {
    future: Pin<Box<dyn Future<Output = ()>>>,
}

unsafe impl Sync for Task {}


/// Internal vtable we use for the context's waker.
static WAKER_VTABLE: RawWakerVTable = RawWakerVTable::new(
    waker_clone,
    waker_wake,
    waker_wake_by_ref,
    waker_drop,
);

static WAKER_NO_DATA: () = ();







async fn test() {
    
    let mut a = async { 4 };
    let mut b = async { 10 };

    spawn(async {
        
    });



}

//! Asynchronous task executor for the runtime.
//! 
use core::sync::atomic::{AtomicBool, Ordering};
use core::task::{Poll, Context, Waker, RawWaker, RawWakerVTable};
use core::future::Future;
use core::pin::Pin;

use alloc::boxed::Box;
use alloc::vec::Vec;


/// Spawn an asynchronous task for the current thread's executor. The
/// thread executor will try to run every task cooperatively. *This
/// function will not block and the task will only be executed once 
/// [`wait`] is called.*
/// 
/// Note that this function can be called from task themselves.
pub fn spawn<F: Future<Output = ()> + 'static>(future: F) {

    let new_tasks = unsafe { &mut NEW_TASKS };
    new_tasks.push(Box::new(TaskImpl::new(future)));

    // TODO: In the future, we may return a handle with a return
    // data, maybe to select over multiple tasks?

}


/// Block the current thread waiting until every spawned task finish.
/// 
/// *This function will **panic** if called from running tasks.*
pub fn wait() {

    unsafe {
        if WAITING {
            panic!("already waiting");
        }
        WAITING = true;
    }

    // Internally store all awake tasks. This vector and the tasks it
    // contains are valid for the duration of the wait.
    let mut tasks = Vec::new();
    let mut ready_tasks = 0;

    loop {

        // SAFETY: The NEW_TASKS vector is manager either by 'spawn'
        // or 'wait' function, but because it's single-threaded, it
        // can't lead to data race, NEW_TASKS is then freed.
        tasks.extend(unsafe { NEW_TASKS.drain(..) });

        for task in &mut tasks {

            let (
                future,
                pending,
            ) = task.split();

            // If the task is pending, don't poll it.
            if pending.load(Ordering::Acquire) {
                break
            }

            // SAFETY: We know that the future is stored inside the
            // task, which is stored in a box and we guarantee that it
            // will not be moved until the end of the task.
            let future = unsafe {
                Pin::new_unchecked(future)
            };

            // SAFETY: This pointer to the pending boolean will be used
            // by waker. The pending boolean is stored inside a box,
            // which will not move until the end of the task.
            let pending_ptr = pending as *const _ as *const ();

            let waker = unsafe {
                Waker::from_raw(RawWaker::new(pending_ptr, &WAKER_VTABLE))
            };

            let mut context = Context::from_waker(&waker);

            // While polling, the vector NEW_TASKS may be modified,
            // but we don't mutate it so it's safe for 'spawn' to be
            // called from tasks.
            match future.poll(&mut context) {
                Poll::Ready(()) => {
                    
                    // TODO: The task has finished, we must remove it.
                    ready_tasks += 1;

                    // For now we set it pending to avoid polling it
                    // again, but it should not be woken up.
                    pending.store(true, Ordering::Release);

                }
                Poll::Pending => {
                    pending.store(true, Ordering::Release);
                }
            }

        }

        if ready_tasks == tasks.len() {
            break
        }

    }

    unsafe { WAITING = false };

}


/// This atomic variable is internally used to track if [`wait`] is
/// being called.
/// 
/// FIXME: In the future, these must be thread-local.
static mut WAITING: bool = false;

/// Indices of tasks that were recently spawned and not yet ran by
/// the [`wait`] function. This vector also contains indices of tasks
/// that has been wake up.
/// 
/// FIXME: In the future, these must be thread-local.
static mut NEW_TASKS: Vec<Box<dyn Task>> = Vec::new();


/// Internal trait used to dynamically store a task.
trait Task {

    /// Split the internal task into its two components, the future
    /// and the pending boolean, used for waker.
    /// 
    /// This function should be the only way to get access to those
    /// components.
    fn split<'a>(&'a mut self) -> (&'a mut dyn Future<Output = ()>, &'a AtomicBool);

}

/// Internal generic structure used only to implement [`Task`] trait
/// and therefore be dynamically called.
struct TaskImpl<F: Future> {
    /// The internal future of the task.
    future: F,
    /// The boolean used to indicate that the task is pending or not,
    /// we are using atomic here because the waker may be called from
    /// interrupts or other threads, we don't know.
    pending: AtomicBool,
}

impl<F: Future<Output = ()>> TaskImpl<F> {

    #[inline]
    fn new(future: F) -> Self {
        Self {
            future,
            pending: AtomicBool::new(false),
        }
    }

}

impl<F: Future<Output = ()>> Task for TaskImpl<F> {

    fn split<'a>(&'a mut self) -> (&'a mut dyn Future<Output = ()>, &'a AtomicBool) {
        (&mut self.future, &self.pending)
    }

}


unsafe fn waker_clone(data: *const ()) -> RawWaker {
    RawWaker::new(data, &WAKER_VTABLE)
}

unsafe fn waker_wake(data: *const ()) {
    // SAFETY: TODO:
    let pending = unsafe { &*(data as *const AtomicBool) };
    pending.store(false, Ordering::Release);
}

unsafe fn waker_wake_by_ref(data: *const ()) {
    // SAFETY: TODO:
    let pending = unsafe { &*(data as *const AtomicBool) };
    pending.store(false, Ordering::Release);
}

unsafe fn waker_drop(data: *const ()) {
    // Nothing to drop, because it's an atomic bool.
    let _ = data;
}

/// Internal vtable we use for the context's waker.
static WAKER_VTABLE: RawWakerVTable = RawWakerVTable::new(
    waker_clone,
    waker_wake,
    waker_wake_by_ref,
    waker_drop,
);

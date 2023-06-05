//! Threading base module.

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem::MaybeUninit;
use core::cell::UnsafeCell;
use core::ptr::{NonNull, addr_of, addr_of_mut};

use alloc::boxed::Box;
use alloc::vec;

use crate::hart::{HartLocal, spin_loop, acquire_interrupt, release_interrupt};
use crate::sym;


/// The thread counter is global to all harts.
static THREAD_COUNTER: AtomicUsize = AtomicUsize::new(0);

/// Hart-local context for threading.
static CONTEXT: HartLocal<HartContext> = HartLocal::new(HartContext {
    current: UnsafeCell::new(None),
});


/// Internal structure containing the state of a thread.
#[repr(C)]
pub struct Thread {
    /// Thread unique ID counter, init process has ID 0. This field
    /// is read-only after initialization and can be freely accessed.
    id: usize,
    /// Pointer to the parent process. This field is read-only after
    /// initialization and can be freely accessed.
    /// 
    /// TODO: Maybe remove it??
    parent: NonNull<Thread>,
    /// Pointer to the previous process.
    prev: NonNull<Thread>,
    /// Pointer to the next process to execute.
    /// 
    /// With the `prev` field, it can be accessed either from 
    /// interrupt context or from regular context. So interrupts
    /// must be disabled temporarily for the duration of the 
    /// modification.
    next: NonNull<Thread>,
    /// Stack allocation, can be used to retrieve length and origin
    /// of the stack. This memory is uninitialized.
    stack: NonNull<[StackSlot]>,
    /// Register context of the thread, this may be uninit because
    /// we don't want to initialize the registers on instantiation.
    context: Context,
}

/// Thread context used to save last registers that should be 
/// restored when the thread will be woken up.
#[repr(C)]
#[derive(Default)]
pub struct Context {
    pc: usize,
    sp: usize,
    s0: usize,
    s1: usize,
    s2: usize,
    s3: usize,
    s4: usize,
    s5: usize,
    s6: usize,
    s7: usize,
    s8: usize,
    s9: usize,
    s10: usize,
    s11: usize,
}

/// Context for the current hart context used for threading. This
/// structure is only accessed immutably, so it's using interior
/// mutability for fields.
struct HartContext {
    /// Pointer to the thread being executed, this pointer is specific
    /// to the current hart. This pointer should not be modified from
    /// interruption, therefore it's safe to mutate it inside non
    /// interrupt context, because it's single-hart.
    current: UnsafeCell<Option<NonNull<Thread>>>,
}

/// Used for aligning the stack allocation to 16 bytes, as specified
/// in the following issue thread:
/// https://github.com/riscv-non-isa/riscv-elf-psabi-doc/issues/21
#[repr(C, align(16))]
#[derive(Copy, Clone)]
struct StackSlot(MaybeUninit<[u8; 16]>);


/// Parameters for a spawning a thread.
pub struct ThreadConfig {
    /// Stack size for the thread to spawn, this size cannot be
    /// modified afterward to the default stack size is made to fit
    /// many scenarios, but it's generally good to customize the stack
    /// size to avoid over-allocation of memory. This will be aligned
    /// to 16 bytes to follow ABI conventions.
    /// 
    /// **If the last chunk that should be 16 bytes is smaller than 
    /// that, it will be thrown, therefore `stack_size` should be a
    /// multiple of 16 to avoid truncation.**
    stack_size: usize,
}

impl Default for ThreadConfig {
    fn default() -> Self {
        Self { 
            stack_size: 1024,
        }
    }
}


/// Idle entry for the current hart. This will wait until a process
/// is ready to run on the hart. This function never returns because
/// it starts the hart's scheduler, which will always return to this
/// particular function is no process is to run.
pub(crate) fn entry_idle() -> ! {

    // This code will run in a small kernel-specific stack.
    loop {

        // SAFETY: See field's documentation.
        let current = unsafe { &mut *CONTEXT.current.get() };

        if let Some(current) = current {
            
            // Switch to the task, but do not save the current context
            // because we know it and we can restore it to all zero.
            unsafe {
                let from_context = addr_of!((*current.as_ptr()).context);
                sym::_thread_switch(core::ptr::null_mut(), from_context);
            }

        }

        // If no process is to run, just spin loop hint.
        spin_loop();

    }

}

/// Almost the same as `entry_idle`, but instead it runs the first
/// thread, which will be required to run all subsequent threads. 
/// At least one hart need to run this function to start a first 
/// thread, but the runtime's entry actually start only one initial
/// process on hart 0.
pub(crate) fn entry_process(entry: fn(), config: ThreadConfig) -> ! {

    let thread = alloc_thread(entry, config);

    // SAFETY: See field's documentation.
    let current = unsafe { &mut *CONTEXT.current.get() };
    *current = Some(thread);

    // Switch to the task, but do not save the current context.
    unsafe {
        let from_context = addr_of!((*thread.as_ptr()).context);
        sym::_thread_switch(core::ptr::null_mut(), from_context);
    }

}


/// Spawn a child thread of the current thread
pub fn spawn(entry: fn(), config: ThreadConfig) {

    let thread = alloc_thread(entry, config);
    
    // SAFETY: See field's documentation.
    let current = unsafe { &mut *CONTEXT.current.get() };

    if let Some(current) = current {

        // Insert the new process in the chain, between the 
        // current process and the next one.
        unsafe {
            (*thread.as_ptr()).parent = *current;
            (*thread.as_ptr()).prev = *current;
            (*thread.as_ptr()).next = (*current.as_ptr()).next;
            (*current.as_ptr()).next = thread;
        }

    } else {

        // First process is its own parent and cycle to itself.
        unsafe { 
            (*thread.as_ptr()).parent = thread;
            (*thread.as_ptr()).prev = thread;
            (*thread.as_ptr()).next = thread;
        }

    }

}

/// Internal method to allocate a thread.
fn alloc_thread(entry: fn(), config: ThreadConfig) -> NonNull<Thread> {

    // Allocate the stack (alignment to 16 bytes).
    let stack_size = config.stack_size / core::mem::size_of::<StackSlot>();
    let stack_vec = vec![StackSlot(MaybeUninit::uninit()); stack_size];
    let stack_box = stack_vec.into_boxed_slice();
    let stack_start = stack_box.as_ptr();

    // SAFETY: Box::into_raw states that returned pointer != null.
    let stack = unsafe {
        NonNull::new_unchecked(Box::into_raw(stack_box))
    };

    let thread = Box::new(Thread {
        id: THREAD_COUNTER.fetch_add(1, Ordering::Relaxed),
        parent: NonNull::dangling(),
        prev: NonNull::dangling(),
        next: NonNull::dangling(),
        stack,
        context: Context {
            pc: entry as usize,
            sp: stack_start as usize,
            ..Context::default()
        },
    });

    // SAFETY: Box::into_raw states that returned pointer != null.
    unsafe {
        NonNull::new_unchecked(Box::into_raw(thread))
    }

}

/// Pause the current process and cooperatively schedule the next one.
pub fn pause() {

    // SAFETY: See field's documentation.
    let current = unsafe { &mut *CONTEXT.current.get() };
    let Some(current) = current else {
        // Invalid state, pause is not called from a thread??
        return
    };

    // Disable interrupts because it's required to access '.next'.
    let state = acquire_interrupt();

    // SAFETY: We can manipulate interrupts because we disabled interrupt.
    let next = unsafe { (*current.as_ptr()).next };
    
    // Get the two context to swap.
    let into_context = unsafe { addr_of_mut!((*current.as_ptr()).context) };
    let from_context = unsafe { addr_of!((*next.as_ptr()).context) };

    // Release interrupts because we are no longer using '.next'.
    release_interrupt(state);

    // Note: From here it's possible to be interrupted, just before
    // the switch, or in between but it's okay since we are no longer
    // accessing '.next' which might be accessed in handlers, and also
    // because no interrupt handler should access the context.

    unsafe {
        sym::_thread_switch(into_context, from_context);
    }

}

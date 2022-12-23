//! Definition of the global trap handler, it will be called for both 
//! interrupt (asynchronous) and exceptions (synchronous).


/// Type alias for interrupt or exceptions trap handlers.
pub type TrapHandler = fn(cause: usize, val: usize);

static mut EXCEPTION_HANDLERS: [TrapHandler; 16] = [default_exception_handler; 16];
static mut INTERRUPT_HANDLERS: [TrapHandler; 64] = [default_interrupt_handler; 64];


/// The default handler for CPU exceptions. This handler panics.
pub fn default_exception_handler(code: usize, val: usize) {

    let _ = val;

    let exc_message = match code {
        Exception::INSTRUCTION_ADDRESS_MISALIGNED => "instruction address misaligned",
        Exception::INSTRUCTION_ACCESS_FAULT => "instruction access fault",
        Exception::ILLEGAL_INSTRUCTION => "illegal instruction",
        Exception::BREAKPOINT => "breakpoint",
        Exception::LOAD_ADDRESS_MISALIGNED => "load address misaligned",
        Exception::LOAD_ACCESS_FAULT => "load access fault",
        Exception::STORE_AMO_ADDRESS_MISALIGNED => "store/amo address misaligned",
        Exception::STORE_AMO_ACCESS_FAULT => "store/amo access fault",
        Exception::ECALL_FROM_U_MODE => "environment call from user mode",
        Exception::ECALL_FROM_S_MODE => "environment call from supervisor mode",
        Exception::ECALL_FROM_M_MODE => "environment call from machine mode",
        Exception::INSTRUCTION_PAGE_FAULT => "instruction page fault",
        Exception::LOAD_PAGE_FAULT => "load page fault",
        Exception::STORE_AMO_PAGE_FAULT => "store/amo page fault",
        _ => panic!("Unhandled CPU exception with unknown code: {code}")
    };

    panic!("Unhandled CPU exception: {exc_message}");

}


/// The default handler for IRQ, do nothing.
pub fn default_interrupt_handler(code: usize, val: usize) {
    let _ = code;
    let _ = val;
}


/// Register an exception handler for the given exception code.
/// 
/// **This function is unsafe to call because no synchronization is guaranteed
/// when accessing the handlers table. Therefore you should call this on startup 
/// and ensure that it run on a single hart.**
pub unsafe fn register_exception_handler(exc: usize, handler: TrapHandler) {
    EXCEPTION_HANDLERS[exc] = handler;
}


/// Register an interrupt handler for the given IRQ number.
/// 
/// **This function is unsafe to call because no synchronization is guaranteed
/// when accessing the handlers table. Therefore you should call this on startup 
/// and ensure that it run on a single hart.**
pub unsafe fn register_interrupt_handler(num: usize, handler: TrapHandler) {
    INTERRUPT_HANDLERS[num] = handler;
}


/// The function called for handling a trap.
/// This function is called from [`crate::asm`].
#[inline]
pub fn trap_handler(cause: usize, val: usize) {
    
    const INTERRUPT_BIT: usize = 1 << (usize::BITS - 1);
    const CODE_MASK: usize = INTERRUPT_BIT - 1;

    let interrupt = cause & INTERRUPT_BIT != 0;
    let code = cause & CODE_MASK;

    if interrupt {
        unsafe { INTERRUPT_HANDLERS[code](code, val) }
    } else {
        unsafe { EXCEPTION_HANDLERS[code](code, val) }
    }
    
}


/// Standard exceptions constants.
pub struct Exception {}
impl Exception {
    const INSTRUCTION_ADDRESS_MISALIGNED: usize = 0;
    const INSTRUCTION_ACCESS_FAULT: usize       = 1;
    const ILLEGAL_INSTRUCTION: usize            = 2;
    const BREAKPOINT: usize                     = 3;
    const LOAD_ADDRESS_MISALIGNED: usize        = 4;
    const LOAD_ACCESS_FAULT: usize              = 5;
    const STORE_AMO_ADDRESS_MISALIGNED: usize   = 6;
    const STORE_AMO_ACCESS_FAULT: usize         = 7;
    const ECALL_FROM_U_MODE: usize              = 8;
    const ECALL_FROM_S_MODE: usize              = 9;
    const ECALL_FROM_M_MODE: usize              = 11;
    const INSTRUCTION_PAGE_FAULT: usize         = 12;
    const LOAD_PAGE_FAULT: usize                = 13;
    const STORE_AMO_PAGE_FAULT: usize           = 15;
}

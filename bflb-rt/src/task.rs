//! Asynchronous task executor for the runtime.

use core::future::Future;
use core::pin::Pin;
use core::task::{Poll, Context};





/// Wait for result of a future and
pub fn wait<F: Future>(mut future: F) {

}


pub fn select<F0: Future, F1: Future>() {

}


/// This structure is used to select results over two concurrent
/// future.
pub struct Selector<F0: Future, F1: Future> {
    f0: Option<F0>,
    f1: Option<F1>,
}

impl<F0: Future, F1: Future> Future for Selector<F0, F1> {

    type Output = Selected<F0, F1>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {

        match (self.f0.take(), self.f1.take()) {
            (None, None) => Poll::Ready(Selected::Complete),
            (None, Some(_)) => todo!(),
            (Some(_), None) => todo!(),
            (Some(_), Some(_)) => todo!(),
        }

    }

}


pub enum Selected<F0: Future, F1: Future> {
    /// Task 0 has been completed.
    F0(F0::Output),
    /// Task 1 has been completed.
    F1(F1::Output),
    /// All task has been completed.
    Complete,
}




#[macro_export]
macro_rules! select {
    (
        $($res:ident @ $future:expr => $exec:expr),+ $(,)?
    ) => {
        
    };
}



async fn test() {
    
    let mut a = async { 4 };
    let mut b = async { 10 };

    select! {
        a_res @ a => 
        b,
    }

}

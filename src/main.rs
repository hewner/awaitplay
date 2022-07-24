use futures::executor::block_on;
use std::pin::Pin;
use futures::Future;
use futures::lock::BiLock;
use futures::join;

use std::task::{Context, Poll, Waker};

struct TestShared {
    ready : bool,
    waker: Option<Waker>,
}

struct TestWaitFuture {
    shared : BiLock<TestShared>,
}

struct TestReadyFuture {
    shared : BiLock<TestShared>,
}

impl TestWaitFuture {
    fn new() -> (TestWaitFuture, TestReadyFuture) {
        let (lock1, lock2) = BiLock::new(TestShared { ready: false, waker : None } );
        (TestWaitFuture { shared : lock1 }, TestReadyFuture { shared : lock2 } )
    }
    
}

impl Future for TestWaitFuture {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("in wait poll");
        // Look at the shared state to see if the timer has already completed.
        let lock_result = self.shared.poll_lock(cx);
        match lock_result {
            Poll::Ready(mut test_shared) => {
                if test_shared.ready {
                    Poll::Ready(())
                } else {
                    test_shared.waker = Some(cx.waker().clone());
                    Poll::Pending
                }
                
            }
            Poll::Pending => Poll::Pending
        }
        
    }
}

impl Future for TestReadyFuture {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("in ready poll");
        let lock_result = self.shared.poll_lock(cx);
        match lock_result {
            Poll::Ready(mut test_shared) => {
                if let Some(waker) =  test_shared.waker.take() {
                    waker.wake()
                }

                test_shared.ready = true;
                Poll::Ready(())
            }
            Poll::Pending => Poll::Pending
        }
        
    }
}



fn main() {

    let (mywait, myready) = TestWaitFuture::new();
    let future = async {
        println!("future1a");
        join!(mywait, myready);
        println!("future1b");
        
    };

    block_on(future);
}

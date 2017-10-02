extern crate futures;
extern crate ipc_future;

use futures::{Async, Future};


#[test]
fn test_ready() {
    let (mut future, data) = ipc_future::IpcFuture::new().unwrap();
    assert_eq!(Async::NotReady, future.poll().unwrap());
    data.provide(1u8).unwrap();
    assert_eq!(Async::Ready(1), future.poll().unwrap());
}


#[test]
#[should_panic]
fn test_done() {
    let (mut future, data) = ipc_future::IpcFuture::new().unwrap();
    data.provide(2u8).unwrap();
    future.poll().unwrap(); // ready
    future.poll().unwrap(); // can't do this again
}

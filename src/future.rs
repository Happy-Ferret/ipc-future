use std::{io, mem};

use bincode::ErrorKind;
use futures::{Async, Future, Poll};
use ipc_channel::ipc::{channel, IpcReceiver, IpcSender};
use serde::{Deserialize, Serialize};

use IpcError;


/// An opaque wrapper around IPC sender that only allows
/// providing the data once for the future to consume.
pub struct IpcFutureData<T> where
    T: Serialize
{
    inner: IpcSender<T>,
}

impl<T> IpcFutureData<T> where
    T: Serialize
{
    /// Provide the data and close the channel.
    pub fn provide(self, value: T) -> Result<(), IpcError> {
        self.inner.send(value)
    }
}


/// A future object representing some data arriving via an IPC channel.
pub enum IpcFuture<T> where
    for<'de> T: Serialize + Deserialize<'de>
{
    /// Data has not yet arrived.
    Waiting(IpcReceiver<T>),
    /// Data is here, to be consumed by a `poll`
    Ready(T),
    /// Data has been consumed.
    Empty,
}

impl<T> IpcFuture<T> where
    for<'de> T: Serialize + Deserialize<'de>
{
    /// Create a new future object accompanied by the future data to provide it.
    pub fn new() -> Result<(Self, IpcFutureData<T>), io::Error> {
        channel()
            .map(|(sender, receiver)| {
                (IpcFuture::Waiting(receiver), IpcFutureData { inner: sender })
            })
    }

    /// Safer version of the `Future::poll`. The data is stored internally
    /// upon receiving and not getting returned.
    pub fn poll_impl(&mut self) -> Poll<(), IpcError> {
        let mut result = None;
        if let IpcFuture::Waiting(ref mut receiver) = *self {
            match receiver.try_recv() {
                Ok(value) => result = Some(value),
                Err(error) => return match *error {
                    ErrorKind::IoError(ref io_err)
                        if io_err.kind() == io::ErrorKind::WouldBlock =>
                        Ok(Async::NotReady),
                    _ => Err(error),
                },
            }
        }
        if let Some(value) = result {
            *self = IpcFuture::Ready(value);
        }
        Ok(Async::Ready(()))
    }

    /// Get a reference to the stored data, if it is ready.
    pub fn as_ref(&self) -> Option<&T> {
        match *self {
            IpcFuture::Waiting(_) | IpcFuture::Empty => None,
            IpcFuture::Ready(ref value) => Some(value),
        }
    }
}

impl<T> Future for IpcFuture<T> where
    for<'de> T: Serialize + Deserialize<'de>
{
    type Item = T;
    type Error = IpcError;

    fn poll(&mut self) -> Poll<T, IpcError> {
        self.poll_impl()?;
        let real_me = mem::replace(self, IpcFuture::Empty);
        Ok(match real_me {
            IpcFuture::Waiting(_) => {
                *self = real_me;
                Async::NotReady
            },
            IpcFuture::Ready(value) => Async::Ready(value),
            IpcFuture::Empty => panic!("Data has already been consumed"),
        })
    }
}

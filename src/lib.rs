//! Future adaptor for IPC channels.
//!
#![warn(missing_docs)]

extern crate bincode;
extern crate futures;
extern crate ipc_channel;
extern crate serde;

pub use ipc_channel::Error as IpcError;
pub use self::future::{IpcFuture, IpcFutureData};

mod future;

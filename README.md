# ipc-future
[![Build Status](https://travis-ci.org/kvark/ipc-future.svg)](https://travis-ci.org/kvark/ipc-future)
[![](http://meritbadge.herokuapp.com/ipc-future)](https://crates.io/crates/ipc-future)
[![Documentation](https://docs.rs/ipc-future/badge.svg)](https://docs.rs/ipc-future)

Future implementation for single-shot IPC channels of [ipc-channel](https://github.com/servo/ipc-channel).

Note: `ipc-channel` already has `async` feature that hooks up to `future-rs`, but it implements `Stream` over `IpcReceiver`, which has a little bit different semantics. Using such a receiver in practice could be less fun than the `IpcFuture` from this crate, given that it keeps the value and allows the user to borrow it.

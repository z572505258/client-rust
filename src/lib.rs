// Copyright 2018 TiKV Project Authors. Licensed under Apache-2.0.

// Long and nested future chains can quickly result in large generic types.
#![type_length_limit = "16777216"]
#![allow(clippy::redundant_closure)]
#![feature(async_await)]

//! This crate provides a clean, ready to use client for [TiKV](https://github.com/tikv/tikv), a
//! distributed transactional Key-Value database written in Rust.
//!
//! With this crate you can easily connect to any TiKV deployment, interact with it, and mutate the
//! data it contains.
//!
//! ## Choosing an API
//!
//! This crate offers both [**raw**](raw/index.html) and
//! [**transactional**](transaction/index.html) APIs. You should choose just one for your system.
//!
//! The *consequence* of supporting transactions is increased overhead of coordination with the
//! placement driver for timestamp acquisition. This is approximately 1 RTT.
//!
//! *While it is possible to use both APIs at the same time, doing so is unsafe and unsupported.*
//!
//! Choose the one that suites your needs as described below, then add the import statement to your
//! file where you need to use the library.
//!
//! ### Transactional
//!
//! The [transactional](transaction/index.html) API supports **transactions** via Multi-Version
//! Concurrency Control (MVCC).
//!
//! **Best when you mostly do** complex sets of actions, actions which may require a rollback,
//! operations affecting multiple keys or values, or operations that depend on strong ordering.
//!
//! ```rust
//! use tikv_client::{*, transaction::*};
//! ```
//!
//! ### Raw
//!
//! The [raw](raw/index.html) API has **reduced coordination overhead**, but lacks any
//! transactional abilities.
//!
//! **Best when you mostly do** single row changes, and have very limited cross-row (eg. foreign
//! key) requirements. You will not be able to use transactions with this API.
//!
//! ```rust
//! use tikv_client::{*, raw::*};
//! ```
//!
//! ## Connect
//!
//! Regardless of which API you choose, you'll need to connect your client
//! ([raw](raw::Client), [transactional](transaction::Client)).
//!
//! ```rust
//! # #![feature(async_await)]
//! # use tikv_client::{*, raw::*};
//! # use futures::prelude::*;
//!
//! # futures::executor::block_on(async {
//! // Configure endpoints and optional TLS.
//! let config = Config::new(vec![ // A list of PD endpoints.
//!     "192.168.0.100:2379",
//!     "192.168.0.101:2379",
//! ]).with_security("root.ca", "internal.cert", "internal.key");
//!
//! // Get an unresolved connection.
//! let connect = Client::connect(config);
//!
//! // Resolve the connection into a client.
//! let client = connect.into_future().await;
//! # });
//! ```
//!
//! At this point, you should seek the documentation in the related API modules.

mod compat;
mod config;
mod errors;
mod kv;
#[cfg(test)]
mod proptests;
pub mod raw;
mod rpc;
pub mod transaction;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use]
extern crate prometheus;

#[doc(inline)]
pub use crate::config::Config;
#[doc(inline)]
pub use crate::errors::Error;
#[doc(inline)]
pub use crate::errors::ErrorKind;
#[doc(inline)]
pub use crate::errors::Result;
#[doc(inline)]
pub use crate::kv::{BoundRange, Key, KvPair, ToOwnedRange, Value};

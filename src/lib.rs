//! The name resolution configured via environment variable.
//!
//! [API Docs](https://docs.rs/ns-env-config) |
//! [Usage](https://github.com/tailhook/ns-env-config#usage) |
//! [Github](https://github.com/tailhook/ns-env-config) |
//! [Crate](https://crates.io/crates/ns-env-config)
//!
//! This crate also re-exports important things from abstract-ns and router so
//! that you don't have to track too many dependencies.
//!
//! # Example
//!
//! ```
//! extern crate ns_env_config;
//! extern crate tokio_core;
//! use tokio_core::reactor::Core;
//!
//! fn main() {
//!     let mut core = Core::new().expect("reactor init");
//!     let ns = ns_env_config::init(&core.handle()).expect("name system init");
//!     let addr = core.run(ns.resolve_auto("localhost", 80)).unwrap();
//!     println!("Localhost HTTP is {:?}", addr);
//! }
//! ```
//!
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]

pub extern crate abstract_ns as ns;
pub extern crate ns_router as router;

extern crate failure;
extern crate futures_cpupool;
extern crate ns_std_threaded;
extern crate tokio_core;
extern crate void;

mod initialize;
pub mod config;

pub use ns::{Address, IpList, Error};
pub use router::{Router};
pub use initialize::{init, init_default, force_config};

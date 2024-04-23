//! This crate defines the Remote Procedure Call API that happens between
//! the docker and the rust application, once for the ftdi driver simulation
//! and the other for communicating with the windows rust application.

pub mod rpc;
pub mod win_daemon;

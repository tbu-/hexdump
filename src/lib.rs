//! Easy hexdump to stdout or as an iterator.
//!
//! # Example
//!
//! ```
//! extern crate hexdump;
//! hexdump::hexdump(b"12345\0\r\n\t .abcdef");
//! ```
//! prints
//!
//! ```text
//! |31323334 35000d0a 09202e61 62636465| 12345.... .abcde 00000000
//! |66|                                  f                00000010
//!                                                        00000011
//! ```

#![warn(missing_docs)]

#![cfg_attr(all(test, feature="nightly-test"), feature(plugin))]
#![cfg_attr(all(test, feature="nightly-test"), plugin(quickcheck_macros))]
#[cfg(all(test, feature="nightly-test"))] extern crate quickcheck;
#[cfg(test)] extern crate num;

extern crate arrayvec;
extern crate itertools;

mod imp;

pub use imp::Line;
pub use imp::Hexdump;
pub use imp::hexdump;
pub use imp::hexdump_iter;
pub use imp::sanitize_byte;

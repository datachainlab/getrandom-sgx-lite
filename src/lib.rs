// Copyright 2018 Developers of the Rand project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Implementation for SGX using RDRAND instruction
#![no_std]
pub use error::Error;

use sgx_trts::trts::rsgx_read_rand;

mod error;

#[allow(deprecated)]
pub fn getrandom_inner(dest: &mut [u8]) -> Result<(), Error> {
    // sgx_read_rand cannot take len=0, but this function does
    if dest.is_empty() {
        return Ok(());
    }

    match rsgx_read_rand(dest) {
        Ok(()) => Ok(()),
        Err(_) => Err(Error::UNAVAILABLE),
    }
}

//#[inline(always)]
//pub fn error_msg_inner(_: NonZeroU32) -> Option<&'static str> { None }

/// Fill `dest` with random bytes from the system's preferred random number
/// source.
///
/// This function returns an error on any failure, including partial reads. We
/// make no guarantees regarding the contents of `dest` on error. If `dest` is
/// empty, `getrandom` immediately returns success, making no calls to the
/// underlying operating system.
///
/// Blocking is possible, at least during early boot; see module documentation.
///
/// In general, `getrandom` will be fast enough for interactive usage, though
/// significantly slower than a user-space CSPRNG; for the latter consider
/// [`rand::thread_rng`](https://docs.rs/rand/*/rand/fn.thread_rng.html).
pub fn getrandom(dest: &mut [u8]) -> Result<(), error::Error> {
    if dest.is_empty() {
        return Ok(());
    }
    getrandom_inner(dest)
}

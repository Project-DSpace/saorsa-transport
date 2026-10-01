// Copyright 2024 Saorsa Labs Ltd.
//
// This software is licensed under the MIT license <LICENSE-MIT or
// https://opensource.org/licenses/MIT> or the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0>, at your
// option. This file may not be copied, modified, or distributed except
// according to those terms.
//
// Unless required by applicable law or agreed to in writing, software
// distributed under these licenses is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.

//! Property-based tests for saorsa-transport
//!
//! This module contains property-based tests that verify invariants
//! and properties of the QUIC protocol implementation.

#![allow(clippy::unwrap_used, clippy::expect_used)]

#[cfg(test)]
pub mod frame_properties;

#[cfg(test)]
pub mod nat_properties;

#[cfg(test)]
pub mod transport_properties;

#[cfg(test)]
pub mod connection_properties;

#[cfg(test)]
pub mod crypto_properties;

#[cfg(test)]
pub mod generators;

/// Common property test configuration
pub mod config {
    use proptest::prelude::*;

    /// Default number of test cases for property tests
    pub const DEFAULT_PROPTEST_CASES: u32 = 256;

    /// Extended number of test cases for thorough testing
    pub const EXTENDED_PROPTEST_CASES: u32 = 1024;

    /// Maximum shrinking iterations
    pub const MAX_SHRINK_ITERS: u32 = 10000;

    /// Get default proptest config
    pub fn default_config() -> ProptestConfig {
        ProptestConfig {
            cases: DEFAULT_PROPTEST_CASES,
            max_shrink_iters: MAX_SHRINK_ITERS,
            ..Default::default()
        }
    }

    /// Get extended proptest config for CI
    pub fn extended_config() -> ProptestConfig {
        ProptestConfig {
            cases: EXTENDED_PROPTEST_CASES,
            max_shrink_iters: MAX_SHRINK_ITERS,
            ..Default::default()
        }
    }
}

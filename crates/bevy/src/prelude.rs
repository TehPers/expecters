//! This module contains commonly used exports from this crate.
//!
//! To keep your imports simple, rather than importing these members
//! individually, you can write:
//!
//! ```
//! # #[allow(unused_imports)]
//! use expecters_bevy::prelude::*;
//! ```
//!
//! While not necessary, it is recommended to glob import this module in any
//! test modules that use this crate.

pub use crate::change_detection::ChangeDetectionAssertions;

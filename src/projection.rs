//! Utility functions to create projection matrices.
//!
//! Each module contains versions for a certain kind of "base" coordinate systems, i.e. the coordinate
//! system that your client application is using. All of them assume +x is pointing to the right.
//!
//! The `rh_yup` module is publicly re-exported as it contains the functions for what is generally seen
//! as the 'standard' computer graphics coordinate space.

pub mod lh_ydown;
pub mod lh_yup;
pub mod rh_ydown;
pub mod rh_yup;

pub use rh_yup::*;

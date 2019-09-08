//! Type classes and data types for Rust
//!
//! This crate provides type classes and data types for Rust Language
//!
//! # Organization
//! This module is organized by next way:
//!
//! * type_classes are the traits ob standart type classes (Monoid, Functors, etc).
//! * data_types are some kinds of data types.
//! * instances are example instance for every type class.
//! * laws are function for asserting that custom type are fulfill laws of concrete type class



pub mod type_classes;
pub mod instances;
pub mod laws;

pub use type_classes::Monoid;
pub use type_classes::Semigroup;

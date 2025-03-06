//  LIB.rs
//    by Lut99
//
//  Created:
//    05 Mar 2025, 17:29:54
//  Last edited:
//    06 Mar 2025, 10:36:24
//  Auto updated?
//    Yes
//
//  Description:
//!   Collection of libraries for working with compilers in Rust.
//

// Define the libraries
#[cfg(feature = "parser")]
pub mod parser;
#[cfg(feature = "punctuated")]
pub mod punctuated;
#[cfg(feature = "railroad")]
pub mod railroad;
#[cfg(feature = "report")]
pub mod report;
#[cfg(feature = "span")]
pub mod span;
#[cfg(feature = "terms")]
pub mod terms;
#[cfg(feature = "visit")]
pub mod visit;

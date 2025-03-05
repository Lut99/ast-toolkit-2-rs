//  MOD.rs
//    by Lut99
//
//  Created:
//    05 Mar 2025, 17:50:20
//  Last edited:
//    05 Mar 2025, 17:51:20
//  Auto updated?
//    Yes
//
//  Description:
//!   Library for generating very human-readable compilation errors.
//!   
//!   This library mostly wraps around [ariadne](::ariadne), except that it aims to be more
//!   general. Specifically, it provides the [`Report`](trait@Report)-trait that can be
//!   [automatically derived](derive@Report) on errors to convenient generate them from normal
//!   Rust errors.
//

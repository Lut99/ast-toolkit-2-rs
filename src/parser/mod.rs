//  MOD.rs
//    by Lut99
//
//  Created:
//    05 Mar 2025, 17:46:02
//  Last edited:
//    05 Mar 2025, 17:49:35
//  Auto updated?
//    Yes
//
//  Description:
//!   Provides traits for AST nodes to parser them from some input source.
//!
//!   This library wraps around [chumsky](::chumsky) to implement the parsing, and you are likely
//!   to interface with it. The only thing this library adds is the [`trait@Parse`]-trait and its
//!   associated [derive macro](derive@Parse).
//

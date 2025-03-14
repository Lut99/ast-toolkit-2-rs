//  LIB.rs
//    by Lut99
//
//  Created:
//    06 Mar 2025, 10:53:08
//  Last edited:
//    06 Mar 2025, 11:01:09
//  Auto updated?
//    Yes
//
//  Description:
//!   Macros for the `ast-toolkit-2`-library.
//

// Define the libaries
#[cfg(feature = "railroad")]
mod railroad;

// Imports
#[allow(unused_imports)]
use proc_macro::TokenStream;


/***** MACROS *****/
/// Derive macro to automatically implement [`Railroad`] on an AST node.
///
/// By default, it will interpret structs as sequences of nodes to parse. You can use attributes to
/// exert some degree of control over every node.
///
/// Enums are interpeted as choices of syntax, with enum variants again interpreted as sequences.
///
/// # Attributes
/// ## Toplevel
/// At the toplevel, you can give the following attributes:
/// - `#[railroad(prefix = ...)]`: Defines the library path to the `railroad`-library. If omitted,
///   defaults to `::ast_toolkit_2::railroad`.
///
/// ## Variant-level
///
/// ## Field-level
#[inline]
#[proc_macro_derive(Railroad, attributes(railroad))]
pub fn railroad(input: TokenStream) -> TokenStream {
    match railroad::railroad(input.into()) {
        Ok(res) => res.into(),
        Err(err) => err.into_compile_error().into(),
    }
}

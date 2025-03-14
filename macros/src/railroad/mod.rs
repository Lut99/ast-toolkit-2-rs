//  MOD.rs
//    by Lut99
//
//  Created:
//    06 Mar 2025, 10:53:31
//  Last edited:
//    14 Mar 2025, 15:36:30
//  Auto updated?
//    Yes
//
//  Description:
//!   Implements macros for the `railroad`-library.
//

use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned as _;
use syn::{Attribute, Data, DeriveInput, Error, Expr, Fields, Ident, LitStr, Meta, Path, PathArguments, PathSegment, Token, Variant};


/***** ATTRIBUTES *****/
/// Defines what we may learn from toplevel attributes.
struct ToplevelAttrs {
    /// The prefix path to the library structs.
    prefix: Path,
}
impl Default for ToplevelAttrs {
    #[inline]
    fn default() -> Self {
        Self {
            prefix: Path {
                leading_colon: Some(Default::default()),
                segments:      {
                    let mut segments = Punctuated::new();
                    segments.push(PathSegment { ident: Ident::new("ast_toolkit_2", Span::call_site()), arguments: PathArguments::None });
                    segments.push(PathSegment { ident: Ident::new("railroad", Span::call_site()), arguments: PathArguments::None });
                    segments
                },
            },
        }
    }
}
impl TryFrom<Vec<Attribute>> for ToplevelAttrs {
    type Error = Error;

    #[inline]
    fn try_from(value: Vec<Attribute>) -> Result<Self, Self::Error> {
        let mut res: Self = Default::default();
        for attr in value {
            match attr.meta {
                Meta::List(l) if l.path.is_ident("railroad") => {
                    // Parse the contents as a list of metas
                    let attrs: Punctuated<ToplevelAttr, Token![,]> = l.parse_args_with(Punctuated::parse_terminated)?;
                    for attr in attrs {
                        match attr {
                            ToplevelAttr::Prefix(path) => res.prefix = path,
                        }
                    }
                },
                _ => continue,
            }
        }
        Ok(res)
    }
}

/// Defines possible toplevel attributes.
enum ToplevelAttr {
    /// Defines the prefix.
    Prefix(Path),
}
impl Parse for ToplevelAttr {
    #[inline]
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Parse the identifier first
        let ident: Path = input.parse()?;
        if ident.is_ident("prefix") {
            // Parse it as a name/value pair of a path
            input.parse::<Token![=]>()?;
            Ok(Self::Prefix(input.parse()?))
        } else {
            Err(Error::new(ident.span(), format!("Unknown attribute {:?}", ident.to_token_stream().to_string())))
        }
    }
}



/// Defines what we may learn from field attributes.
struct FieldAttrs {
    /// The kind of the field.
    kind:     FieldKind,
    /// A manual comment for this field.
    comment:  Option<Expr>,
    /// Manual override to mark a field as optional.
    optional: bool,
    /// Manual override to mark a field as repeated.
    repeated: bool,
}
impl Default for FieldAttrs {
    #[inline]
    fn default() -> Self { Self { kind: FieldKind::Terminal, comment: None, optional: false, repeated: false } }
}

/// Defines the possible kinds of fields.
enum FieldKind {
    /// We will generate the field in full.
    Terminal,
    /// We will generate the field's label only.
    NonTerminal,
    /// We will use the `RailroadDelim`-implementation instead.
    ///
    /// Encodes an expression to find the contents in between the delimiter.
    Delim(Expr),
}

/// Defines possible field-level attributes.
enum FieldAttr {}





/***** HELPER FUNCTIONS *****/
/// Generates a railroad expression for the given struct.
///
/// # Arguments
/// - `fields`: The [`Fields`] to derive the expression from.
///
/// # Returns
fn generate_struct_railroad_expr(fields: Fields) -> Result<TokenStream2, Error> {
    // Go through the fields to generate the individual expressions
    let mut field_exprs: Vec<TokenStream2> = Vec::with_capacity(fields.len());
    for field in fields {
        // Read the attributes
    }
    return Ok(TokenStream2::new());
}

/// Generates a railroad expression for the given enum.
///
/// # Arguments
/// - `variants`: The list of [`Variant`]s to derive the expression from.
///
/// # Returns
fn generate_enum_railroad_expr(variants: Punctuated<Variant, Token![,]>) -> Result<TokenStream2, Error> { todo!() }





/***** LIBRARY *****/
/// Implements the `Railroad` derive-macro.
///
/// See the [actual macro](super::railroad()) for more information.
///
/// # Arguments
/// - `input`: A [`TokenStream2`] encoding the input to parse and derive from.
///
/// # Returns
/// A stream that encodes the generated impl.
///
/// # Errors
/// This function may error if something about the input was malformed (probably attributes).
pub fn railroad(input: TokenStream2) -> Result<TokenStream2, Error> {
    // Parse as derive macro input
    let DeriveInput { attrs, vis, ident, generics, data } = syn::parse2(input)?;
    let tattrs: ToplevelAttrs = attrs.try_into()?;

    // Switch on the target to generate the expression
    let expr: TokenStream2 = match data {
        Data::Struct(s) => generate_struct_railroad_expr(s.fields)?,
        Data::Enum(e) => generate_enum_railroad_expr(e.variants)?,
        Data::Union(u) => return Err(Error::new(u.union_token.span(), "Cannot derive Railroad on unions")),
    };
    todo!()
}

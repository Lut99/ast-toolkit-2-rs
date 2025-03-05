//  MOD.rs
//    by Lut99
//
//  Created:
//    05 Mar 2025, 17:52:30
//  Last edited:
//    05 Mar 2025, 17:53:48
//  Auto updated?
//    Yes
//
//  Description:
//!   Library for implementing the visitor-pattern for your AST.
//!   
//!   This library specifically contributes the [`Visitable`](trait@Visitable),
//!   [`VisitableMut`](trait@VisitableMut) and [`VisitableOwned`](trait@VisitableOwned)-traits to
//!   mark nodes in your AST as visitable. Then, equivalent derive macros are provided to do so
//!   ergonomically.
//

//  MOD.rs
//    by Lut99
//
//  Created:
//    05 Mar 2025, 17:31:22
//  Last edited:
//    06 Mar 2025, 13:03:05
//  Auto updated?
//    Yes
//
//  Description:
//!   Provides traits for AST nodes to serialize them to nodes for the
//!   excellent [railroad](::railroad)-crate.
//

// Module definitions
pub mod diagram;
mod impls;

// Re-exports
pub use diagram::Diagram;
pub use railroad;


/***** LIBRARY *****/
/// Marks your node as convertible into a [`Diagram`] or a [`railroad` Node](railroad::Node).
pub trait Railroad {
    /// The type of [`railroad`-node](railroad::Node) to which this node is serialized when
    /// calling [`Railroad::to_railroad_node()`].
    type Output: 'static + railroad::Node;
    /// The type of [`railroad`-node](railroad::Node) to which this node is serialized when
    /// calling [`Railroad::to_railroad_node_inline()`].
    type InlineOutput: 'static + railroad::Node;


    /// Serializes this node into a [`railroad`](railroad) track.
    ///
    /// This can then be added to a [`Diagram`] to create a visual SVG.
    ///
    /// Unlike [`Railroad::to_railroad_node_inline()`], this function should always generate the
    /// full syntax for this node.
    ///
    /// # Returns
    /// A [`NonTerminal`](railroad::NonTerminal) that can be rendered as a railroad track.
    #[inline]
    fn to_railroad_node() -> Self::Output;

    /// Serializes this node into a [`railroad`](railroad) track.
    ///
    /// This can then be added to a [`Diagram`] to create a visual SVG.
    ///
    /// This particular function should generate an "inline"-version of the node. For example, if
    /// the node encodes a large part of the syntax, then consider generating a simple label here
    /// and then later [adding it as a separate track](Diagram::add_nonterm()) in the final
    /// diagram.
    ///
    /// By default, it simply refers to the [`Railroad::to_railroad_node()`]-implementation.
    ///
    /// # Returns
    /// A [`Node`](railroad::Node) that can be rendered within a railroad track.
    #[inline]
    fn to_railroad_node_inline() -> Self::InlineOutput { Self::to_railroad_node() }

    /// Convenience function for generation a diagram that only shows this node.
    ///
    /// Usually, you don't need to override this. Only your toplevel node should to make sure that
    /// it plus any nonterminals in your tree are rendered correctly.
    ///
    /// By default, it will create a new [`Diagram`]-object with only the
    /// [serialization](Railroad::to_railroad_node()) of this node added to it.
    #[inline]
    fn to_railroad_diagram() -> Diagram<'static> { Diagram::new(Self::to_railroad_node()) }
}



/// Marks that a delimiting node can be converted into a [`Diagram`] or a
/// [`railroad` Node](railroad::Node).
pub trait RailroadDelim: Railroad {
    /// The type of [`railroad`-node](railroad::Node)-node to which this node is serialized.
    type DelimOutput: 'static + railroad::Node;


    /// Serializes this node into a [`railroad` Node](railroad::Node).
    ///
    /// # Arguments
    /// - `node`: Some other [`railroad` Node](railroad::Node) to wrap this delimited node around.
    ///
    /// # Returns
    /// A [`Node`](railroad::Node) that can be rendered as a railroad diagram.
    fn to_railroad_delim_node<'n>(node: impl 'static + railroad::Node) -> Self::DelimOutput;
}

//  MOD.rs
//    by Lut99
//
//  Created:
//    05 Mar 2025, 17:31:22
//  Last edited:
//    05 Mar 2025, 18:31:36
//  Auto updated?
//    Yes
//
//  Description:
//!   Provides traits for AST nodes to serialize them to nodes for the
//!   excellent [railroad](::railroad)-crate.
//

use std::fmt::Display;

// Re-export the lib
pub use railroad;


/***** LIBRARY STRUCTS *****/
/// Allows one to take a serialized [`railroad` Node](railroad::Node) and write it to a file.
///
/// Usually, you should override [`Railroad::to_railroad_diagram()`] on your toplevel node to
/// produce one properly with all the nonterminals in your tree.
pub struct Diagram<'n> {
    /// The toplevel node to make the main track with.
    toplevel: Option<Box<dyn 'n + railroad::Node>>,
    /// A list of additional nonterminals to render.
    nonterms: Vec<railroad::LabeledBox<Box<dyn 'n + railroad::Node>, railroad::Comment>>,
}
// Constructors
impl<'n> Default for Diagram<'n> {
    #[inline]
    fn default() -> Self { Self::empty() }
}
impl<'n> Diagram<'n> {
    /// Constructor for the Diagram that initializes it without the toplevel track.
    ///
    /// Use [`Diagram::set_track()`] to set it.
    ///
    /// # Returns
    /// A new Diagram that does not have any tracks in it yet.
    #[inline]
    pub fn empty() -> Self { Self { toplevel: None, nonterms: Vec::new() } }

    /// Constructor for the Diagram that initializes it without the toplevel track, but with
    /// capacity for at least a given number of nonterminals.
    ///
    /// Use [`Diagram::set_track()`] to set the toplevel track.
    ///
    /// # Arguments
    /// - `capacity`: A minimum number of nonterminals to reserve space for. Precisely, it is
    ///   guaranteed that the resulting Diagram can store at least `capacity` nonterminals before
    ///   having to re-allocate its internal buffer.
    ///
    /// # Returns
    /// A new Diagram that does not have any tracks in it yet, but enough `capacity` for at least
    /// that number of nonterminals.
    #[inline]
    pub fn empty_with_capacity(capacity: usize) -> Self { Self { toplevel: None, nonterms: Vec::with_capacity(capacity) } }

    /// Constructor for the Diagram that initializes it with a toplevel track.
    ///
    /// Note that you would probably still like to add nonterminals to render those as separate
    /// tracks. To do so, call [`Diagram::add_nonterm()`].
    ///
    /// # Arguments
    /// - `track`: A [`railroad` Node](railroad::Node) that represents the toplevel track to
    ///   render.
    ///
    /// # Returns
    /// A new Diagram that will render the given toplevel `track`.
    #[inline]
    pub fn new(track: impl 'n + railroad::Node) -> Self { Self { toplevel: Some(Box::new(track)), nonterms: Vec::new() } }

    /// Constructor for the Diagram that initializes it with a toplevel track.
    ///
    /// Note that you would probably still like to add nonterminals to render those as separate
    /// tracks. To do so, call [`Diagram::add_nonterm()`].
    ///
    /// # Arguments
    /// - `track`: A [`railroad` Node](railroad::Node) that represents the toplevel track to
    ///   render.
    /// - `capacity`: A minimum number of nonterminals to reserve space for. Precisely, it is
    ///   guaranteed that the resulting Diagram can store at least `capacity` nonterminals before
    ///   having to re-allocate its internal buffer.
    ///
    /// # Returns
    /// A new Diagram that will render the given toplevel `track`, and enough `capacity` for at
    /// least the given number of nonterminals.
    #[inline]
    pub fn with_capacity(track: impl 'n + railroad::Node, capacity: usize) -> Self {
        Self { toplevel: Some(Box::new(track)), nonterms: Vec::with_capacity(capacity) }
    }

    /// Constructor for the Diagram that initializes it with a toplevel track and given
    /// nonterminals.
    ///
    /// # Arguments
    /// - `track`: A [`railroad` Node](railroad::Node) that represents the toplevel track to
    ///   render.
    /// - `nonterms`: An [`Iterator`] that yields pairs of names for nonterminals and the
    ///   serialized nonterminals (as [`railroad` Node](railroad::Node)s) themselves.
    ///
    /// # Returns
    /// A new Diagram that will render the given toplevel `track` and given `nonterms`.
    #[inline]
    pub fn with_nonterms(track: impl 'n + railroad::Node, nonterms: impl IntoIterator<Item = (impl Display, impl 'n + railroad::Node)>) -> Self {
        let nonterms = nonterms.into_iter();
        let size_hint: (usize, Option<usize>) = nonterms.size_hint();
        let mut res = Self::with_capacity(track, size_hint.1.unwrap_or(size_hint.0));
        for (name, nonterm) in nonterms {
            res.add_nonterm(name, nonterm);
        }
        res
    }
}

// Collection
impl<'n> Diagram<'n> {
    /// Sets or overrides the toplevel track to render.
    ///
    /// # Arguments
    /// - `track`: A [`railroad` Node](railroad::Node) representing the new to-be-rendered toplevel
    ///   track.
    ///
    /// # Returns
    /// Any track that was previous set, or [`None`] if none was.
    ///
    /// Note that the original type of the node itself is erased due to internal implementation
    /// details.
    #[inline]
    pub fn set_track(&mut self, track: impl 'n + railroad::Node) -> Option<Box<dyn 'n + railroad::Node>> { self.toplevel.replace(Box::new(track)) }

    /// Adds a new nonterminal to the Diagram.
    ///
    /// # Arguments
    /// - `name`: Some name to name the nonterminal.
    /// - `nonterm`: A [`railroad` Node](railroad::Node) that represents its track.
    #[inline]
    pub fn add_nonterm(&mut self, name: impl Display, nonterm: impl 'n + railroad::Node) {
        self.nonterms.push(railroad::LabeledBox::new(Box::new(nonterm), railroad::Comment::new(name.to_string())))
    }
}

// Rendering
impl<'n> Diagram<'n> {
    /* TODO */
}





/***** LIBRARY INTERFACES *****/
/// Marks your node as convertible into a railroad node.
pub trait Railroad {
    /// The type of [`railroad`-node](railroad::Node)-node to which this node is serialized.
    type Output: 'static + railroad::Node;


    /// Serializes this node into a [`railroad`-node](railroad::Node).
    ///
    /// # Returns
    /// A [`Node`](railroad::Node) that can be rendered as a railroad diagram.
    fn to_railroad_node() -> Self::Output;

    /// Serializes a referral to this node instead of the node itself.
    ///
    /// This is useful for when the node itself occurs multiple times and is very large. Instead,
    /// the node generated by this function can be used to refer to its label instead.
    ///
    /// By default, it will generate a label that bears the output of
    /// [`type_name()`](std::any::type_name) for this type.
    ///
    /// # Returns
    /// A [`NonTerminal`](railroad::NonTerminal) that can be rendered as a railroad diagram.
    #[inline]
    fn to_railroad_label() -> railroad::NonTerminal { railroad::NonTerminal::new(std::any::type_name::<Self>().into()) }

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

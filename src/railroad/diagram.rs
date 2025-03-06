//  DIAGRAM.rs
//    by Lut99
//
//  Created:
//    06 Mar 2025, 10:22:10
//  Last edited:
//    06 Mar 2025, 10:28:11
//  Auto updated?
//    Yes
//
//  Description:
//!   Defines the toplevel [`Diagram`] that can serialize a railroad
//!   diagram to SVG and the likes.
//

use std::error;
use std::fmt::{Display, Formatter, Result as FResult};
use std::path::{Path, PathBuf};


/***** ERRORS *****/
/// Defines errors returned by [`Diagram::into_svg_file()`].
#[derive(Debug)]
pub enum Error {
    /// We failed to write to the file.
    FileWrite { path: PathBuf, err: std::io::Error },
}
impl Display for Error {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        match self {
            Self::FileWrite { path, .. } => write!(f, "Failed to write to file {:?}", path.display()),
        }
    }
}
impl error::Error for Error {
    #[inline]
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::FileWrite { err, .. } => Some(err),
        }
    }
}





/***** FORMATTERS *****/
/// Serializes a [`Diagram`] as SVG tags.
///
/// Note that the [`Diagram`] has already been converted to a
/// [`railroad` Diagram](railroad::Diagram) internally with SVG style sheet additions. Mainly, this
/// means that it's cheap to write, and you can easily write it multiple times.
pub struct DiagramSvgFormatter<'n> {
    /// The railroad diagram to serialize.
    diag: railroad::Diagram<railroad::VerticalGrid<Box<dyn 'n + railroad::Node>>>,
}
impl<'n> Display for DiagramSvgFormatter<'n> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult { write!(f, "{}", self.diag) }
}





/***** LIBRARY *****/
/// Allows one to take a serialized [`railroad` Node](railroad::Node) and write it to a file.
///
/// Usually, you should override
/// [`Railroad::to_railroad_diagram()`](super::Railroad::to_railroad_diagram()) on your toplevel
/// node to produce one properly with all the nonterminals in your tree.
pub struct Diagram<'n> {
    /// The toplevel node to make the main track with.
    toplevel: Option<Box<dyn 'n + railroad::Node>>,
    /// A list of additional nonterminals to render.
    nonterms: Vec<railroad::Sequence<Box<dyn 'n + railroad::Node>>>,
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
        self.nonterms.push(railroad::Sequence::new(vec![
            Box::new(railroad::Comment::new(name.to_string())),
            Box::new(railroad::Start),
            Box::new(nonterm),
            Box::new(railroad::End),
        ]))
    }
}

// Rendering
impl<'n> Diagram<'n> {
    /// Returns a formatter that will write this Diagram as SVG tags.
    ///
    /// # Returns
    /// A [`DiagramSvgFormatter`] that implements [`Display`] for writing the SVG text.
    #[inline]
    pub fn into_svg(self) -> DiagramSvgFormatter<'n> {
        let mut diag: railroad::Diagram<_> = self.into();
        diag.add_element(railroad::svg::Element::new("style").set("type", "text/css").text(railroad::DEFAULT_CSS));
        DiagramSvgFormatter { diag }
    }

    /// Attempts to write the Diagram to a given file as SVG.
    ///
    /// This function wraps [`Diagram::into_svg()`] and [`std::fs::write()`] to make it happen.
    ///
    /// # Errors
    /// This function can fail if we failed to write the file.
    #[inline]
    pub fn into_svg_file(self, path: impl AsRef<Path>) -> Result<(), Error> {
        let path: &Path = path.as_ref();
        std::fs::write(path, self.into_svg().to_string()).map_err(|err| Error::FileWrite { path: path.into(), err })
    }
}
impl<'n> From<Diagram<'n>> for railroad::Diagram<railroad::VerticalGrid<Box<dyn 'n + railroad::Node>>> {
    #[inline]
    fn from(value: Diagram<'n>) -> Self {
        let mut items: Vec<Box<dyn 'n + railroad::Node>> = Vec::with_capacity(if value.toplevel.is_some() { 1 } else { 0 } + value.nonterms.len());
        if let Some(toplevel) = value.toplevel {
            items.push(toplevel);
        }
        for nonterm in value.nonterms {
            items.push(Box::new(nonterm));
        }
        railroad::Diagram::new(railroad::VerticalGrid::new(items))
    }
}

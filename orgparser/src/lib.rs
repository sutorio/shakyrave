//! Parser for *a subset of* the org syntax.
//!
//! > IMPORTANT: this is designed to fulfil my requirements, and is *not*
//! > currently general-purpose. See below for decisions taken.
//!
//! This is all based on [pulldown-cmark](https://github.com/raphlinus/pulldown-cmark/),
//! adjusted for the requirements of org, and for my personal requirements.
//!
//! Org can be *treated* as a lightweight markup language, but it is quite a bit
//! more than that. What *I* am interested in is using it to write, and from there
//! to publish the files [primarily] to HTML. This program provides the means to
//! take the contents of an org-mode file and generate an iterable tree structure.
//!
//! During the parsing step, I want to apply certain fixes that may be required:
//! these are documented and implemented in the sibling `orgdoctor` crate.
//!
//! This crate does not deal with writing the tree to {insert target markup language}.
//!
//! However, converting the tree to a markup language allows for testing the 
//! parser by converting it to markup (and testing the converter by converting
//! to org).
//!
//! ## Org syntax/structure
//!
//! The syntax can be divided into "elements" and "objects".
//!
//! An "object" can be thought of as an inline element, something that can exist 
//! within the scope of a paragraph.
//!
//! An `Element` is something that exists at the same or greater scope than a 
//! paragraph. `Element`s can be divided, from broadest to narrowest scope, 
//! into headings, sections, greater elements and lesser elements.
//!
//! These sub-classes define categories of syntactic environments: they are 
//! scopes, and every syntactic component only exists within a specific scope/s,
//! with four exceptions: "headings", "sections", "property drawers" and
//! "planning lines".
//!
//! "lesser elements" are elements that cannot contain any other elements. So a 
//! paragraph is a "lesser element".
//!
//! "greater elements" can contain other greater elements or lesser elements.
//!
//! "sections" can contain both greater and lesser elements.
//!
//! "headings" can contain a section and other headings.
//!
//!
//! ## References
//!
//! - [Org-mode's developer guide to the syntax](https://orgmode.org/worg/org-syntax.html)
//! - [Tutorial on parsing org mode format into a tree structure](http://xahlee.info/emacs/emacs/elisp_parse_org_mode.html)
use std::collections::HashMap;

use nom::{
    bytes::complete::take_until, combinator::map, sequence::{tuple, pair},
    IResult, Parser, multi::many1_count, AsChar, character::complete::satisfy,
};
use nom_supreme::{
    error::ErrorTree,
    tag::complete::tag,
    ParserExt,
};


#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// -----------------------------------------------------------------------------
// Top-level error handling
// -----------------------------------------------------------------------------

/// Provide a boxed error type to propogate errors upwards.
pub type BoxedError = std::boxed::Box<dyn std::error::Error + std::marker::Send + std::marker::Sync>;

/// Wrap result to leverage `BoxedError`.
pub type Result<T> = std::result::Result<T, BoxedError>;

// -----------------------------------------------------------------------------
// Basic combinators
// -----------------------------------------------------------------------------

pub fn markup<'a>(start: &'a str, end: &'a str) -> (impl FnMut(&'a str) -> IResult<&'a str, &'a str>) {
    map(tuple((tag(start), take_until(end), tag(end))), |res| res.1)
}


// -----------------------------------------------------------------------------
// Headings
// -----------------------------------------------------------------------------


/// # Headings
///
/// Although headings aren't the smallest unit of syntax, it makes sense to start
/// coding the parser from here: they are the most important part of the document,
/// and propvide its structure.
///
/// The order they heading elements are described here matches the mandatory 
/// order they must appear in the document.
///
/// ## Examples
///
/// `* TODO [#A] Stuff to do today :todo:task:`
///
///
/// ## Deviations from spec
///
/// - "reduced level" (an integer), which comes after "level", is excised.
/// - "headline" is made mandatory.
///
///
/// ## To add to config
///
/// - Additional keywords + default of "TODO".
/// - Priority range + default of 1/2/3.
///
pub struct Header<'a> {
    /// Level is denoted by one or more `*` characters in the document.
    pub level: usize,
    /// An optional keyword: by default just "TODO". Configurable.
    pub keyword: Option<&'a str>,
    /// An optional integer priority. Configurable.
    pub priority: Option<usize>,
    /// The headline string.
    pub headline: &'a str,
    /// A set of tags.
    pub tags: Option<Vec<&'a str>>,
}

/// Placeholder: the max header size should be configurable.
pub fn is_valid_header_level<'a>(input: &'a usize) -> bool {
    let max_header_size: usize = 6;
    input <= &max_header_size
}

pub fn header_level(input: &str) -> IResult<&str, usize, ErrorTree<&str>> {
    many1_count(tag("*"))
    .verify(is_valid_header_level)
    .terminated(tag(" "))
    .context("A header level is denoted by one two six asterisks followed by a space")
    .parse(input)
}

pub fn keyword(input: &str) -> IResult<&str, Option<&str>, ErrorTree<&str>> {
    unimplemented!()    
}

// TODO: placeholder, need to take the priority cookie & convert it to a useful number.
// pub fn is_valid_priority(input: char) -> bool {
//    ORG_PRIORITY_COOKIES.contains(&input)
// }

pub fn priority(input: &str) -> IResult<&str, Option<&str>, ErrorTree<&str>> {
   markup("#[", "]")
   .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_header_level() {
        let input = "**** ";

        assert_eq!(header_level(input).unwrap(), ("", 4));
    }
}

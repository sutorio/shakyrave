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
//!
//! ## References
//!
//! - [Org-mode's developer guide to the syntax](https://orgmode.org/worg/org-syntax.html)
//! - [Tutorial on parsing org mode format into a tree structure](http://xahlee.info/emacs/emacs/elisp_parse_org_mode.html)
use std::{convert::TryFrom, fmt::Display};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// -----------------------------------------------------------------------------
// Errors
// -----------------------------------------------------------------------------

pub type BoxedError = std::boxed::Box<dyn 
    std::error::Error   // required to satisfy ?
    + std::marker::Send // required for threads
    + std::marker::Sync // required for threads
>;

pub type Result<T> = std::Result<T, BoxedError>;

// -----------------------------------------------------------------------------
// Reference Example (this will be referred to throughout):
// -----------------------------------------------------------------------------

/*
org syntax example

* header
#+TITLE: Tasks
* TODO [#A] hh.1 :tag1:tag2:
:PROPERTIES:
:OWNER: Dav
:ID: 123
:END:
*some* thing in water
** hh.1.1
unordered lists
- bold *love*
- italic /slanted text/
- underline _undies_
- verbatim =as is=
- code ~1+1=3~ but +wrong+
#+BEGIN_CENTER
does not compute
#+END_CENTER
*** hh.1.1.1
#+BEGIN_EXAMPLE
var x = 3
x + 4
#+END_EXAMPLE
*** hh.1.1.2
DEADLINE: <2019-01-25 Fri>
learn emacs lisp
#+BEGIN_SRC emacs-lisp
(+ 3 4)
#+END_SRC
** hh.1.2
** hh.1.3
* hh.2
** hh.2.1
another date [2019-01-25 Fri]
more date <2019-01-25 Fri 20:08>
*/


// > Any Org document is represented by a sequence of elements, that can 
// > recursively contain other elements and/or objects.

// -----------------------------------------------------------------------------
// SECTION TITLE
// -----------------------------------------------------------------------------

// The syntax can be divided into "elements" and "objects".

/// An `Element` is something that exists at the same or greater scope than a 
/// paragraph. `Element`s can be divided, from broadest to narrowest scope, 
/// into headings, sections, greater elements and lesser elements.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Element<'a> {
    Heading {
        level: HeadingLevel,
        keyword: Option<String>,
    },
    Section,
    GreaterElement,
    LesserElement,
}

/// The end of an `Element`
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ElementEnd {
    Heading,
    Section,
    GreaterElement,
    LesserElement,
}

pub enum Object<'a> {
    PlainText,
    Markup(MarkupType),
    // TODO: Entity,
    // TODO: LaTeXFragment,
    Superscript,
    Subscript,
}

pub enum ObjectEnd { }


pub enum MarkupType {
    Bold,
    Italic,
    Underline,
    Verbatim,
    Code,
    StrikeThru,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum HeadingLevel {
    H1 = 1,
    H2,
    H3,
    H4,
    H5,
    H6,
}


/// Returned when trying to convert a `usize` into a `Heading` but it fails
/// because the usize isn't a valid heading level
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct InvalidHeadingLevel(usize);

impl TryFrom<usize> for HeadingLevel {
    type Error = InvalidHeadingLevel;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::H1),
            2 => Ok(Self::H2),
            3 => Ok(Self::H3),
            4 => Ok(Self::H4),
            5 => Ok(Self::H5),
            6 => Ok(Self::H6),
            _ => Err(InvalidHeadingLevel(value)),
        }
    }
}

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
//! An "object" can be thought of as an inline element, something that can exist 
//! within the scope of a paragraph.
//!
//! ## The org file structure
//!
//!

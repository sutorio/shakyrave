//! Parsing functionality for *a subset of* the org syntax.
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


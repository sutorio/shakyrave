//! # OrgDoctor
//! 
//! Small utility library to clean up any issues with org files that might make 
//! them difficult to parse.
//!
//! ## Issue 1: Zeroth section
//!
//! The mobile application [Orgzly](https://orgzly.com/) is great, but each 
//! "notebook" (what it calls individual .org files) is simply a collection
//! of headers. This is completely legitimate, and completely valid, but to
//! _publish_ these files as HTML, I want to ensure that a
//! [zeroth section](https://orgmode.org/worg/org-syntax.html#org391ad58) is 
//! always present. That zeroth section will need at the bare minimum a title
//! and a preface.
//!
//! What I want is to always be able to populate `<head>` information such as
//! the document title and description, and that same for the visible top-level
//! `<header>`.
//!
//! ## Issue 2: Unique IDs
//!
//! The drawer for each header can have a custom ID.
//!
//! ```org
//! * My header
//!   :PROPERTIES:
//!   :CUSTOM_ID: id123
//!   :END:
//!
//!   Yadda yadda yadda.
//! ```
//! This needs to be enforced, and added if not present -- this will enable 
//! cross-referencing and linking (_a la_ Zettel/Obsidian/etc). Note that the
//! zeroth section also allows for properties, so same applies there.
//!
use orgize::Org;



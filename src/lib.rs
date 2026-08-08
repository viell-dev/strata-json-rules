//! RFC 8259 JSON token rules for the Strata parser ecosystem.
//!
//! Each rule is a zero-sized [`Matcher`](strata_lexer::Matcher) recognizing one RFC 8259 token
//! shape from read-only lookahead. A rule *measures* how far it would match and never consumes
//! input; a [`SpecScanner`](strata_lexer::SpecScanner) driver owns all consumption. Rules are
//! exported individually so superset languages (JSONC, JSON5, ...) can reuse exactly the shapes
//! they share with JSON, lifting [`JsonToken`] into their own kind enum via
//! [`MatcherExt::map_kind`](strata_lexer::MatcherExt::map_kind), and add or replace only what
//! differs (e.g. a comment rule).
//!
//! String and number rules validate *lexical shape* here — bad escapes, unescaped control
//! characters, malformed `\uXXXX`, invalid UTF-8, and number-grammar violations are lexical
//! failures reported at the offending byte. Escape decoding and numeric conversion stay
//! downstream, at the resolver boundary.
//!
//! The crate is standalone on purpose: a parser for a superset language depends on these rules
//! (and its own additions) without pulling in the JSON parser.

mod kind;
mod rules;
mod shared;

pub use kind::JsonToken;
pub use rules::{
    BomRule, LiteralRule, NumberRule, PunctRule, StrictRuleSet, StringRule, WhitespaceRule,
    strict_rule_set, strict_scanner,
};
pub use shared::SharedRuleSet;

/// Compile-checks the README examples alongside the doctests.
#[cfg(doctest)]
#[doc = include_str!("../README.md")]
mod readme {}

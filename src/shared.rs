//! A cheaply cloneable, shared rule set.

use std::sync::Arc;
use strata_lexer::{Candidate, LexError, Lookahead, MatcherSet};

/// A cheaply cloneable, shared rule set.
///
/// [`MatcherVec`](strata_lexer::MatcherVec) and other non-`Clone` rule sets cannot be handed to a
/// reusable parser directly, because the parser clones its rules into a fresh scanner per parse.
/// `SharedRuleSet` wraps any [`MatcherSet`] in an [`Arc`] so cloning shares one set. It is
/// generic over the kind, so superset crates can use it for their own rule sets too.
#[derive(Debug)]
pub struct SharedRuleSet<M> {
    rules: Arc<M>,
}

impl<M> SharedRuleSet<M> {
    /// Wraps `rules` for shared, cloneable use.
    pub fn new(rules: M) -> Self {
        Self {
            rules: Arc::new(rules),
        }
    }
}

impl<M> Clone for SharedRuleSet<M> {
    fn clone(&self) -> Self {
        Self {
            rules: Arc::clone(&self.rules),
        }
    }
}

impl<M: MatcherSet> MatcherSet for SharedRuleSet<M> {
    type Kind = M::Kind;

    fn measure(&self, look: &mut Lookahead<'_>) -> Result<Vec<Candidate<Self::Kind>>, LexError> {
        self.rules.measure(look)
    }
}

//! The RFC 8259 token kind enum.

/// Token kinds produced by the RFC 8259 token rules.
///
/// Kinds are payload-free: a token records *what was found and where* (kind plus byte span), and
/// the source bytes stay addressable through the retained source. Superset grammars typically
/// wrap this enum in their own kind (e.g. `JsoncToken::Json(JsonToken)`) and lift the rules with
/// [`MatcherExt::map_kind`](strata_lexer::MatcherExt::map_kind).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum JsonToken {
    /// `{` — begin-object structural character (RFC 8259 §2).
    LeftBrace,
    /// `}` — end-object structural character (RFC 8259 §2).
    RightBrace,
    /// `[` — begin-array structural character (RFC 8259 §2).
    LeftBracket,
    /// `]` — end-array structural character (RFC 8259 §2).
    RightBracket,
    /// `:` — name-separator structural character (RFC 8259 §2).
    Colon,
    /// `,` — value-separator structural character (RFC 8259 §2).
    Comma,
    /// The `true` literal (RFC 8259 §3).
    True,
    /// The `false` literal (RFC 8259 §3).
    False,
    /// The `null` literal (RFC 8259 §3).
    Null,
    /// A complete string token, span covering both quotes (RFC 8259 §7).
    String,
    /// A complete number token (RFC 8259 §6).
    Number,
    /// A run of insignificant whitespace: space, tab, CR, LF (RFC 8259 §2).
    Whitespace,
    /// A byte-order mark (U+FEFF) at the very start of the input (RFC 8259 §8.1).
    Bom,
    /// End of input.
    Eof,
}

impl JsonToken {
    /// Returns whether this kind is trivia — grammatically insignificant source content that a
    /// fidelity-preserving CST retains instead of discarding.
    pub const fn is_trivia(self) -> bool {
        matches!(self, Self::Whitespace | Self::Bom)
    }

    /// A short human-readable description for diagnostics.
    pub const fn describe(self) -> &'static str {
        match self {
            Self::LeftBrace => "'{'",
            Self::RightBrace => "'}'",
            Self::LeftBracket => "'['",
            Self::RightBracket => "']'",
            Self::Colon => "':'",
            Self::Comma => "','",
            Self::True => "'true'",
            Self::False => "'false'",
            Self::Null => "'null'",
            Self::String => "a string",
            Self::Number => "a number",
            Self::Whitespace => "whitespace",
            Self::Bom => "a byte-order mark",
            Self::Eof => "end of input",
        }
    }
}

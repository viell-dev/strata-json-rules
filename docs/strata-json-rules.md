# strata-json-rules

`strata-json-rules` is the RFC 8259 JSON token-rules crate for the Strata parser ecosystem: the
lexical shapes of JSON as composable, read-only matchers on `strata-lexer`'s matcher layer. It is
standalone on purpose — a superset language (JSONC, JSON5, ...) depends on just these rules and
adds its own, without pulling in the JSON parser.

The crate sits at the rules layer of the per-language stack:

```text
strata-lexer -> strata-json-rules (this crate) -> strata-json-parser -> strata-json
```

- **Edition / MSRV:** Rust 2024, MSRV `1.87.0`.
- **License:** MPL-2.0.
- **Dependencies:** `strata-lexer` only (which brings `strata-reader` and `strata-lease`).

## 1. Module and public-API map

| Module   | Public items |
|----------|--------------|
| `kind`   | `JsonToken` (with `is_trivia()`, `describe()`) |
| `rules`  | `WhitespaceRule`, `PunctRule`, `LiteralRule`, `StringRule`, `NumberRule`, `BomRule`, `StrictRuleSet`, `strict_rule_set()`, `strict_scanner()` |
| `shared` | `SharedRuleSet<M>` |

Everything is re-exported at the crate root.

## 2. Token kinds

`JsonToken` is a payload-free, `Copy` kind enum: the six structural characters, `True`/`False`/
`Null`, `String`, `Number`, `Whitespace`, `Bom`, and `Eof`. A token records *what was found and
where* (kind plus byte span); the source bytes stay addressable through the retained source.
Superset grammars wrap the enum in their own kind (e.g. `JsoncToken::Json(JsonToken)`) and lift
the rules with `MatcherExt::map_kind`.

## 3. The rules

Each rule is a zero-sized `strata_lexer::Matcher` recognizing one RFC 8259 token shape from
read-only lookahead (measure-then-advance; a declined match cannot move the cursor). Rules that
measure a possibly-zero length decline through `Match::from_len` — a zero-length match is
unrepresentable in the lexer by construction.

- `WhitespaceRule` — a run of exactly space/tab/CR/LF (§2).
- `PunctRule` — `{` `}` `[` `]` `:` `,` (§2).
- `LiteralRule` — exactly `true`, `false`, `null` (§3).
- `StringRule` — a complete string including both quotes (§7). Validates *lexical shape* at the
  offending byte: unescaped control characters, escapes outside `\" \\ \/ \b \f \n \r \t \uXXXX`,
  `\u` without four hex digits, and invalid UTF-8 are lexical errors
  (`LexErrorKind::Unrecognized` with a precise position; unclosed strings are
  `UnterminatedDelimiter` at the opening quote). Lone surrogate escapes are accepted (§8.2
  discourages, does not forbid); surrogate-pair combination is resolver work, served by
  `strata-json-parser`'s `decode` module.
- `NumberRule` — the exact §6 grammar `[ "-" ] int [ frac ] [ exp ]`. Leading zeros and empty
  fraction/exponent parts are lexical errors; a bare `-`/`+`/`.` matches nothing.
- `BomRule` — the UTF-8 encoding of U+FEFF at byte offset 0 only (§8.1). The rule recognizes the
  shape; the *parser* owns the policy (reject strictly, or retain as trivia under `allow_bom`).
  This buys a precise "leading byte-order mark" diagnostic instead of a generic unrecognized
  byte.

`StrictRuleSet` is the monomorphic tuple of all six rules; `strict_rule_set()` constructs it and
`strict_scanner()` wraps it in a `SpecScanner` with `JsonToken::Eof`. `SharedRuleSet<M>` is an
`Arc`-backed `MatcherSet` wrapper making non-`Clone` rule sets (e.g. `MatcherVec`) cheaply
cloneable for a reusable parser; it is kind-generic, so superset crates can use it too.

## 4. Composition points for supersets

A superset rules crate defines its own kind enum, lifts the shared shapes, and adds or replaces
only what differs:

```rust
enum JsoncToken { Json(JsonToken), Comment, Eof }

let wrap: fn(JsonToken) -> JsoncToken = JsoncToken::Json;
let rules: MatcherVec<JsoncToken> = vec![
    LineCommentRule.boxed(),                  // the shape the superset adds
    WhitespaceRule.map_kind(wrap).boxed(),    // everything shared with JSON, lifted
    PunctRule.map_kind(wrap).boxed(),
    LiteralRule.map_kind(wrap).boxed(),
    StringRule.map_kind(wrap).boxed(),
    NumberRule.map_kind(wrap).boxed(),
];
let scanner = SpecScanner::new(rules, JsoncToken::Eof);
```

Rules to keep are lifted with `MatcherExt::map_kind`; rules to change are simply not included
(`BomRule` is omitted above); new rules are added alongside. Reused rules keep their validation —
`01` stays a lexical error inside the superset. `tests/composition.rs` is the worked example;
`strata-jsonc-rules` and `strata-json5-rules` are the real ones.

## 5. Design choices

- **Validation at the lexical boundary.** String/number shape violations are caught here, so a
  parsed CST is known-well-formed without any decoding pass. The cost: such failures surface as
  `LexErrorKind::Unrecognized` with an exact position rather than a named "bad escape" kind.
- **Rules standalone, policy downstream.** The rules describe which shapes *exist*; parsers own
  BOM policy, depth limits, and option gating. That split is what makes the rules reusable by
  supersets without dragging parser decisions along.
- **Sub-token error positions** advance on the token's start line via `Position::advanced_by`,
  which is exact because no JSON string/number lexeme byte before the offending offset can be a
  line break.

## 6. Quality bar

Same as the ecosystem: `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D
warnings` under the strict lint set in `Cargo.toml`, and `cargo test` all clean. Lint exceptions
use `#[expect(lint, reason = "…")]` on the smallest existing scope.

## 7. Limitations and follow-up

- The public API is not settled; this is an initial-development crate on an experimental branch.
- Package verification (`cargo package`) is blocked until `strata-lexer` is published and the
  path dependency gains a version.

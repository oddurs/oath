//! Source text to tokens.
//!
//! Every token carries the byte span it came from, so slicing the source by a
//! token's span always yields the text that produced it. That property is what
//! lets every later stage report a position without threading one through by
//! hand, and it is tested rather than assumed.
//!
//! Layout is not significant: indentation means nothing. A line break does,
//! because it separates one declaration or match arm from the next, so runs of
//! blank lines and comments collapse into a single [`TokenKind::Newline`] and the
//! parser decides where a separator matters.

use crate::diagnostic::{Diagnostic, Span};

/// What a token is. Text is not carried here: the span plus the source gives it
/// back, so there is one representation of a token's text rather than two that
/// can disagree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    /// A decimal integer, already parsed, so overflow is caught with a span
    /// rather than surfacing much later as a wrong answer.
    Int(i64),
    /// A quoted string with its escapes resolved. Keeper labels are the only
    /// use today, which is why the escape set is deliberately small.
    Str(String),
    /// An identifier or a type name. Keywords are separate variants, so the
    /// parser never compares strings.
    Ident,

    Oath,
    Eg,
    Keep,
    By,
    Let,
    In,
    If,
    Then,
    Else,
    Match,
    With,
    Fn,

    /// `->`
    Arrow,
    /// `:`
    Colon,
    /// `,`
    Comma,
    /// `|`
    Pipe,
    /// `;`
    Semi,
    /// `=`
    Eq,
    /// `==`
    EqEq,
    /// `!=`
    NotEq,
    /// `<`
    Lt,
    /// `<=`
    LtEq,
    /// `>`
    Gt,
    /// `>=`
    GtEq,
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
    LBracket,
    RBracket,

    /// One or more line breaks, with any blank lines and comments between them.
    Newline,
    /// The end of the source. Emitted so the parser always has something to
    /// report a position against, even for an empty file.
    Eof,
}

impl TokenKind {
    /// How this token reads in a diagnostic. Variants carrying text describe
    /// their shape rather than their value, because the message already quotes
    /// the source where the value matters.
    pub fn describe(&self) -> &'static str {
        match self {
            TokenKind::Int(_) => "an integer",
            TokenKind::Str(_) => "a string",
            TokenKind::Ident => "an identifier",
            TokenKind::Oath => "`oath`",
            TokenKind::Eg => "`eg`",
            TokenKind::Keep => "`keep`",
            TokenKind::By => "`by`",
            TokenKind::Let => "`let`",
            TokenKind::In => "`in`",
            TokenKind::If => "`if`",
            TokenKind::Then => "`then`",
            TokenKind::Else => "`else`",
            TokenKind::Match => "`match`",
            TokenKind::With => "`with`",
            TokenKind::Fn => "`fn`",
            TokenKind::Arrow => "`->`",
            TokenKind::Colon => "`:`",
            TokenKind::Comma => "`,`",
            TokenKind::Pipe => "`|`",
            TokenKind::Semi => "`;`",
            TokenKind::Eq => "`=`",
            TokenKind::EqEq => "`==`",
            TokenKind::NotEq => "`!=`",
            TokenKind::Lt => "`<`",
            TokenKind::LtEq => "`<=`",
            TokenKind::Gt => "`>`",
            TokenKind::GtEq => "`>=`",
            TokenKind::Plus => "`+`",
            TokenKind::Minus => "`-`",
            TokenKind::Star => "`*`",
            TokenKind::Slash => "`/`",
            TokenKind::LParen => "`(`",
            TokenKind::RParen => "`)`",
            TokenKind::LBracket => "`[`",
            TokenKind::RBracket => "`]`",
            TokenKind::Newline => "a line break",
            TokenKind::Eof => "the end of the file",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    /// The source text this token came from.
    pub fn text<'a>(&self, src: &'a str) -> &'a str {
        self.span.text(src).unwrap_or_default()
    }
}

/// Tokenise `src`, stopping at the first thing that is not a token.
///
/// One diagnostic rather than a list: a stray character usually means the rest of
/// the file will not lex sensibly either, and a cascade of invented errors is
/// worse than one true one.
pub fn tokens(src: &str) -> Result<Vec<Token>, Diagnostic> {
    Lexer::new(src).run()
}

struct Lexer<'a> {
    src: &'a str,
    pos: usize,
    out: Vec<Token>,
}

impl<'a> Lexer<'a> {
    fn new(src: &'a str) -> Self {
        Self {
            src,
            pos: 0,
            out: Vec::new(),
        }
    }

    fn peek(&self) -> Option<char> {
        self.src[self.pos..].chars().next()
    }

    fn peek_second(&self) -> Option<char> {
        let mut cs = self.src[self.pos..].chars();
        cs.next();
        cs.next()
    }

    /// Advance one character. Stepping by the character's UTF-8 width is what
    /// keeps every span on a boundary, so `Span::text` can never fail.
    fn bump(&mut self) -> Option<char> {
        let c = self.peek()?;
        self.pos += c.len_utf8();
        Some(c)
    }

    fn eat(&mut self, want: char) -> bool {
        if self.peek() == Some(want) {
            self.pos += want.len_utf8();
            true
        } else {
            false
        }
    }

    fn push(&mut self, kind: TokenKind, start: usize) {
        self.out.push(Token {
            kind,
            span: Span::new(start, self.pos),
        });
    }

    fn run(mut self) -> Result<Vec<Token>, Diagnostic> {
        loop {
            self.skip_inline_trivia();

            let start = self.pos;
            let Some(c) = self.peek() else {
                self.out.push(Token {
                    kind: TokenKind::Eof,
                    span: Span::at(self.pos),
                });
                return Ok(self.out);
            };

            match c {
                '\n' | '\r' => self.line_break(start),
                '0'..='9' => self.number(start)?,
                '"' => self.string(start)?,
                c if is_ident_start(c) => self.word(start),
                _ => {
                    self.bump();
                    match c {
                        '(' => self.push(TokenKind::LParen, start),
                        ')' => self.push(TokenKind::RParen, start),
                        '[' => self.push(TokenKind::LBracket, start),
                        ']' => self.push(TokenKind::RBracket, start),
                        ':' => self.push(TokenKind::Colon, start),
                        ',' => self.push(TokenKind::Comma, start),
                        '|' => self.push(TokenKind::Pipe, start),
                        ';' => self.push(TokenKind::Semi, start),
                        '+' => self.push(TokenKind::Plus, start),
                        '*' => self.push(TokenKind::Star, start),
                        '/' => self.push(TokenKind::Slash, start),
                        '-' => {
                            // `--` is a comment, which skip_inline_trivia has
                            // already consumed, so a `-` here is subtraction or
                            // the head of `->`.
                            if self.eat('>') {
                                self.push(TokenKind::Arrow, start);
                            } else {
                                self.push(TokenKind::Minus, start);
                            }
                        }
                        '=' => {
                            if self.eat('=') {
                                self.push(TokenKind::EqEq, start);
                            } else {
                                self.push(TokenKind::Eq, start);
                            }
                        }
                        '<' => {
                            if self.eat('=') {
                                self.push(TokenKind::LtEq, start);
                            } else {
                                self.push(TokenKind::Lt, start);
                            }
                        }
                        '>' => {
                            if self.eat('=') {
                                self.push(TokenKind::GtEq, start);
                            } else {
                                self.push(TokenKind::Gt, start);
                            }
                        }
                        '!' => {
                            if self.eat('=') {
                                self.push(TokenKind::NotEq, start);
                            } else {
                                return Err(Diagnostic::new(
                                    Span::new(start, self.pos),
                                    "`!` is not an operator here",
                                )
                                .with_hint("did you mean `!=`?"));
                            }
                        }
                        _ => {
                            return Err(Diagnostic::new(
                                Span::new(start, self.pos),
                                format!("unexpected character `{c}`"),
                            ));
                        }
                    }
                }
            }
        }
    }

    /// Spaces, tabs and comments, but never a line break: those are tokens.
    fn skip_inline_trivia(&mut self) {
        loop {
            match self.peek() {
                Some(' ') | Some('\t') => {
                    self.bump();
                }
                Some('-') if self.peek_second() == Some('-') => {
                    while !matches!(self.peek(), None | Some('\n') | Some('\r')) {
                        self.bump();
                    }
                }
                _ => return,
            }
        }
    }

    /// One token for a run of line breaks, so blank lines between declarations
    /// do not each become a separator the parser has to skip.
    fn line_break(&mut self, start: usize) {
        self.bump();
        if self.src[start..self.pos].starts_with('\r') {
            self.eat('\n');
        }
        let first_break_end = self.pos;

        loop {
            let mark = self.pos;
            self.skip_inline_trivia();
            match self.peek() {
                Some('\n') => {
                    self.bump();
                }
                Some('\r') => {
                    self.bump();
                    self.eat('\n');
                }
                _ => {
                    // Trailing spaces belong to whatever comes next, not to the
                    // separator, so give them back.
                    self.pos = mark;
                    break;
                }
            }
        }

        // The span is the first break alone: a separator's text is one line
        // ending, however many blank lines followed it.
        self.out.push(Token {
            kind: TokenKind::Newline,
            span: Span::new(start, first_break_end),
        });
    }

    fn number(&mut self, start: usize) -> Result<(), Diagnostic> {
        while matches!(self.peek(), Some('0'..='9')) {
            self.bump();
        }

        // `12abc` is a typo, not an integer beside a name. Saying so here beats
        // a confusing parse error two tokens later.
        if let Some(c) = self.peek()
            && is_ident_start(c)
        {
            let bad = self.pos;
            while matches!(self.peek(), Some(c) if is_ident_continue(c)) {
                self.bump();
            }
            return Err(Diagnostic::new(
                Span::new(bad, self.pos),
                "a number cannot be followed directly by a letter",
            )
            .with_hint("put a space between them"));
        }

        let text = &self.src[start..self.pos];
        let value = text.parse::<i64>().map_err(|_| {
            Diagnostic::new(
                Span::new(start, self.pos),
                format!("integer `{text}` is out of range"),
            )
            .with_hint("integers run from -9223372036854775808 to 9223372036854775807")
        })?;

        self.push(TokenKind::Int(value), start);
        Ok(())
    }

    fn string(&mut self, start: usize) -> Result<(), Diagnostic> {
        self.bump(); // the opening quote
        let mut value = String::new();

        loop {
            let at = self.pos;
            let Some(c) = self.bump() else {
                return Err(
                    Diagnostic::new(Span::new(start, self.pos), "unterminated string")
                        .with_hint("add a closing `\"`"),
                );
            };

            match c {
                '"' => {
                    self.push(TokenKind::Str(value), start);
                    return Ok(());
                }
                // A line break inside a string is far more often a missing quote
                // than a deliberate multi-line label, and the error points at the
                // opening quote rather than at the end of the file.
                '\n' | '\r' => {
                    return Err(Diagnostic::new(
                        Span::new(start, at),
                        "unterminated string at the end of the line",
                    )
                    .with_hint("add a closing `\"`"));
                }
                '\\' => {
                    // `at` is the backslash, so the span covers the whole
                    // escape rather than just the character after it.
                    match self.bump() {
                        Some('"') => value.push('"'),
                        Some('\\') => value.push('\\'),
                        Some('n') => value.push('\n'),
                        Some('t') => value.push('\t'),
                        Some(other) => {
                            return Err(Diagnostic::new(
                                Span::new(at, self.pos),
                                format!("`\\{other}` is not an escape"),
                            )
                            .with_hint("the escapes are `\\\"`, `\\\\`, `\\n` and `\\t`"));
                        }
                        None => {
                            return Err(Diagnostic::new(
                                Span::new(start, self.pos),
                                "unterminated string",
                            )
                            .with_hint("add a closing `\"`"));
                        }
                    }
                }
                _ => value.push(c),
            }
        }
    }

    fn word(&mut self, start: usize) {
        while matches!(self.peek(), Some(c) if is_ident_continue(c)) {
            self.bump();
        }
        let kind = keyword(&self.src[start..self.pos]).unwrap_or(TokenKind::Ident);
        self.push(kind, start);
    }
}

/// The v0.1 keywords. Later ones arrive with the features that need them, so a
/// word here is a word the parser can already do something with.
fn keyword(text: &str) -> Option<TokenKind> {
    Some(match text {
        "oath" => TokenKind::Oath,
        "eg" => TokenKind::Eg,
        "keep" => TokenKind::Keep,
        "by" => TokenKind::By,
        "let" => TokenKind::Let,
        "in" => TokenKind::In,
        "if" => TokenKind::If,
        "then" => TokenKind::Then,
        "else" => TokenKind::Else,
        "match" => TokenKind::Match,
        "with" => TokenKind::With,
        "fn" => TokenKind::Fn,
        _ => return None,
    })
}

/// ASCII only, plus `_`. Unicode identifiers would have to settle normalisation
/// before a name could take part in a hash, and that is the hashing spike's
/// question, not this one's.
fn is_ident_start(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

fn is_ident_continue(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::panic)]

    use super::*;
    use std::collections::HashSet;
    use std::mem::discriminant;

    fn lex(src: &str) -> Vec<Token> {
        tokens(src).unwrap_or_else(|d| panic!("expected {src:?} to lex, got: {d}"))
    }

    fn kinds(src: &str) -> Vec<TokenKind> {
        lex(src).into_iter().map(|t| t.kind).collect()
    }

    fn error(src: &str) -> Diagnostic {
        tokens(src).expect_err(&format!("expected {src:?} to fail"))
    }

    /// Every variant, so coverage can be asserted rather than hoped for. A new
    /// token kind must be added here; `TokenKind::describe` has an exhaustive
    /// match, so the compiler already forces a visit to this file.
    fn all_kinds() -> Vec<TokenKind> {
        vec![
            TokenKind::Int(0),
            TokenKind::Str(String::new()),
            TokenKind::Ident,
            TokenKind::Oath,
            TokenKind::Eg,
            TokenKind::Keep,
            TokenKind::By,
            TokenKind::Let,
            TokenKind::In,
            TokenKind::If,
            TokenKind::Then,
            TokenKind::Else,
            TokenKind::Match,
            TokenKind::With,
            TokenKind::Fn,
            TokenKind::Arrow,
            TokenKind::Colon,
            TokenKind::Comma,
            TokenKind::Pipe,
            TokenKind::Semi,
            TokenKind::Eq,
            TokenKind::EqEq,
            TokenKind::NotEq,
            TokenKind::Lt,
            TokenKind::LtEq,
            TokenKind::Gt,
            TokenKind::GtEq,
            TokenKind::Plus,
            TokenKind::Minus,
            TokenKind::Star,
            TokenKind::Slash,
            TokenKind::LParen,
            TokenKind::RParen,
            TokenKind::LBracket,
            TokenKind::RBracket,
            TokenKind::Newline,
            TokenKind::Eof,
        ]
    }

    /// A source exercising every token kind at once.
    const CORPUS: &str = r#"oath sort : List Int -> List Int
  eg sort [3,1,2] == [1,2,3]

keep sort by "reference"
  -- the empty list is already sorted
  | [] -> 0
  | (x:xs) -> let n = x * 2 / 1 - 1 in
      if n >= 1 then match n with | _ -> fn y -> y + 1 else 0;
  | z -> z != 9 < 8 <= 7 > 6
"#;

    #[test]
    fn the_corpus_produces_every_token_kind() {
        // Compared by discriminant rather than by `describe`, so two kinds
        // that happen to read the same in a message cannot mask a gap.
        let seen: HashSet<_> = kinds(CORPUS).iter().map(discriminant).collect();
        let missing: Vec<_> = all_kinds()
            .iter()
            .filter(|k| !seen.contains(&discriminant(*k)))
            .map(|k| k.describe())
            .collect();
        assert!(missing.is_empty(), "the corpus never produces: {missing:?}");
    }

    #[test]
    fn every_span_slices_back_to_the_token_it_came_from() {
        for token in lex(CORPUS) {
            let text = token
                .span
                .text(CORPUS)
                .unwrap_or_else(|| panic!("{:?} has a span off a boundary", token.kind));
            match &token.kind {
                TokenKind::Eof => assert_eq!(text, "", "eof spans nothing"),
                TokenKind::Newline => {
                    assert!(
                        text == "\n" || text == "\r\n",
                        "a separator is one line ending"
                    )
                }
                TokenKind::Int(v) => assert_eq!(text.parse::<i64>().ok(), Some(*v)),
                TokenKind::Str(_) => {
                    assert!(
                        text.starts_with('"') && text.ends_with('"'),
                        "quotes are in the span"
                    )
                }
                TokenKind::Ident => assert!(is_ident_start(text.chars().next().unwrap())),
                other => {
                    // Every fixed token's text is exactly what `describe` quotes.
                    let quoted = other.describe().trim_matches('`');
                    assert_eq!(text, quoted, "{other:?} should span {quoted:?}");
                }
            }
        }
    }

    #[test]
    fn a_span_is_never_wider_than_the_source() {
        for token in lex(CORPUS) {
            assert!(token.span.end <= CORPUS.len());
            assert!(token.span.start <= token.span.end);
        }
    }

    #[test]
    fn keywords_are_their_own_kinds_and_near_misses_are_not() {
        assert_eq!(kinds("oath")[0], TokenKind::Oath);
        assert_eq!(kinds("keep")[0], TokenKind::Keep);
        assert_eq!(kinds("fn")[0], TokenKind::Fn);
        for near in ["oaths", "keeper", "Oath", "fnord", "ifx", "_let", "let_"] {
            assert_eq!(kinds(near)[0], TokenKind::Ident, "{near} is not a keyword");
        }
    }

    #[test]
    fn an_underscore_is_an_identifier() {
        assert_eq!(kinds("_"), vec![TokenKind::Ident, TokenKind::Eof]);
        assert_eq!(kinds("_x1"), vec![TokenKind::Ident, TokenKind::Eof]);
    }

    #[test]
    fn comments_run_to_the_end_of_the_line_and_leave_the_break() {
        assert_eq!(
            kinds("1 -- two\n3"),
            vec![
                TokenKind::Int(1),
                TokenKind::Newline,
                TokenKind::Int(3),
                TokenKind::Eof
            ]
        );
    }

    #[test]
    fn a_comment_may_end_the_file_without_a_line_break() {
        assert_eq!(
            kinds("1 -- trailing"),
            vec![TokenKind::Int(1), TokenKind::Eof]
        );
        assert_eq!(kinds("-- only a comment"), vec![TokenKind::Eof]);
    }

    #[test]
    fn a_comment_line_between_declarations_does_not_add_a_separator() {
        assert_eq!(
            kinds("1\n-- a note\n2"),
            vec![
                TokenKind::Int(1),
                TokenKind::Newline,
                TokenKind::Int(2),
                TokenKind::Eof
            ]
        );
    }

    #[test]
    fn blank_lines_collapse_into_one_separator() {
        assert_eq!(
            kinds("1\n\n\n  \n2"),
            vec![
                TokenKind::Int(1),
                TokenKind::Newline,
                TokenKind::Int(2),
                TokenKind::Eof
            ]
        );
    }

    #[test]
    fn indentation_is_not_a_token() {
        assert_eq!(
            kinds("1\n\t\t2"),
            vec![
                TokenKind::Int(1),
                TokenKind::Newline,
                TokenKind::Int(2),
                TokenKind::Eof
            ]
        );
    }

    #[test]
    fn carriage_returns_are_one_separator_not_two() {
        assert_eq!(
            kinds("1\r\n2"),
            vec![
                TokenKind::Int(1),
                TokenKind::Newline,
                TokenKind::Int(2),
                TokenKind::Eof
            ]
        );
        let toks = lex("1\r\n2");
        assert_eq!(toks[1].span.text("1\r\n2"), Some("\r\n"));
    }

    #[test]
    fn an_empty_source_is_just_the_end_of_the_file() {
        assert_eq!(kinds(""), vec![TokenKind::Eof]);
        assert_eq!(kinds("   \t "), vec![TokenKind::Eof]);
        let toks = lex("");
        assert!(toks[0].span.is_empty());
    }

    #[test]
    fn a_source_of_only_a_line_break_still_separates() {
        assert_eq!(kinds("\n"), vec![TokenKind::Newline, TokenKind::Eof]);
    }

    #[test]
    fn two_character_operators_win_over_one() {
        assert_eq!(
            kinds("= == -> - <= < >= > !="),
            vec![
                TokenKind::Eq,
                TokenKind::EqEq,
                TokenKind::Arrow,
                TokenKind::Minus,
                TokenKind::LtEq,
                TokenKind::Lt,
                TokenKind::GtEq,
                TokenKind::Gt,
                TokenKind::NotEq,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn subtraction_needs_a_space_before_a_negative_because_two_dashes_are_a_comment() {
        // Documented rather than worked around: every language with `--`
        // comments has this, and the alternative is a special case in the
        // scanner that surprises the reader instead.
        assert_eq!(
            kinds("1 - 2"),
            vec![
                TokenKind::Int(1),
                TokenKind::Minus,
                TokenKind::Int(2),
                TokenKind::Eof
            ]
        );
        assert_eq!(kinds("1--2"), vec![TokenKind::Int(1), TokenKind::Eof]);
    }

    #[test]
    fn integers_are_parsed_at_lex_time() {
        assert_eq!(kinds("0 7 9223372036854775807")[0], TokenKind::Int(0));
        assert_eq!(
            kinds("0 7 9223372036854775807")[2],
            TokenKind::Int(9223372036854775807)
        );
    }

    #[test]
    fn an_integer_too_large_to_hold_is_an_error_with_its_span() {
        let d = error("1 + 99999999999999999999");
        assert!(d.message.contains("out of range"), "{}", d.message);
        assert_eq!(
            d.span.text("1 + 99999999999999999999"),
            Some("99999999999999999999")
        );
    }

    #[test]
    fn a_number_stuck_to_a_letter_is_caught_where_it_happens() {
        let src = "let x = 12abc";
        let d = error(src);
        assert!(
            d.message
                .contains("cannot be followed directly by a letter"),
            "{}",
            d.message
        );
        assert_eq!(d.span.text(src), Some("abc"));
        assert!(
            d.hint.is_some(),
            "this one has an obvious fix, so it should suggest it"
        );
    }

    #[test]
    fn strings_carry_their_resolved_value_and_keep_quotes_in_the_span() {
        let toks = lex(r#""reference""#);
        assert_eq!(toks[0].kind, TokenKind::Str("reference".to_string()));
        assert_eq!(toks[0].span.text(r#""reference""#), Some(r#""reference""#));
    }

    #[test]
    fn the_four_escapes_resolve_and_others_are_refused() {
        let toks = lex(r#""a\"b\\c\nd\te""#);
        assert_eq!(toks[0].kind, TokenKind::Str("a\"b\\c\nd\te".to_string()));

        let src = r#""oops\q""#;
        let d = error(src);
        assert!(d.message.contains("is not an escape"), "{}", d.message);
        assert_eq!(d.span.text(src), Some("\\q"));
    }

    #[test]
    fn an_empty_string_is_a_string() {
        assert_eq!(lex(r#""""#)[0].kind, TokenKind::Str(String::new()));
    }

    #[test]
    fn an_unterminated_string_points_at_the_quote_that_opened_it() {
        let src = "keep sort by \"fast";
        let d = error(src);
        assert!(d.message.contains("unterminated"), "{}", d.message);
        assert_eq!(d.span.start, src.find('"').unwrap());

        let with_break = "keep sort by \"fast\nkeep";
        let d = error(with_break);
        assert!(d.message.contains("end of the line"), "{}", d.message);
        assert_eq!(d.span.text(with_break), Some("\"fast"));
    }

    #[test]
    fn a_trailing_backslash_in_a_string_is_unterminated_not_a_panic() {
        let d = error("\"a\\");
        assert!(d.message.contains("unterminated"), "{}", d.message);
    }

    #[test]
    fn an_unexpected_character_is_an_error_with_a_span_and_not_a_panic() {
        let src = "let x = 1 # 2";
        let d = error(src);
        assert!(
            d.message.contains("unexpected character `#`"),
            "{}",
            d.message
        );
        assert_eq!(d.span.text(src), Some("#"));
    }

    #[test]
    fn a_lone_bang_suggests_the_operator_it_was_probably_meant_to_be() {
        let d = error("a ! b");
        assert_eq!(d.hint.as_deref(), Some("did you mean `!=`?"));
    }

    #[test]
    fn a_multibyte_character_reports_a_span_on_a_boundary() {
        // The whole point of stepping by UTF-8 width: a span that split this
        // character would make `Span::text` return None and the renderer
        // print nothing useful.
        for src in ["let x = π", "-- π\n§", "\"π\" §"] {
            let d = error(src);
            assert!(
                d.span.text(src).is_some(),
                "{src:?} gave a span off a boundary: {}",
                d.span
            );
        }
    }

    #[test]
    fn a_multibyte_character_inside_a_string_is_content_not_an_error() {
        assert_eq!(lex("\"πλ\"")[0].kind, TokenKind::Str("πλ".to_string()));
    }

    #[test]
    fn lexing_never_panics_on_arbitrary_input() {
        // Not a fuzzer, but the shapes that break hand-written scanners: a
        // truncated operator, a lone escape, an unclosed bracket, bare bytes.
        for src in [
            "",
            "\\",
            "\"",
            "\"\\",
            "!",
            "-",
            "--",
            "->",
            "1",
            "1.",
            ".",
            ":",
            "::",
            "(",
            "[",
            "\r",
            "\n\r",
            "\u{0}",
            "é",
            "1e9",
            "0x1f",
            "\"\n\"",
            "a--b",
            "999999999999999999999",
        ] {
            let _ = tokens(src);
        }
    }

    #[test]
    fn a_realistic_declaration_lexes_to_what_the_parser_expects() {
        let src = "oath sort : List Int -> List Int";
        assert_eq!(
            kinds(src),
            vec![
                TokenKind::Oath,
                TokenKind::Ident,
                TokenKind::Colon,
                TokenKind::Ident,
                TokenKind::Ident,
                TokenKind::Arrow,
                TokenKind::Ident,
                TokenKind::Ident,
                TokenKind::Eof,
            ]
        );
        let toks = lex(src);
        assert_eq!(toks[1].text(src), "sort");
        assert_eq!(toks[3].text(src), "List");
    }
}

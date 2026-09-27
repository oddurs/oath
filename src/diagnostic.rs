//! Positions and errors.
//!
//! Every stage reports through one [`Diagnostic`] type so that rendering lives in
//! exactly one place. A failure without a [`Span`] is a bug report the reader
//! cannot act on, so the span is not optional.

use std::fmt;
use std::ops::Range;

/// A half-open byte range into the source that produced it.
///
/// Byte offsets rather than line and column because that is what slicing wants;
/// turning one into a human position is the renderer's job.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        debug_assert!(start <= end, "a span cannot end before it starts");
        Self { start, end }
    }

    /// An empty span, for pointing at a position rather than at text.
    pub fn at(offset: usize) -> Self {
        Self {
            start: offset,
            end: offset,
        }
    }

    pub fn len(&self) -> usize {
        self.end - self.start
    }

    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }

    /// The smallest span covering both.
    pub fn to(self, other: Span) -> Span {
        Span {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }

    pub fn range(&self) -> Range<usize> {
        self.start..self.end
    }

    /// The text this span covers, or `None` if it does not land on character
    /// boundaries of `src`. Returning an `Option` keeps a wrong span from
    /// becoming a panic in the renderer.
    pub fn text<'a>(&self, src: &'a str) -> Option<&'a str> {
        src.get(self.range())
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}..{}", self.start, self.end)
    }
}

/// Something wrong with the user's program, with somewhere to point.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub span: Span,
    pub message: String,
    /// One concrete thing to try. Absent when there is nothing honest to suggest;
    /// a guessed hint is worse than none.
    pub hint: Option<String>,
}

impl Diagnostic {
    pub fn new(span: Span, message: impl Into<String>) -> Self {
        Self {
            span,
            message: message.into(),
            hint: None,
        }
    }

    /// Point at a position rather than at text, for something missing: the
    /// expression after `=`, the closing quote that never came.
    pub fn at_end(message: impl Into<String>, offset: usize) -> Self {
        Self {
            span: Span::at(offset),
            message: message.into(),
            hint: None,
        }
    }

    pub fn with_hint(mut self, hint: impl Into<String>) -> Self {
        self.hint = Some(hint.into());
        self
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)?;
        if let Some(hint) = &self.hint {
            write!(f, " ({hint})")?;
        }
        Ok(())
    }
}

/// How far a tab advances. Four, because that is what the formatter and the
/// editors configured alongside it use, so a caret lines up with what the author
/// sees.
const TAB_WIDTH: usize = 4;

/// Where an offset falls in the source.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position {
    /// 1-based.
    line: usize,
    /// 1-based, counted in characters with a tab as one, which is what an editor
    /// reports and therefore what a reader can act on.
    column: usize,
    line_start: usize,
    line_end: usize,
}

/// Step an offset back to the start of the character containing it.
///
/// A span is supposed to land on character boundaries, and the lexer's do. A
/// later stage doing arithmetic on one may not, and a diagnostic is the worst
/// possible place to panic: it is already the error path. Snapping keeps every
/// slice below infallible.
fn snap(src: &str, offset: usize) -> usize {
    let mut offset = offset.min(src.len());
    while offset > 0 && !src.is_char_boundary(offset) {
        offset -= 1;
    }
    offset
}

/// Locate `offset`, which may be one past the end: that is where an
/// end-of-file diagnostic points.
fn locate(src: &str, offset: usize) -> Position {
    let offset = snap(src, offset);
    let before = &src[..offset];
    let line = 1 + before.matches('\n').count();
    let line_start = before.rfind('\n').map_or(0, |i| i + 1);
    let line_end = src[offset..].find('\n').map_or(src.len(), |i| offset + i);
    let column = 1 + src[line_start..offset].chars().count();
    Position {
        line,
        column,
        line_start,
        line_end,
    }
}

/// The column a string ends at, in terminal cells, with tabs advancing to the
/// next stop. Used for the caret, never for the reported column.
fn visual_width(text: &str) -> usize {
    text.chars().fold(0, |w, c| {
        if c == '\t' {
            (w / TAB_WIDTH + 1) * TAB_WIDTH
        } else {
            w + 1
        }
    })
}

/// The same expansion as [`visual_width`], applied to the text itself, so the
/// displayed line and the caret under it agree about where things are.
fn expand_tabs(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for c in text.chars() {
        if c == '\t' {
            let width = visual_width(&out);
            out.extend(std::iter::repeat_n(
                ' ',
                (width / TAB_WIDTH + 1) * TAB_WIDTH - width,
            ));
        } else {
            out.push(c);
        }
    }
    out
}

impl Diagnostic {
    /// Render for a terminal, pointing at the source.
    ///
    /// ```text
    /// sort.oath:3:11: unexpected character `#`
    ///   |
    /// 3 | let x = 1 # 2
    ///   |           ^
    ///   = help: did you mean `!=`?
    /// ```
    ///
    /// `path` is a label rather than a real file, so a REPL can pass something
    /// like `<repl>`. Nothing here can fail: a span that does not land on a
    /// character boundary renders without the source line rather than panicking,
    /// because losing a message is worse than losing its underline.
    pub fn render(&self, path: &str, src: &str) -> String {
        let start = snap(src, self.span.start);
        let end = snap(src, self.span.end.max(self.span.start));
        let at = locate(src, start);
        let mut out = format!("{path}:{}:{}: {}\n", at.line, at.column, self.message);

        let number = at.line.to_string();
        let pad = " ".repeat(number.len());

        // A line whose text cannot be recovered still gets its position and
        // message; only the underline is dropped.
        if let Some(line) = src.get(at.line_start..at.line_end) {
            let line = line.strip_suffix('\r').unwrap_or(line);
            let before = &line[..(start - at.line_start).min(line.len())];

            // A span reaching past this line is underlined to the end of it: the
            // first line is where the reader looks.
            let visible_end = end.min(at.line_end).max(start);
            let covered = src.get(start..visible_end).unwrap_or_default();
            let covered = covered.strip_suffix('\r').unwrap_or(covered);

            let indent = visual_width(before);
            // An empty span points at a position, so it still needs one caret.
            let width = visual_width(covered).max(1);

            out.push_str(&format!("{pad} |\n"));
            // No trailing space on a blank line: an end-of-file diagnostic
            // after the last newline points at one, and invisible whitespace in
            // output is a nuisance to test and to read.
            let shown = expand_tabs(line);
            if shown.is_empty() {
                out.push_str(&format!("{number} |\n"));
            } else {
                out.push_str(&format!("{number} | {shown}\n"));
            }
            out.push_str(&format!(
                "{pad} | {}{}\n",
                " ".repeat(indent),
                "^".repeat(width)
            ));
        }

        if let Some(hint) = &self.hint {
            out.push_str(&format!("{pad} = help: {hint}\n"));
        }

        out
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::panic)]

    use super::*;

    #[test]
    fn a_span_slices_the_text_it_covers() {
        let src = "oath sort";
        assert_eq!(Span::new(5, 9).text(src), Some("sort"));
        assert_eq!(Span::new(0, 4).text(src), Some("oath"));
    }

    #[test]
    fn a_span_off_a_character_boundary_is_none_rather_than_a_panic() {
        let src = "π";
        assert_eq!(Span::new(0, 1).text(src), None);
        assert_eq!(Span::new(0, 2).text(src), Some("π"));
    }

    #[test]
    fn spans_join_to_cover_both() {
        assert_eq!(Span::new(2, 4).to(Span::new(8, 9)), Span::new(2, 9));
        assert_eq!(Span::new(8, 9).to(Span::new(2, 4)), Span::new(2, 9));
    }

    #[test]
    fn an_empty_span_points_at_a_position() {
        let s = Span::at(4);
        assert!(s.is_empty());
        assert_eq!(s.len(), 0);
        assert_eq!(s.text("abcdef"), Some(""));
    }

    #[test]
    fn a_hint_shows_in_the_display_form() {
        let d = Diagnostic::new(Span::at(0), "unexpected character").with_hint("try removing it");
        assert_eq!(d.to_string(), "unexpected character (try removing it)");
        assert_eq!(Diagnostic::new(Span::at(0), "bare").to_string(), "bare");
    }
}

#[cfg(test)]
mod render_tests {
    #![allow(clippy::unwrap_used, clippy::panic)]

    use super::*;
    use crate::syntax::lexer;
    use insta::assert_snapshot;

    /// Spans are computed from the source rather than written as numbers, so a
    /// test says what it means and cannot drift by one.
    fn at(src: &str, needle: &str) -> Span {
        let start = src
            .find(needle)
            .expect("the needle should be in the source");
        Span::new(start, start + needle.len())
    }

    #[test]
    fn a_caret_lands_under_the_character_it_names() {
        let src = "let x = 1 # 2";
        let d = Diagnostic::new(at(src, "#"), "unexpected character `#`");
        assert_snapshot!(d.render("sort.oath", src), @r"
        sort.oath:1:11: unexpected character `#`
          |
        1 | let x = 1 # 2
          |           ^
        ");
    }

    #[test]
    fn a_hint_renders_as_a_help_line() {
        let src = "a ! b";
        let d = Diagnostic::new(at(src, "!"), "`!` is not an operator here")
            .with_hint("did you mean `!=`?");
        assert_snapshot!(d.render("sort.oath", src), @r"
        sort.oath:1:3: `!` is not an operator here
          |
        1 | a ! b
          |   ^
          = help: did you mean `!=`?
        ");
    }

    #[test]
    fn the_line_and_column_are_one_based_and_count_from_the_line_start() {
        let src = "oath sort
  eg 1
let x = 1 # 2
";
        let d = Diagnostic::new(at(src, "#"), "unexpected character `#`");
        assert_snapshot!(d.render("sort.oath", src), @r"
        sort.oath:3:11: unexpected character `#`
          |
        3 | let x = 1 # 2
          |           ^
        ");
    }

    #[test]
    fn a_wide_span_is_underlined_across_its_whole_width() {
        let src = "1 + 99999999999999999999";
        let d = Diagnostic::new(at(src, "99999999999999999999"), "integer out of range");
        assert_snapshot!(d.render("sort.oath", src), @r"
        sort.oath:1:5: integer out of range
          |
        1 | 1 + 99999999999999999999
          |     ^^^^^^^^^^^^^^^^^^^^
        ");
    }

    #[test]
    fn tabs_expand_so_the_caret_aligns_with_what_the_author_sees() {
        let src = "keep sort
\t\t1 # 2
";
        let d = Diagnostic::new(at(src, "#"), "unexpected character `#`");
        // The reported column is 5 because an editor counts a tab as one
        // column, while the caret sits at cell 11 because a terminal expands it.
        // Both are right for their reader.
        assert_snapshot!(d.render("sort.oath", src), @r"
        sort.oath:2:5: unexpected character `#`
          |
        2 |         1 # 2
          |           ^
        ");
    }

    #[test]
    fn a_multibyte_line_keeps_the_caret_under_the_right_character() {
        let src = "let π = 1 # 2";
        let d = Diagnostic::new(at(src, "#"), "unexpected character `#`");
        assert_snapshot!(d.render("sort.oath", src), @r"
        sort.oath:1:11: unexpected character `#`
          |
        1 | let π = 1 # 2
          |           ^
        ");
    }

    #[test]
    fn an_empty_span_gets_one_caret_at_the_position() {
        let src = "let x =";
        let d = Diagnostic::at_end("expected an expression", src.len());
        assert_snapshot!(d.render("sort.oath", src), @r"
        sort.oath:1:8: expected an expression
          |
        1 | let x =
          |        ^
        ");
    }

    #[test]
    fn an_offset_past_the_last_newline_points_at_the_next_line() {
        let src = "let x =
";
        let d = Diagnostic::at_end("expected an expression", src.len());
        assert_snapshot!(d.render("sort.oath", src), @r"
        sort.oath:2:1: expected an expression
          |
        2 |
          | ^
        ");
    }

    #[test]
    fn a_span_crossing_lines_underlines_only_the_first_one() {
        let src = "\"fast\nkeep";
        let d = Diagnostic::new(Span::new(0, src.len()), "unterminated string");
        assert_snapshot!(d.render("sort.oath", src), @r#"
        sort.oath:1:1: unterminated string
          |
        1 | "fast
          | ^^^^^
        "#);
    }

    #[test]
    fn the_gutter_widens_for_a_longer_line_number() {
        let src = "x\n".repeat(9);
        let d = Diagnostic::at_end("unexpected end", src.len());
        assert_snapshot!(d.render("sort.oath", &src), @r"
        sort.oath:10:1: unexpected end
           |
        10 |
           | ^
        ");
    }

    #[test]
    fn a_real_lexer_error_renders_end_to_end() {
        let src = "oath sort : List Int\nkeep sort by \"fast\n";
        let d = lexer::tokens(src).expect_err("an unterminated string should fail");
        assert_snapshot!(d.render("sort.oath", src), @r#"
        sort.oath:2:14: unterminated string at the end of the line
          |
        2 | keep sort by "fast
          |              ^^^^^
          = help: add a closing `"`
        "#);
    }

    #[test]
    fn rendering_never_panics_whatever_span_it_is_given() {
        // A caller can hand over a span that is too long, reversed by
        // construction elsewhere, or off a character boundary. Losing the
        // underline is acceptable; losing the message to a panic is not.
        let sources = [
            "",
            "\n",
            "a",
            "π\u{3bb}",
            "x\ty\nπ\n",
            "\r\n\r\n",
            "no trailing newline",
        ];
        for src in sources {
            for start in 0..=src.len() + 2 {
                for end in start..=src.len() + 2 {
                    let out = Diagnostic::new(Span { start, end }, "m").render("f", src);
                    assert!(
                        out.starts_with("f:"),
                        "every render names the file: {out:?}"
                    );
                    assert!(out.ends_with('\n'), "every render is a complete block");
                }
            }
        }
    }
}
